extern crate xcb;

use xcb::base;
use std::process;

pub fn init_xcb(programname: &String) -> base::Connection {
    match base::Connection::connect(None) {
        Ok((conn, _)) => conn,
        Err(_) => {
            println!("{}: Unable to connect to the X server", programname);
            process::exit(1);
        }
    }
}
