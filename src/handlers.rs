use crate::context::Context;
use crate::lesson::{get_day_timetable, print_day};
use crate::weekday_wrapper::WeekdayWrapper;
use carapax::{methods::SendMessage, types::Command, ExecuteError};
use std::convert::TryFrom;

#[carapax::handler(command = "/set_group")]
pub async fn set_group_handler(
    context: &Context,
    command: Command,
) -> Result<carapax::HandlerResult, ExecuteError> {
    let chat_id = command.get_message().get_chat_id();
    let args = command.get_args();
    info!(
        "Got command /set_group from {} with args {:?}",
        chat_id, args
    );

    if args.is_empty() {
        let method = SendMessage::new(chat_id, "Possible group values are: M4140 or M4141");
        context.api.execute(method).await?;
        return Ok(carapax::HandlerResult::Stop);
    }

    let group = &args[0];
    // TODO: handle cyrillic 'М'
    if group != "M4140" && group != "M4141" {
        let method = SendMessage::new(chat_id, "Possible group values are: M4140 or M4141");
        context.api.execute(method).await?;
        return Ok(carapax::HandlerResult::Stop);
    }

    let mut session = context.session_manager.get_session(&command).unwrap();
    session.set("group", group).await.unwrap();
    context
        .api
        .execute(SendMessage::new(chat_id, "Group set"))
        .await?;
    Ok(carapax::HandlerResult::Stop)
}

#[carapax::handler(command = "/get_group")]
pub async fn get_group_handler(
    context: &Context,
    command: Command,
) -> Result<carapax::HandlerResult, ExecuteError> {
    let chat_id = command.get_message().get_chat_id();
    info!("Got command /get_group from {}", chat_id);
    let mut session = context.session_manager.get_session(&command).unwrap();
    let group: Option<String> = session.get("group").await.unwrap();
    let message = match group {
        Some(g) => format!("Your group is {}", g),
        None => String::from("Your group is not set"),
    };
    context
        .api
        .execute(SendMessage::new(chat_id, message))
        .await?;
    Ok(carapax::HandlerResult::Stop)
}

#[carapax::handler(command = "/get_today")]
pub async fn get_today_handler(
    context: &Context,
    command: Command,
) -> Result<carapax::HandlerResult, ExecuteError> {
    let chat_id = command.get_message().get_chat_id();
    info!("Got command /get_today from {}", chat_id);
    let day = WeekdayWrapper::get_today();
    let lessons = get_day_timetable(day.to_json_file()).await?;
    let method = carapax::methods::SendMessage::new(chat_id, print_day(&lessons))
        .disable_web_page_preview(true)
        .parse_mode(carapax::types::ParseMode::MarkdownV2);
    context.api.execute(method).await?;
    Ok(carapax::HandlerResult::Stop)
}

#[carapax::handler(command = "/get_next_lesson")]
pub async fn get_next_lesson_handler(
    context: &Context,
    command: Command,
) -> Result<carapax::HandlerResult, ExecuteError> {
    let chat_id = command.get_message().get_chat_id();
    info!("Got command /get_next_lesson from {}", chat_id);
    let day = WeekdayWrapper::get_today();
    let lessons = get_day_timetable(day.to_json_file()).await?;
    let current_time = chrono::Local::now();
    let next_lesson = lessons.iter().find(|&l| l.is_next(&current_time));
    let message = match next_lesson {
        Some(lesson) => lesson.print(),
        None => String::from("Сегодня больше нет уроков"),
    };
    let method = carapax::methods::SendMessage::new(chat_id, message)
        .disable_web_page_preview(true)
        .parse_mode(carapax::types::ParseMode::MarkdownV2);
    context.api.execute(method).await?;
    Ok(carapax::HandlerResult::Stop)
}

#[carapax::handler(command = "/get_day")]
pub async fn get_day_handler(
    context: &Context,
    command: Command,
) -> Result<carapax::HandlerResult, ExecuteError> {
    let chat_id = command.get_message().get_chat_id();
    let args = command.get_args();
    info!("Got command /get_day from {} with args {:?}", chat_id, args);
    let day = WeekdayWrapper::try_from(&args[0][..]);
    if day.is_err() {
        let method =
            carapax::methods::SendMessage::new(chat_id, format!("Bad argument: {}", args[0]));
        context.api.execute(method).await?;
        return Ok(carapax::HandlerResult::Stop);
    }
    let day = day.unwrap();
    let lessons = get_day_timetable(day.to_json_file()).await?;
    let method = carapax::methods::SendMessage::new(chat_id, print_day(&lessons))
        .disable_web_page_preview(true)
        .parse_mode(carapax::types::ParseMode::MarkdownV2);
    context.api.execute(method).await?;
    Ok(carapax::HandlerResult::Stop)
}
