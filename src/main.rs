use dbus::{ffidisp::Connection, Message};
use std::ffi::CString;
use std::os::raw::{c_int, c_ulong};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{thread, time};
use x11::xlib;

use widget::{BatteryWidget, DiscordWidget, TimeWidget, Widget};
pub mod widget;

const UPDATE_FREQ_MS: u64 = 100;

fn main() {
    // Widget seperation
    let widget_icon_sep = String::from(" ".repeat(1));
    let inter_widget_sep = String::from(" ".repeat(5));

    // Create a channel for dbus callbacks to discord widget
    let (disc_tx, disc_rx): (Sender<Message>, Receiver<Message>) = channel();

    // Create widgets
    let mut widgets: Vec<Box<dyn Widget>> = vec![
        Box::new(DiscordWidget::new(disc_rx)),
        Box::new(TimeWidget {}),
        Box::new(BatteryWidget::new()),
    ];

    // X11 helpers
    let dpy: *mut xlib::Display = unsafe { xlib::XOpenDisplay(std::ptr::null()) };
    let screen: c_int = unsafe { xlib::XDefaultScreen(dpy) };
    let win: c_ulong = unsafe { xlib::XRootWindow(dpy, screen) };

    // Launch a thread to recieve on DBus and dispatch the message to relevant widgets
    thread::spawn(move || {
        let dbus_conn = Connection::new_session().unwrap();
        dbus_conn
            .add_match("interface='org.kde.StatusNotifierItem',member='NewIconThemePath'")
            .unwrap();
        loop {
            if let Some(msg) = dbus_conn.incoming(1000).next() {
                if &(*msg.path().unwrap()) == "/org/ayatana/NotificationItem/discord1" {
                    disc_tx.send(msg).unwrap();
                }
            }
        }
    });

    loop {
        // Determine bar text from widgets
        let mut bar_text = String::new();
        for w in widgets.iter_mut() {
            bar_text.push_str(&w.get_icon());
            bar_text.push_str(&widget_icon_sep);
            bar_text.push_str(&w.get_text());
            bar_text.push_str(&inter_widget_sep);
        }
        let c_str: CString = CString::new(bar_text).unwrap();

        // Write to bar
        unsafe {
            xlib::XStoreName(dpy, win, c_str.as_ptr());
            xlib::XFlush(dpy);
        }
        thread::sleep(time::Duration::from_millis(UPDATE_FREQ_MS))
    }
}
