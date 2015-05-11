extern crate xlib;

use libc::*;
use std::ptr;
use std::ffi;
use xlib::{ Display, Pixmap, Window, XClassHint, XDefaultScreenOfDisplay, XID, XOpenDisplay, XRootWindowOfScreen, XSizeHints };

//
// XWMHints Type and Linking Against Xutf8SetWMProperties
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

#[link(name = "X11")]
extern {
  fn Xutf8SetWMProperties(display: *mut Display, window: Window, window_name: *mut c_char,
                          icon_name: *mut c_char, argv: *mut *mut c_char, argc: i32,
                          normal_hints: *mut XSizeHints, wm_hints: *mut XWMHints,
                          class_hints: *mut XClassHint);
}

//
// X Extension Linking
//

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

#[link(name = "Xrender")]
extern {
  fn XRenderQueryExtension(display: *mut Display, event: *mut i32, error: *mut i32) -> i32;
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
      "Xrender" => if XRenderQueryExtension(xserver.display, event_base, error_base) == 0 {
                     panic!("No XRender extension!");
                   },
      "Xshape" => if XShapeQueryExtension(xserver.display, event_base, error_base) == 0 {
                    panic!("No XShape extension!");
                  },
      _ => panic!(format!("Don't know how to query for {} extension", name)),
    }
  }
}
