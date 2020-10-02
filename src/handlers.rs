use crate::day::WeekdayWrapper;
use crate::lesson::{get_day_timetable, print_day};
use carapax::{types::Command, Api, ExecuteError};

use std::convert::TryFrom;

#[carapax::handler(command = "/get_today")]
pub async fn get_today_handler(
    api: &Api,
    command: Command,
) -> Result<carapax::HandlerResult, ExecuteError> {
    let chat_id = command.get_message().get_chat_id();
    info!("Got command /get_today from {}", chat_id);
    let day = WeekdayWrapper::get_today();
    let response = get_day_timetable(day.to_json_file()).await?;
    let method = carapax::methods::SendMessage::new(chat_id, print_day(&response))
        .disable_web_page_preview(true)
        .parse_mode(carapax::types::ParseMode::MarkdownV2);
    api.execute(method).await?;
    Ok(carapax::HandlerResult::Stop)
}

#[carapax::handler(command = "/get_day")]
pub async fn get_day_handler(
    api: &Api,
    command: Command,
) -> Result<carapax::HandlerResult, ExecuteError> {
    let chat_id = command.get_message().get_chat_id();
    let args = command.get_args();
    info!("Got command /get_day from {} with args {:?}", chat_id, args);
    let day = WeekdayWrapper::try_from(&args[0][..]);
    if day.is_err() {
        let method =
            carapax::methods::SendMessage::new(chat_id, format!("Bad argument: {}", args[0]));
        api.execute(method).await?;
        return Ok(carapax::HandlerResult::Stop);
    }
    let day = day.unwrap();
    let response = get_day_timetable(day.to_json_file()).await?;
    let method = carapax::methods::SendMessage::new(chat_id, print_day(&response))
        .disable_web_page_preview(true)
        .parse_mode(carapax::types::ParseMode::MarkdownV2);
    api.execute(method).await?;
    Ok(carapax::HandlerResult::Stop)
}