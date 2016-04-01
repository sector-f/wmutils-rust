extern crate xcb;

use std::env;
use std::process;
use xcb::base;
use xcb::xproto;

pub mod util;

fn usage(programname: &String) {
    println!("Usage: {} [-a] <x> <y> <wid> [wid...]", programname);
    process::exit(1);
}

fn resize(conn: &base::Connection, win: xproto::Window, absolute: bool, mut x: i32, mut y: i32) {
    let geometry_cookie = xproto::get_geometry(&conn, win);
    let geometry_cookie_reply_result = geometry_cookie.get_reply();

    let reply = match geometry_cookie_reply_result {
        Ok(v) => v,
        Err(_) => return,
    };

    if absolute {
        x -= reply.x() as i32 + reply.width() as i32;
        y -= reply.y() as i32 + reply.height() as i32;
    }
}

fn main() {
    let programname = env::args().nth(0).unwrap_or_else(|| String::new());
    let mut args: Vec<_> = env::args().collect();
    if args.len() < 4 || args[1] == "-h" || args[1] == "--help" {
        usage(&programname);
    }
    args.remove(0);

    let mut absolute = false;
    if args[0] == "-a" {
        absolute = true;
        args.remove(0);
    }

    let connection = util::init_xcb(&programname);

    let x = i32::from_str_radix(&args[0], 10).unwrap_or(0);
    let y = i32::from_str_radix(&args[1], 10).unwrap_or(0);

    for argument in args.iter().skip(2) {
        let win = util::get_window_id(&argument);
        resize(&connection, win, absolute, x, y);
    }
}
