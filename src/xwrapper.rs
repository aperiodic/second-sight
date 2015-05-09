extern crate xlib;

use std::ptr;
use xlib::{ Display, Window, XDefaultScreenOfDisplay, XOpenDisplay, XQueryExtension, XRootWindowOfScreen };

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
