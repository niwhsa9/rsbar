
use super::base_widget::Widget;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;


pub struct BatteryWidget {
    percent : i32
}
impl BatteryWidget {
    pub fn new() -> BatteryWidget {
        BatteryWidget {percent : 0}
    }
}

impl Widget for BatteryWidget {
    fn get_text(&mut self) -> String {
        let bat_percent_path = Path::new("/sys/class/power_supply/BAT0/capacity");
        let mut bat_percent_file = match File::open(&bat_percent_path) {
            Err(_) => panic!("Could not read battery"),
            Ok(file) => file
        };
        let mut percent = String::new();
        match bat_percent_file.read_to_string(&mut percent) {
            Err(err) => panic!("Could not read battery"),
            Ok(_) => {}
        };
        percent.retain(|c| !c.is_whitespace());
        println!("{}", percent);
        self.percent = percent.parse().unwrap();
        percent.push_str("%");
        return percent

    }

    fn get_icon(&self) -> String {
        let bat_status_path = Path::new("/sys/class/power_supply/BAT0/status");
        let mut bat_status_file = match File::open(&bat_status_path) {
            Err(_) => panic!("Could not read bat status"),
            Ok(file) => file
        };
        let mut status = String::new();
        match bat_status_file.read_to_string(&mut status) {
            Err(err) => panic!("Could not read bat status"),
            Ok(_) => {}
        };
        status.retain(|c| !c.is_whitespace());

        let icon = match status.as_str() {
            "Discharging" => {
                match self.percent {
                    90.. => "",
                    70.. => "",
                    50.. => "",
                    _ => ""
                }
            }, 
            _ => ""

        }.to_string();
        return icon;
    }
}