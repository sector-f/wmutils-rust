extern crate xcb;

use xcb::base;
use std::process;

pub fn init_xcb(programname: &String) -> (base::Connection, i32) {
    let (connection, screen_num) = base::Connection::connect();

    if connection.has_error() {
        println!("{}: Unable to connect to the X server", programname);
        process::exit(1);
    }

    (connection, screen_num)
}
