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

pub fn mapped(conn: &base::Connection, window: xproto::Window) -> bool {
    let win_attrib_cookie = xproto::get_window_attributes(&conn, window);
    let win_attrib_cookie_reply_result = win_attrib_cookie.get_reply();

    let map_state = win_attrib_cookie_reply_result.expect("Failed to get window status").map_state();

    if map_state == xcb::xproto::MAP_STATE_VIEWABLE as u8 {
        true
    } else {
        false
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
