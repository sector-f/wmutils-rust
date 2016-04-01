extern crate xcb;

use std::env;
use std::process;
use xcb::base;
use xcb::xproto;

pub mod util;

enum Attribute {
    W,
    H,
    X,
    Y,
    B,
}

fn usage(programname: &String) {
    println!("Usage: {} [-h] [bmiowhxy] <wid>", programname);
    process::exit(1);
}

fn get_attribute(conn: &base::Connection, win: xproto::Window, attr: Attribute) -> i32 {
    if ! util::exists(&conn, win) {
        println!("0x{:08x}: No such window", win);
        process::exit(1);
    }

    let geometry_cookie = xproto::get_geometry(&conn, win);
    let geometry_cookie_reply = geometry_cookie.get_reply().unwrap();

    match attr {
        Attribute::X => geometry_cookie_reply.x() as i32,
        Attribute::Y => geometry_cookie_reply.y() as i32,
        Attribute::W => geometry_cookie_reply.width() as i32,
        Attribute::H => geometry_cookie_reply.height() as i32,
        Attribute::B => geometry_cookie_reply.border_width() as i32,
    }
}

fn main() {
    let programname = env::args().nth(0).unwrap_or_else(|| String::new());
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 || args[1] == "-h" || args[1] == "--help" {
        usage(&programname);
    }

    let connection = util::init_xcb(&programname);

    if args.len() == 2 {
        let win = util::get_window_id(&args[1]);
        match util::exists(&connection, win) {
            true => process::exit(0),
            false => process::exit(1),
        }
    }

    let win = util::get_window_id(&args[2]);

    let mut iter = args[1].chars().peekable();
    while let Some(c) = iter.next() {
        match c {
            'i' => print!("0x{:08x}", win),
            'b' => print!("{}", get_attribute(&connection, win, Attribute::B)),
            'h' => print!("{}", get_attribute(&connection, win, Attribute::H)),
            'x' => print!("{}", get_attribute(&connection, win, Attribute::X)),
            'y' => print!("{}", get_attribute(&connection, win, Attribute::Y)),
            'w' => print!("{}", get_attribute(&connection, win, Attribute::W)),
            'o' => if util::ignore(&connection, win) {
                        std::process::exit(0);
                    } else {
                        std::process::exit(1);
                    },
            'm' => if util::mapped(&connection, win) {
                        std::process::exit(0);
                    } else {
                        std::process::exit(1);
                    },
            _ => usage(&programname),
        }

        if iter.peek() == None {
            println!("");
        } else {
            print!(" ");
        }
    }

    connection.flush();
}
