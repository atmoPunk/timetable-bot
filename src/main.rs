#[macro_use]
extern crate log;

mod context;
mod handlers;
mod lesson;
mod weekday_wrapper;

#[tokio::main]
async fn main() {
    simplelog::TermLogger::init(
        simplelog::LevelFilter::Info,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
    )
    .unwrap();

    dotenv::dotenv().expect(".env not found");

    let token = std::env::var("CARAPAX_TOKEN").expect("CARAPAX_TOKEN is not set");
    let config = carapax::Config::new(token);
    let api = carapax::Api::new(config).expect("Failed to create API");
    let session_dir = std::env::var("SESSION_DIRECTORY").unwrap();
    let session_dir = std::path::PathBuf::from(session_dir);
    let backend = carapax::session::backend::fs::FilesystemBackend::new(&session_dir);

    let authorized_users = std::env::var("AUTHORIZED_USERS")
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.to_string())
        .collect::<std::collections::HashSet<String>>();

    handlers::AUTHORIZED_USERS.set(authorized_users).unwrap();

    let mut dispatcher = carapax::Dispatcher::new(context::Context {
        api: api.clone(),
        session_manager: carapax::session::SessionManager::new(backend),
    });
    dispatcher.add_handler(handlers::get_day_handler);
    dispatcher.add_handler(handlers::get_today_handler);
    dispatcher.add_handler(handlers::get_next_lesson_handler);
    dispatcher.add_handler(handlers::set_group_handler);
    dispatcher.add_handler(handlers::get_group_handler);
    info!("Starting bot");
    carapax::longpoll::LongPoll::new(api, dispatcher)
        .run()
        .await
}
