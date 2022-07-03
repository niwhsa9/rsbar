use dbus::{ffidisp::Connection, Message, MessageType};
use std::sync::mpsc::{Receiver, channel};
use super::base_widget::Widget;

pub struct DiscordWidget {
    pub rx : Receiver<Message>,
    notif_pending : bool
}

impl Widget for DiscordWidget {
    fn get_text(&mut self) -> String {
        String::from("")
    }

    fn get_icon(&self) -> String {
        //String::from("")
        // Check for any pending messages on the channel

        String::from("")
    }
}

impl DiscordWidget {
    pub fn new(rx : Receiver<Message>) -> DiscordWidget {
       DiscordWidget { 
           rx : rx,
           notif_pending : false
        }
    }
}