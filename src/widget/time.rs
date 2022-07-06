use super::base_widget::Widget;
use chrono::prelude::*;

pub struct TimeWidget {}

impl Widget for TimeWidget {
    fn get_text(&mut self) -> String {
        let now: DateTime<Local> = Local::now();
        now.format("%b %d, %I:%M %p").to_string()
    }

    fn get_icon(&self) -> String {
        String::from("")
    }
}
