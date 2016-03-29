extern crate xcb;

use std::env;
use std::process;
use xcb::xproto;

mod util;

fn usage(programname: &String) {
    println!("Usage: {} <wid>", programname);
    process::exit(1);
}

fn main() {
    let programname = env::args().nth(0).unwrap_or_else(|| String::new());
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 || args[1] == "-h" || args[1] == "--help" {
        usage(&programname);
    }

    let connection = util::init_xcb(&programname);

    let win = util::get_window_id(&args[1]);

    xproto::set_input_focus(&connection, xproto::INPUT_FOCUS_POINTER_ROOT as u8, win, xproto::TIME_CURRENT_TIME);
    connection.flush();
}
