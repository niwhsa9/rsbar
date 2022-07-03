use dbus::{ffidisp::Connection, Message, MessageType};
use std::sync::mpsc::{Receiver, channel};
use super::base_widget::Widget;

pub struct DiscordWidget {
    pub rx : Receiver<Message>,
    notif_pending : bool
}

impl Widget for DiscordWidget {
    fn get_text(&mut self) -> String {
        // Check for any pending messages on the channel
        match self.rx.try_recv() {
            Ok(_) => {
                self.notif_pending = !self.notif_pending;
            } 
            Err(_) => {}
        }
        String::from("")
    }

    fn get_icon(&self) -> String {
        if self.notif_pending {
            String::from("")
        } else {
            String::from("")
        }
    }
}

impl DiscordWidget {
    pub fn new(rx : Receiver<Message>) -> DiscordWidget {
       DiscordWidget { 
           rx : rx,
           notif_pending : true
        }
    }
}