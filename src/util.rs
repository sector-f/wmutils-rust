extern crate xcb;

use xcb::base;
use xcb::xproto;
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

pub fn get_window_id(input: &String) -> xproto::Window {
    let window = if input.starts_with("0x") {
        input[2..].to_owned()
    } else {
        input.to_owned()
    };

    match u32::from_str_radix(&window, 16) {
        Ok(val) => val,
        Err(_) => 0,
    }
}
