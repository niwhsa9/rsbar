use dbus::{ffidisp::Connection, Message, MessageType}
pub struct DiscordWidget {
    
}

impl Widget for DiscordWidget {
    fn get_text(&mut self) -> String {
    }

    fn get_icon(&self) -> String {
        //String::from("")
    }
}

// Message recieve is a seperate thread?
impl DiscordWidget {
    fn notify_callback(msg : &Message) {

    }
}