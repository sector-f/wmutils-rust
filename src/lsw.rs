extern crate xcb;
extern crate clap;
mod util;

use clap::{Arg, App};

#[derive(Copy,Clone)]
struct Flags {
    all   : bool,
    hidden: bool,
    ignore: bool,
}

impl Flags {
    fn none(self) -> bool {
           !self.all
        && !self.hidden
        && !self.ignore
    }
}

fn main() {
    let args = App::new("lsw")
        .about("list child windows")
        .arg(Arg::with_name("a").short("a").help("List all windows."))
        .arg(Arg::with_name("r").short("r").help("Print the ID of the root window."))
        .arg(Arg::with_name("o").short("o").help("List windows whose override_redirect attribute is set to 1."))
        .arg(Arg::with_name("u").short("u").help("List unmapped (invisible) windows."))
        .arg(Arg::with_name("wid").multiple(true))
        .get_matches();

    // Initialize xcb values
    let conn = util::init_xcb("lsw");
    let setup = conn.get_setup();
    let screen = util::get_screen(&setup);
    let root = screen.root();

    // Get all passed window ids
    let mut wids = match args.values_of("<wid>") {
        Some(ids) => ids.map(util::get_window_id).collect(),
        None => vec![screen.root()]
    };

    // Print requested info
    if args.is_present("r") {
        println!("0x{:08x}", root);
        return;
    }

    let flags = Flags {
        all   : args.is_present("a"),
        hidden: args.is_present("u"),
        ignore: args.is_present("o"),
    };

    // Print the children window IDs if applicable
    for wid in wids {
        let tree = util::get_query_tree(&conn, wid);
        for &child in tree.children() {
            if should_print(&conn, child, flags) {
                println!("0x{:08x}", child);
            }
        }
    }
}

fn should_print(conn: &xcb::Connection, window: xcb::Window, flags: Flags) -> bool {
        flags.all
    || (!util::mapped(conn, window) && flags.hidden)
    || ( util::ignore(conn, window) && flags.ignore)
    ||      util::mapped(conn, window)
        && !util::ignore(conn, window)
        && flags.none()
}
