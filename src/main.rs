use x11::xlib;
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::os::raw::c_ulong;
use std::ffi::CString;

//mod time_widget;

fn main() {
    let n = 0 as *const c_char;
    let v = CString::new("LOL").unwrap();

    let dpy : *mut xlib::Display = unsafe {xlib::XOpenDisplay(n)};
    let screen : c_int = unsafe{xlib::XDefaultScreen(dpy)};
    let win : c_ulong = unsafe{xlib::XRootWindow(dpy, screen)};

    unsafe {
        xlib::XStoreName(dpy, win, v.as_ptr());
        xlib::XFlush(dpy);
    }
    loop {
        unsafe {
            xlib::XStoreName(dpy, win, v.as_ptr());
            xlib::XFlush(dpy);
        }
    }

}
