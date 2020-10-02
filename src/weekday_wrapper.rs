use chrono::Datelike;
use std::convert::TryFrom;

pub struct WeekdayWrapper(chrono::Weekday);

impl WeekdayWrapper {
    pub fn to_json_file(&self) -> &'static str {
        use chrono::Weekday::*;
        match self.0 {
            Mon => "monday.json",
            Tue => "tuesday.json",
            Wed => "wednesday.json",
            Thu => "thursday.json",
            Fri => "friday.json",
            Sat => "saturday.json",
            Sun => "sunday.json",
        }
    }

    pub fn get_today() -> Self {
        let now = chrono::Local::now();
        WeekdayWrapper(now.weekday())
    }
}

impl TryFrom<&str> for WeekdayWrapper {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use chrono::Weekday::*;
        let value = value.trim().to_lowercase();
        match &value[..] {
            "monday" | "понедельник" => Ok(WeekdayWrapper(Mon)),
            "tuesday" | "вторник" => Ok(WeekdayWrapper(Tue)),
            "wednesday" | "среда" => Ok(WeekdayWrapper(Wed)),
            "thursday" | "четверг" => Ok(WeekdayWrapper(Thu)),
            "friday" | "пятница" => Ok(WeekdayWrapper(Fri)),
            "saturday" | "суббота" => Ok(WeekdayWrapper(Sat)),
            "sunday" | "воскресенье" => Ok(WeekdayWrapper(Sun)),
            _ => Err("Unknown day"),
        }
    }
}
