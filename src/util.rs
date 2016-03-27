extern crate xcb;

use xcb::base;
use std::process;

pub fn init_xcb() -> (base::Connection, i32) {
    let (connection, screen_num) = base::Connection::connect();

    if connection.has_error() {
        println!("Could not connect to X server");
        process::exit(1);
    }

    (connection, screen_num)
}
