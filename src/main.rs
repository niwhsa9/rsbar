use x11::xlib;
use std::os::raw::{c_int, c_ulong};
use std::ffi::CString;
use std::{thread, time};

mod time_widget;

fn main() {
    let bar_text = CString::new("LOL").unwrap();

    // X11 helpers
    let dpy : *mut xlib::Display = unsafe {xlib::XOpenDisplay(std::ptr::null())};
    let screen : c_int = unsafe{xlib::XDefaultScreen(dpy)};
    let win : c_ulong = unsafe{xlib::XRootWindow(dpy, screen)};

    loop {  
        // Determine bar text from modules


        // Write to bar
        unsafe {
            xlib::XStoreName(dpy, win, v.as_ptr());
            xlib::XFlush(dpy);
        }
        thread::sleep(time::Duration::from_millis(1000))
    }

    // Free dpy
    xlib::XCloseDisplay(dpy);

}
