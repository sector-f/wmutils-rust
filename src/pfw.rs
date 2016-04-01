// Implementation of wmutils' pfw (print focused window)

extern crate xcb;

use xcb::xproto;
use std::env;

pub mod util;

fn main() {
    let programname = env::args().nth(0).unwrap_or_else(|| String::new());

    let connection = util::init_xcb(&programname);

    let c = xproto::get_input_focus(&connection);
    let r = c.get_reply();

    match r {
        Ok(r) => println!("0x{:08x}", r.focus()),
        Err(e) => println!("Error: {:?}", e)
    }
}
