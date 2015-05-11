extern crate libc;
extern crate xlib;

use xwrapper::{ XServer, query_extension, register_compositing_window_manager };
mod xwrapper;

fn ensure_extensions(xserver: &XServer) {
  query_extension(xserver, "Xcomposite");
  query_extension(xserver, "Xdamage");
  query_extension(xserver, "Xfixes");
  query_extension(xserver, "Xrender");
  query_extension(xserver, "Xshape");
}

fn main() {
  let xserver = XServer::new();
  ensure_extensions(&xserver);

  let name = ffi::CString::new("secondsight").unwrap();
  let did_register = register_compositing_window_manager(&xserver, &name);
  if !did_register {
    panic!("Another compositing window manager is already running");
  }

  println!("Opened a display and found all the necessary X extensions!");
}
