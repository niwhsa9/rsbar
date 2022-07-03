use dbus::{ffidisp::Connection, Message, MessageType}
use std::sync::mpsc::channel;

pub struct DiscordWidget {
    rx : Reciever
}

impl Widget for DiscordWidget {
    fn get_text(&mut self) -> String {

    }

    fn get_icon(&self) -> String {
        //String::from("")
        // Check for any pending messages on the channel
    }
}

// Message recieve is a seperate thread?
impl DiscordWidget {
    fn notify_callback(msg : &Message) {

    }
}