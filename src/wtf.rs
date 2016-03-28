extern crate xcb;

use std::env;
use std::process;
use xcb::xproto;
use xcb::ffi;

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

    let (connection, screen_num) = util::init_xcb(&programname);

    // Basically recreating strtoul(3)
    let input = if args[1].starts_with("0x") {
        args[1][2..].to_owned()
    } else {
        args[1].to_owned()
    };
    let win = match u32::from_str_radix(&input, 16) {
        Ok(val) => val,
        Err(_) => 0,
    };

    // println!("{}", win);

    xproto::set_input_focus(&connection, xproto::INPUT_FOCUS_POINTER_ROOT as u8, win, xproto::TIME_CURRENT_TIME);
    connection.flush();
}
