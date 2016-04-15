// Implementation of wmutils' pfw (print focused window)

extern crate xcb;
extern crate clap;

use clap::App;

pub mod util;

fn main() {
    App::new("pfw").about("print focused window").get_matches();

    let connection = util::init_xcb("pfw");

    let c = xcb::get_input_focus(&connection);
    let r = c.get_reply();

    match r {
        Ok(r) => println!("0x{:08x}", r.focus()),
        Err(e) => println!("Error: {:?}", e)
    }
}
