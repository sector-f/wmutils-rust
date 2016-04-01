extern crate xcb;

use xcb::base;
use xcb::xproto;
use std::process;
use std::mem::transmute;
use std::ops::{Deref,DerefMut};

pub fn init_xcb(programname: &String) -> base::Connection {
    match base::Connection::connect(None) {
        Ok((conn, _)) => conn,
        Err(_) => {
            println!("{}: Unable to connect to the X server", programname);
            process::exit(1);
        }
    }
}

// pub fn get_screen(conn: &base::Connection) -> xproto::Screen {
//     let setup: xproto::Setup = conn.get_setup();
//     let mut screen_iter: xproto::ScreenIterator = setup.roots();
//     let screen_option = screen_iter.next();
//     let screen: xproto::Screen = screen_option.expect("Lost connection to X server");
//     screen
// }

pub fn exists(conn: &base::Connection, window: xproto::Window) -> bool {
    let win_attrib_cookie = xproto::get_window_attributes(&conn, window);
    let win_attrib_cookie_reply_result = win_attrib_cookie.get_reply();

    match win_attrib_cookie_reply_result {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn mapped(conn: &base::Connection, window: xproto::Window) -> bool {
    let win_attrib_cookie = xproto::get_window_attributes(&conn, window);
    let win_attrib_cookie_reply_result = win_attrib_cookie.get_reply();

    let map_state = win_attrib_cookie_reply_result.expect("Failed to get window status").map_state();

    if map_state == xcb::xproto::MAP_STATE_VIEWABLE as u8 {
        true
    } else {
        false
    }
}

pub fn ignore(conn: &base::Connection, window: xproto::Window) -> bool {
    let win_attrib_cookie = xproto::get_window_attributes(&conn, window);
    let win_attrib_cookie_reply_result = win_attrib_cookie.get_reply();

    win_attrib_cookie_reply_result.expect("Failed to get window status")
        .override_redirect()
}

pub fn get_window_id(input: &String) -> xproto::Window {
    let window = if input.starts_with("0x") {
        input[2..].to_owned()
    } else {
        input.to_owned()
    };

    match u32::from_str_radix(&window, 16) {
        Ok(val) => val,
        Err(_) => 0,
    }
}

pub struct OwningRefMut<T, R> {
    owned: *mut T,
    borrow: Option<R>
}

impl <'a, T: 'a, R: 'a> OwningRefMut<T, R> {
    fn new<F: Fn(&'a mut T) -> R>(owned: Box<T>, f: F) -> OwningRefMut<T, R> {
        let owned = Box::into_raw(owned);
        let ref_mut: &mut T = unsafe { transmute(owned) };
        let borrow = f(ref_mut);
        OwningRefMut {
            owned: owned,
            borrow: Some(borrow)
        }
    }
}

impl <T, R> Drop for OwningRefMut<T, R> {
    fn drop(&mut self) {
        // Drop borrow first, then drop owned
        self.borrow.take();
        unsafe { Box::from_raw(self.owned) };
    }
}

impl <T, R> Deref for OwningRefMut<T, R> {
    type Target = R;
    fn deref(&self) -> &Self::Target {
        self.borrow.as_ref().expect("bug")
    }
}

impl <T, R> DerefMut for OwningRefMut<T, R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.borrow.as_mut().unwrap()
    }
}
