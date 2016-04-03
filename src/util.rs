extern crate xcb;

use xcb::base;
use xcb::xproto;
use std::process;

pub fn init_xcb(programname: &str) -> base::Connection {
    match base::Connection::connect(None) {
        Ok((conn, _)) => conn,
        Err(_) => {
            println!("{}: Unable to connect to the X server", programname);
            process::exit(1);
        }
    }
}

pub fn get_screen<'a>(setup: &'a xproto::Setup) -> xproto::Screen<'a> {
    setup.roots().next().expect("Lost connection to X server")
}

pub fn exists(conn: &base::Connection, window: xproto::Window) -> bool {
    let win_attrib_cookie = xproto::get_window_attributes(&conn, window);
    let win_attrib_cookie_reply_result = win_attrib_cookie.get_reply();

    match win_attrib_cookie_reply_result {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn mapped(conn: &xcb::Connection, window: xcb::Window) -> bool {
    let attrs = xcb::get_window_attributes(&conn, window).get_reply();
    match attrs {
        Ok(attrs) => attrs.map_state() as u32 == xcb::MAP_STATE_VIEWABLE,
        _ => false
    }
}

pub fn ignore(conn: &xcb::Connection, window: xcb::Window) -> bool {
    let attrs = xcb::get_window_attributes(&conn, window).get_reply();
    match attrs {
        Ok(attrs) => attrs.override_redirect(),
        _ => false
    }
}

pub fn get_window_id(input: &str) -> xproto::Window {
    let window = if input.starts_with("0x") {
        &input[2..]
    } else {
        input
    };

    match u32::from_str_radix(window, 16) {
        Ok(val) => val,
        Err(_) => 0,
    }
}
