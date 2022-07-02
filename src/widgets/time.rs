use super::base_widget::Widget;
use chrono::prelude::*;

pub struct TimeWidget {
    
}

impl Widget for TimeWidget {
    fn get_text(&self) -> String {
        let now : DateTime<Local> = Local::now();
        now.format("%b %d, %l:%M %P").to_string()
    }

    fn get_icon(&self) -> String {
        String::from("")
    }
}