use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    name: String,
    #[serde(rename = "type")]
    lesson_type: String,
    link: String,
    password: Option<String>,
    start_m: i32,
    end_m: i32,
}

impl Lesson {
    pub fn print(&self) -> String {
        format!(
            "{:02}:{:02} \\- {:02}:{:02}\t[{}]({}) \\({}\\)",
            self.start_m / 60,
            self.start_m % 60,
            self.end_m / 60,
            self.end_m % 60,
            self.name.replace("+", "\\+"),
            self.link,
            self.lesson_type
        )
    }
}

pub(crate) async fn get_day_timetable(day: &str) -> Result<Vec<Lesson>, reqwest::Error> {
    let lessons = reqwest::get(&format!("http://localhost:8000/timetable/{}", day))
        .await?
        .json::<Vec<Lesson>>()
        .await?;
    Ok(lessons)
}

pub fn print_day(lessons: &[Lesson]) -> String {
    if lessons.is_empty() {
        return String::from("В этот день нет уроков");
    }
    lessons
        .iter()
        .map(|lesson| lesson.print())
        .collect::<Vec<String>>()
        .join("\n")
}
