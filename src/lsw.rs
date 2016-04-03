extern crate xcb;
extern crate docopt;
mod util;

use docopt::Docopt;
use std::env::args;

const USAGE: &'static str = "
lsw - list child windows

lsw lists child windows of the given window.
If no windows are given, lsw lists the children of the root window.

Usage:
  lsw [-arouh] [<wid>...]

Options:
  -a          List all windows.
  -r          Print the ID of the root window.
  -o          List windows whose override_redirect attribute is set to 1.
  -u          List unmapped (invisible) windows.
  -h, --help  Print this help.
";

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
    let args = Docopt::new(USAGE)
        .and_then(|d| d
            .argv(args())
            .parse()
        )
        .unwrap_or_else(|e| e.exit());

    // println!("{:?}", args);

    // Initialize xcb values
    let conn = util::init_xcb("lsw");
    let setup = conn.get_setup();
    let screen = util::get_screen(&setup);
    let root = screen.root();

    // Get all passed window ids
    let mut wids: Vec<_> = args.get_vec("<wid>").into_iter().map(util::get_window_id).collect();
    if wids.is_empty() {
        wids.push(screen.root())
    }

    // Print requested info
    if args.get_bool("-r") {
        println!("0x{:08x}", root);
        return;
    }

    let flags = Flags {
        all   : args.get_bool("-a"),
        hidden: args.get_bool("-u"),
        ignore: args.get_bool("-o"),
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
