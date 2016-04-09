extern crate xcb;
extern crate clap;

use clap::{App, Arg};

pub mod util;

fn main() {
    let args = App::new("killw")
        .about("kill windows")
        .arg(Arg::with_name("parent")
            .short("p")
            .help("Kill the parent application of the window instead of the window itself"))
        .arg(Arg::with_name("wid")
            .multiple(true)
            .required(true))
        .get_matches();

    let connection = util::init_xcb("killw");
    let wids = args.values_of("wid").unwrap(); // Unwrap is fine, the arg is required

    for wid in wids {
        let wid = util::get_window_id(&wid);

        if args.is_present("parent") {
            xcb::kill_client(&connection, wid);
        } else {
            xcb::destroy_window(&connection, wid);
        }
    }

    connection.flush();
}
