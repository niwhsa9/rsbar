use x11::xlib;
use std::os::raw::{c_int, c_ulong};
use std::ffi::CString;
use std::{thread, time};

use widgets::{TimeWidget, Widget};
pub mod widgets;

fn main() {
    // X11 helpers
    let dpy : *mut xlib::Display = unsafe {xlib::XOpenDisplay(std::ptr::null())};
    let screen : c_int = unsafe{xlib::XDefaultScreen(dpy)};
    let win : c_ulong = unsafe{xlib::XRootWindow(dpy, screen)};

    loop {  
        // Determine bar text from modules

        let mut bar_text = String::new();
        let time = TimeWidget {};
        bar_text.push_str(&time.get_text());
        let c_str : CString = CString::new(bar_text).unwrap();

        // Write to bar
        unsafe {
            xlib::XStoreName(dpy, win, c_str.as_ptr());
            xlib::XFlush(dpy);
        }
        thread::sleep(time::Duration::from_millis(1000))
    }
    println!("Done");
    // Free dpy
    unsafe {
        xlib::XCloseDisplay(dpy);
    }

}
