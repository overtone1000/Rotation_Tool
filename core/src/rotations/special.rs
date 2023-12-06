pub mod weekdays {
    pub fn weekday_to_str(wd: chrono::Weekday) -> String {
        format!("{}", wd)
    }
}

pub const all: &str = "All";
