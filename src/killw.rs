extern crate xcb;

use std::env;
use std::process;
use xcb::xproto;

pub mod util;

fn usage(programname: &String) {
    println!("Usage: {} <wid>", programname);
    process::exit(1);
}

fn main() {
    let programname = env::args().nth(0).unwrap_or_else(|| String::new());
    let mut args: Vec<_> = env::args().collect();
    if args.len() < 2 || args[1] == "-h" || args[1] == "--help" {
        usage(&programname);
    }
    args.remove(0);

    let mut parent: bool = false;
    if args[0] == "-p" {
        parent = true;
        args.remove(0);
    }

    let connection = util::init_xcb(&programname);

    for window in args {
        let win = util::get_window_id(&window);

        if parent {
            xproto::kill_client(&connection, win);
        } else {
            xproto::destroy_window(&connection, win);
        }
    }

    connection.flush();
}
