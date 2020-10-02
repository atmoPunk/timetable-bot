#[macro_use]
extern crate log;

mod day;
mod handlers;
mod lesson;

use crate::handlers::*;

#[tokio::main]
async fn main() {
    simplelog::TermLogger::init(
        simplelog::LevelFilter::Info,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
    )
    .unwrap();
    let token = std::env::var("CARAPAX_TOKEN").expect("CARAPAX_TOKEN is not set");
    let config = carapax::Config::new(token);
    let api = carapax::Api::new(config).expect("Failed to create API");
    let mut dispatcher = carapax::Dispatcher::new(api.clone());
    dispatcher.add_handler(get_day_handler);
    dispatcher.add_handler(get_today_handler);
    info!("Starting bot");
    carapax::longpoll::LongPoll::new(api, dispatcher)
        .run()
        .await
}
