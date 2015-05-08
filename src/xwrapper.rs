use std::ptr;
use xlib::{ Display, Window, XOpenDisplay, XDefaultScreenOfDisplay, XRootWindowOfScreen };

pub struct XServer {
  display: *mut Display;
  root:         Window;
}

impl XServer {
  pub fn new() -> XServer {
    unsafe {
      let display = XOpenDisplay(ptr::null_mut());
      let screen = XDefaultScreenOfDisplay(display);
      let root = XRootWindowOfScreen(screen);

      XServer {
        display: display;
        root: root;
      }
    }
  }
}
