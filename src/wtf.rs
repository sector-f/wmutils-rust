extern crate xcb;
extern crate clap;

use clap::{App, Arg};

pub mod util;

fn main() {
    let args = App::new("wtf")
        .about("transfer window focus")
        .arg(Arg::with_name("wid").required(true))
        .get_matches();

    let wid = args.value_of("wid").unwrap(); // Unwrap is fine, the arg is required
    let wid = util::get_window_id(wid);

    let connection = util::init_xcb("wtf");

    xcb::set_input_focus(&connection, xcb::INPUT_FOCUS_POINTER_ROOT as u8, wid, xcb::TIME_CURRENT_TIME);

    connection.flush();
}
