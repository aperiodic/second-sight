extern crate xlib;

use std::ptr;
use xlib::{ Display, Window, XDefaultScreenOfDisplay, XOpenDisplay, XQueryExtension, XRootWindowOfScreen };

#[link(name = "Xrender")]
extern {
  fn XRenderQueryExtension(display: *mut Display, event: *mut i32, error: *mut i32) -> i32;
}

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

      let mut render_event = &mut 0;
      let mut render_error = &mut 0;
      if XRenderQueryExtension(display, render_event, render_error) == 0 {
        panic!("No XRender extension!");
      }

      XServer {
        display: display,
        root: root
      }
    }
  }
}
