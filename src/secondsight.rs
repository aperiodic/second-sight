extern crate libc;
extern crate xlib;
use xlib::{ XEvent };

use xwrapper::{ XServer, query_extension };
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

  println!("Opened a display and found all the necessary X extensions!");
}
