#[macro_use]
extern crate log;

use carapax::{types::Command, Api, Config, Dispatcher, ExecuteError};
use serde::{Deserialize, Serialize};
use simplelog::*;

#[derive(Debug, Clone, Copy)]
enum DayOfTheWeek {
    Monday,
    Tuesday,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Lesson {
    name: String,
    #[serde(rename = "type")]
    lesson_type: String,
    link: String,
    password: Option<String>,
}

impl Lesson {
    fn print(&self) -> String {
        format!("[{}]({}): {}", self.name, self.link, self.lesson_type)
    }
}

async fn get_day_timetable(day: DayOfTheWeek) -> Result<Vec<Lesson>, reqwest::Error> {
    let file = match day {
        DayOfTheWeek::Monday => "monday.json",
        DayOfTheWeek::Tuesday => "tuesday.json",
    };
    let lessons = reqwest::get(&format!("http://localhost:8000/timetable/{}", file))
        .await?
        .json::<Vec<Lesson>>()
        .await?;
    Ok(lessons)
}

fn print_day(lessons: &[Lesson]) -> String {
    lessons
        .iter()
        .map(|lesson| lesson.print())
        .collect::<Vec<String>>()
        .join("\n")
}

#[carapax::handler(command = "/get_day")]
async fn get_day_handler(
    api: &Api,
    command: Command,
) -> Result<carapax::HandlerResult, ExecuteError> {
    let message = command.get_message();
    let chat_id = message.get_chat_id();
    let args = command.get_args();
    info!("Got command /get_day from {} with args {:?}", chat_id, args);
    let day = if args[0].starts_with("mon") {
        DayOfTheWeek::Monday
    } else {
        DayOfTheWeek::Tuesday
    };
    let response = get_day_timetable(day).await?;
    let method = carapax::methods::SendMessage::new(chat_id, print_day(&response))
        .disable_web_page_preview(true)
        .parse_mode(carapax::types::ParseMode::MarkdownV2);
    api.execute(method).await?;
    Ok(carapax::HandlerResult::Stop)
}

#[tokio::main]
async fn main() {
    TermLogger::init(
        LevelFilter::Info,
        simplelog::Config::default(),
        TerminalMode::Mixed,
    )
    .unwrap();
    let token = std::env::var("CARAPAX_TOKEN").expect("CARAPAX_TOKEN is not set");
    let config = Config::new(token);
    let api = Api::new(config).expect("Failed to create API");
    let mut dispatcher = Dispatcher::new(api.clone());
    dispatcher.add_handler(get_day_handler);
    debug!("Starting bot");
    carapax::longpoll::LongPoll::new(api, dispatcher)
        .run()
        .await
}
