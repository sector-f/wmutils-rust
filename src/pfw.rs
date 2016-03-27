// Implementation of wmutils' pfw (print focused window)

extern crate xcb;

use xcb::xproto;

mod util;

fn main() {
    let (connection, screen_num) = util::init_xcb();

    let c = xproto::get_input_focus(&connection);
    let r = c.get_reply();

    match r {
        Ok(r) => println!("0x{:08x}", r.focus()),
        Err(e) => println!("Error: {:?}", e)
    }
}
