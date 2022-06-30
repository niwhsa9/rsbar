use x11::xlib;
use std::os::raw::c_char;
use std::ffi::CString;


fn main() {
    let n = 0 as *const c_char;
    let v = CString::new("Hello, world! ").unwrap();
    unsafe {
        let dpy = xlib::XOpenDisplay(n);
        let screen = xlib::XDefaultScreen(dpy);
        let root = xlib::XRootWindow(dpy, screen);
        xlib::XStoreName(dpy, root, v.as_ptr());
        xlib::XFlush(dpy);
    }
    println!("Hello");

}
