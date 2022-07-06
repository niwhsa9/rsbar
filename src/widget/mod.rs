mod battery;
mod discord;
mod time;

pub use battery::BatteryWidget;
pub use discord::DiscordWidget;
pub use time::TimeWidget;


pub trait Widget {
    fn get_text(&mut self) -> String;
    fn get_icon(&self) -> String;
}