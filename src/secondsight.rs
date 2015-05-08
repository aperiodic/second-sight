extern crate libc;
extern crate xlib;
use xlib::{ XEvent };

use xwrapper::XServer;
mod xwrapper;

fn main() {
  let xserv = XServer::new();

  println!("Opened a display and found Xrender extension!");
}
