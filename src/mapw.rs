extern crate xcb;
extern crate clap;

use clap::{App, Arg, ArgGroup};

pub mod util;

fn main() {
    let args = App::new("mapw")
        .about("map or unmap windows")
        .arg(Arg::with_name("map").short("m").help("Map (show) wid."))
        .arg(Arg::with_name("toggle").short("t").help("Toggle wid's visibility."))
        .arg(Arg::with_name("unmap").short("u").help("Unmap (hide) wid."))
        .group(ArgGroup::with_name("options").required(true).args(&[
            "map", "toggle", "unmap"
        ]))
        .arg(Arg::with_name("wid")
            .multiple(true)
            .required(true))
        .get_matches();

    let connection = util::init_xcb("mapw");
    let wids = args.values_of("wid").unwrap(); // Unwrap is fine, the arg is required

    let action: fn(&xcb::Connection, xcb::Window) =
        if args.is_present("map") { map }
        else if args.is_present("unmap") { unmap }
        else { toggle };

    for wid in wids {
        let wid = util::get_window_id(wid);
        action(&connection, wid);
    }

    connection.flush();
}

fn map(connection: &xcb::Connection, window: xcb::Window) {
    xcb::map_window(connection, window);
}

fn unmap(connection: &xcb::Connection, window: xcb::Window) {
    xcb::unmap_window(connection, window);
}

fn toggle(connection: &xcb::Connection, window: xcb::Window) {
    if util::mapped(connection, window) {
        xcb::unmap_window(connection, window);
    }
    else {
        xcb::map_window(connection, window);
    }
}
