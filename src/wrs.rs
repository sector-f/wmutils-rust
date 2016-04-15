extern crate xcb;
#[macro_use]
extern crate clap;

use clap::{App, Arg};

pub mod util;

fn resize(conn: &xcb::Connection, win: xcb::Window, absolute: bool, mut x: i16, mut y: i16) {
    let setup = conn.get_setup();
    let screen = util::get_screen(&setup);

    let geometry_cookie = xcb::get_geometry(&conn, win);
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

    // xcb::configure_window(conn,
    //                          win,
    //         &[(xcb::CONFIG_WINDOW_WIDTH as u16, 500 as u32),
    //         (xcb::CONFIG_WINDOW_HEIGHT as u16, 300 as u32),
    //         (xcb::STACK_MODE_ABOVE as u16, xcb::CONFIG_WINDOW_STACK_MODE as u32)]);

    xcb::configure_window(conn,
        win,
        &[(xcb::CONFIG_WINDOW_WIDTH as u16, (window_width + x) as u32),
        (xcb::CONFIG_WINDOW_HEIGHT as u16, (window_height + y) as u32),
        (xcb::STACK_MODE_ABOVE as u16, xcb::CONFIG_WINDOW_STACK_MODE as u32)]);
}

fn main() {
    let args = App::new("wrs")
        .about("resize windows")
        .arg(Arg::with_name("absolute").short("a"))
        .arg(Arg::with_name("x").required(true))
        .arg(Arg::with_name("y").required(true))
        .arg(Arg::with_name("wid")
            .required(true)
            .multiple(true))
        .get_matches();


    let absolute = args.is_present("absolute");
    let x = value_t!(args.value_of("x"), i16).unwrap_or_else(invalid_number);
    let y = value_t!(args.value_of("y"), i16).unwrap_or_else(invalid_number);
    let wids = args.values_of("wid").unwrap(); // Unwrap is fine, the arg is required

    let connection = util::init_xcb("wrs");

    for wid in wids {
        let wid = util::get_window_id(wid);
        resize(&connection, wid, absolute, x, y);
    }
    connection.flush();
}

fn invalid_number<T, E>(_: E) -> T {
    use std::io::Write;
    write!(::std::io::stderr(), "invalid number format\n").unwrap();
    ::std::process::exit(1);
}
