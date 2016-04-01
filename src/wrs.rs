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

fn resize(conn: &base::Connection, win: xproto::Window, absolute: bool, mut x: i16, mut y: i16) {
    let setup = conn.get_setup();
    let screen = util::get_screen(&setup);

    let geometry_cookie = xproto::get_geometry(&conn, win);
    let geometry_cookie_reply_result = geometry_cookie.get_reply();

    let reply = match geometry_cookie_reply_result {
        Ok(v) => v,
        Err(_) => return,
    };

    let window_width = reply.width() as i16;
    let window_height = reply.height() as i16;
    let window_x = reply.x();
    let window_y = reply.y();
    let window_border = reply.border_width() as i16;
    let screen_width = screen.width_in_pixels() as i16;
    let screen_height = screen.height_in_pixels() as i16;

    if absolute {
        x -= window_x + window_width;
        y -= window_y + window_height;
    }

    if window_x + window_width + 2 * window_border + x > screen_width {
        x = screen_width - (window_x + window_width + 2 * window_border)
    }

    if window_y + window_height + 2 * window_border + y > screen_height {
        y = screen_height - (window_y + window_height + 2 * window_border)
    }

    // This shows that the values are being interpreted correctly
    // println!("Old width: {}", window_width);
    // println!("Old height: {}", window_height);
    // println!("New width: {}", window_width + x);
    // println!("New height: {}", window_height + y);

    // xproto::configure_window(conn,
    //                          win,
    //         &[(xproto::CONFIG_WINDOW_WIDTH as u16, 500 as u32),
    //         (xproto::CONFIG_WINDOW_HEIGHT as u16, 300 as u32),
    //         (xproto::STACK_MODE_ABOVE as u16, xproto::CONFIG_WINDOW_STACK_MODE as u32)]);

    xproto::configure_window(conn,
        win,
        &[(xproto::CONFIG_WINDOW_WIDTH as u16, (window_width + x) as u32),
        (xproto::CONFIG_WINDOW_HEIGHT as u16, (window_height + y) as u32),
        (xproto::STACK_MODE_ABOVE as u16, xproto::CONFIG_WINDOW_STACK_MODE as u32)]);
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

    let x = i16::from_str_radix(&args[0], 10).unwrap_or(0);
    let y = i16::from_str_radix(&args[1], 10).unwrap_or(0);

    for argument in args.iter().skip(2) {
        let win = util::get_window_id(&argument);
        resize(&connection, win, absolute, x, y);
    }
    connection.flush();
}
