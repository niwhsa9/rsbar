use super::Widget;

pub struct BatteryWidget {
    percent: i32,
}
impl BatteryWidget {
    pub fn new() -> BatteryWidget {
        BatteryWidget { percent: 0 }
    }
}

impl Widget for BatteryWidget {
    fn get_text(&mut self) -> String {
        let bat_percent_path = "/sys/class/power_supply/BAT0/capacity";
        let mut percent = std::fs::read_to_string(bat_percent_path).expect("Could not read battery");
        percent.retain(|c| !c.is_whitespace());
        self.percent = percent.parse().unwrap();
        percent.push_str("%");
        return percent;
    }

    fn get_icon(&self) -> String {
        let bat_status_path = "/sys/class/power_supply/BAT0/status";
        let mut status = std::fs::read_to_string(bat_status_path).expect("Could not read battery");
        status.retain(|c| !c.is_whitespace());

        let icon = match status.as_str() {
            "Discharging" => match self.percent {
                90.. => "",
                70.. => "",
                50.. => "",
                _ => "",
            },
            _ => "",
        }
        .to_string();
        return icon;
    }
}
