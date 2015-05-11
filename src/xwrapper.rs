use libc::*;

use std::ffi::{CString, CStr};
use std::ptr;

use xlib::{ Display, Pixmap, Window, XClassHint, XCreateSimpleWindow, XDefaultScreen, XDefaultScreenOfDisplay, XGetSelectionOwner, XID, XInternAtom, XOpenDisplay, XRootWindowOfScreen, XSetSelectionOwner, XSizeHints };
use x11::xrender::{ XRenderQueryExtension };
use x11::xlib::{ _XDisplay };

static XNone: u64 = 0;

//
// Xlib Types
//

#[repr(C)]
pub struct struct__XWMHints {
  pub flags: c_long,
  pub input: c_int,
  pub initial_state: c_int,
  pub icon_pixmap: Pixmap,
  pub icon_window: Window,
  pub icon_x: c_int,
  pub icon_y: c_int,
  pub icon_mask: Pixmap,
  pub window_group: XID,
}
pub type XWMHints = struct__XWMHints;

//
// Xlib & X Extensions Linking
//


#[link(name = "X11")]
extern {
  fn Xutf8SetWMProperties(display: *mut Display, window: Window, window_name: *mut c_char,
                          icon_name: *mut c_char, argv: *mut *mut c_char, argc: i32,
                          normal_hints: *mut XSizeHints, wm_hints: *mut XWMHints,
                          class_hints: *mut XClassHint);
}

#[link(name = "Xcomposite")]
extern {
  fn XCompositeQueryExtension(display: *mut Display, event: *mut i32, error: *mut i32) -> i32;
}

#[link(name = "Xdamage")]
extern {
  fn XDamageQueryExtension(display: *mut Display, event: *mut i32, error: *mut i32) -> i32;
}

#[link(name = "Xext")]
extern {
  fn XShapeQueryExtension(display: *mut Display, event: *mut i32, error: *mut i32) -> i32;
}

#[link(name = "Xfixes")]
extern {
  fn XFixesQueryExtension(display: *mut Display, event: *mut i32, error: *mut i32) -> i32;
}

//
// XServer Struct
//

pub struct XServer {
  display: *mut Display,
  root:         Window
}

impl XServer {
  pub fn new() -> XServer {
    unsafe {
      let display = XOpenDisplay(ptr::null_mut());
      if display.is_null() {
        panic!("Could not open display!");
      }

      let screen = XDefaultScreenOfDisplay(display);
      let root = XRootWindowOfScreen(screen);

      XServer {
        display: display,
        root: root
      }
    }
  }
}

pub fn query_extension(xserver: &XServer, name: &str) {
  let mut event_base = &mut 0;
  let mut error_base = &mut 0;

  unsafe {
    match name {
      "Xcomposite" => if XCompositeQueryExtension(xserver.display, event_base, error_base) == 0 {
                        panic!("No XComposite extension!");
                      },
      "Xdamage" => if XDamageQueryExtension(xserver.display, event_base, error_base) == 0 {
                     panic!("No XDamage extension!");
                   },
      "Xfixes" => if XFixesQueryExtension(xserver.display, event_base, error_base) == 0 {
                    panic!("No XFixes extension!");
                  },
      "Xrender" => { let have_xrender = XRenderQueryExtension(xserver.display as *mut _XDisplay,
                                                              event_base, error_base);
                     if  have_xrender == 0 {
                       panic!("No XRender extension!");
                     }
                   },
      "Xshape" => if XShapeQueryExtension(xserver.display, event_base, error_base) == 0 {
                    panic!("No XShape extension!");
                  },
      _ => panic!(format!("Don't know how to query for {} extension", name)),
    }
  }
}

pub struct WindowSettings {
  pub opacity: XID,
  pub type_atom: XID,
  pub is_desktop: XID,
  pub is_dock: XID,
  pub is_toolbar: XID,
  pub is_menu: XID,
  pub is_util: XID,
  pub is_splash: XID,
  pub is_dialog: XID,
  pub is_dropdown: XID,
  pub is_popup: XID,
  pub is_tooltip: XID,
  pub is_notification: XID,
  pub is_combo: XID,
  pub is_dnd: XID,
  pub is_normal: XID,
}

pub fn find_window_settings(xserver: &XServer) -> WindowSettings {
  WindowSettings {
    opacity: intern_atom(xserver, "_NET_WM_WINDOW_OPACITY"),
    type_atom: intern_atom(xserver, "_NET_WM_WINDOW_TYPE"),

    is_desktop: intern_atom(xserver, "_NET_WM_WINDOW_TYPE_DESKTOP"),
    is_dock: intern_atom(xserver, "_NET_WM_WINDOW_TYPE_DOCK"),
    is_toolbar: intern_atom(xserver, "_NET_WM_WINDOW_TYPE_TOOLBAR"),
    is_menu: intern_atom(xserver, "_NET_WM_WINDOW_TYPE_MENU"),
    is_util: intern_atom(xserver, "_NET_WM_WINDOW_TYPE_UTILITY"),
    is_splash: intern_atom(xserver, "_NET_WM_WINDOW_TYPE_SPLASH"),
    is_dialog: intern_atom(xserver, "_NET_WM_WINDOW_TYPE_DIALOG"),
    is_dropdown: intern_atom(xserver, "_NET_WM_WINDOW_TYPE_DROPDOWN"),
    is_popup: intern_atom(xserver, "_NET_WM_WINDOW_TYPE_POPUP"),
    is_tooltip: intern_atom(xserver, "_NET_WM_WINDOW_TYPE_TOOLTIP"),
    is_notification: intern_atom(xserver, "_NET_WM_WINDOW_TYPE_NOTIFICATION"),
    is_combo: intern_atom(xserver, "_NET_WM_WINDOW_TYPE_COMBO"),
    is_dnd: intern_atom(xserver, "_NET_WM_WINDOW_TYPE_DND"),
    is_normal: intern_atom(xserver, "_NET_WM_WINDOW_TYPE_NORMAL"),
  }
}

pub fn intern_atom(xserver: &XServer, name: &str) -> XID {
  return unsafe { XInternAtom(xserver.display, name.as_ptr() as *mut c_char, 0) };
}

pub fn register_compositing_window_manager(xserver: &XServer, wm_name: &CStr) -> bool {
  let screen = unsafe { XDefaultScreen(xserver.display) };
  let reg_atom_name = CString::new(format!("_NET_WM_CM_S{}", screen)).unwrap();
  let reg_atom = unsafe { XInternAtom(xserver.display, reg_atom_name.as_ptr() as *mut c_char, 0) };
  let extant_window = unsafe { XGetSelectionOwner(xserver.display, reg_atom) };

  if extant_window != XNone {
    return false;
  }

  unsafe {
    let our_window = XCreateSimpleWindow(xserver.display, xserver.root,
                                         0, 0, 1, 1, 0, XNone, XNone);
    Xutf8SetWMProperties(xserver.display, our_window,
                         wm_name.as_ptr() as *mut c_char, wm_name.as_ptr() as *mut c_char,
                         ptr::null_mut(), 0, ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
    XSetSelectionOwner(xserver.display, reg_atom, our_window, 0);
  };

  return true;
}
