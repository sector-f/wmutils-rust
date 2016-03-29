extern crate xcb;

use std::env;
use std::process;
use xcb::xproto;

mod util;

enum Action {
    Map,
    Unmap,
    Toggle,
    Usage,
}

fn usage(programname: &String) {
    println!("Usage: {} [-h] [-mut <wid> [wid..]]", programname);
    process::exit(1);
}

fn main() {
    let programname = env::args().nth(0).unwrap_or_else(|| String::new());
    let args: Vec<_> = env::args().collect();

    if args.len() <= 2 {
        usage(&programname);
    }

    let action =
        if args[1] == "-m" {
            Action::Map
        } else if args[1] == "-u" {
            Action::Unmap
        } else if args[1] == "-t" {
            Action::Toggle
        } else {
            Action::Usage
        };

    if let Action::Usage = action {
        usage(&programname);
    }

    let connection = util::init_xcb(&programname);
    let window = util::get_window_id(&args[2]);

    if let Action::Map = action {
        xproto::map_window(&connection, window);
    } else if let Action::Unmap = action {
        xproto::unmap_window(&connection, window);
    } else if let Action::Toggle = action {
        if util::mapped(&connection, window) {
            xproto::unmap_window(&connection, window);
        } else {
        xproto::map_window(&connection, window);
        }
    }
    connection.flush();
}
