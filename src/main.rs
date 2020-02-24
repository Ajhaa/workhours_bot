#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;
use teloxide::{prelude::*, utils::command::BotCommand};
use self::models::{LogEntry, NewLogEntry};

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;


#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "log hours from today.")]
    Log,
    #[command(description = "echo arguments")]
    Echo,
    #[command(description = "find entries")]
    Entries,

}


async fn answer(
    cx: DispatcherHandlerCx<Message>,
    command: Command,
    args: Vec<String>,
) -> ResponseResult<()> {
    let name = if let Some(x) = cx.update.from() {
        &x.first_name
    } else {
        "Anonymous"
    };

    let user_id = if let Some(x) = cx.update.from() {
        x.id
    } else {
        0
    };

    match command {
        Command::Help => cx.answer(Command::descriptions()).send().await?,
        Command::Log => {
            let hours = args.get(0);
            if let Some(x) = hours {
                create_entry(user_id, x.parse::<f32>().unwrap());
                cx.answer(format!("Logged {} hours", x)).send().await?
            } else {
                cx.answer("expected hours as argument").send().await?
            }
        },
        Command::Echo => {
            if args.is_empty() {
                cx.answer(format!("I'm afraid I can't do that, {}", name)).send().await?
            } else {
                cx.answer(args.join(" ")).send().await?
            }
        },
        Command::Entries => {
            let entries = get_entries();
            let strs: Vec<String> = entries.into_iter().map(|x| format!("{} hours for user {}", x.hours, x.user_id)).collect();
            cx.answer(strs.join("\n")).send().await?
        }
    };

    Ok(())
}

async fn handle_commands(rx: DispatcherHandlerRx<Message>) {
    // Only iterate through commands in a proper format:
    rx.commands::<Command>()
        // Execute all incoming commands concurrently:
        .for_each_concurrent(None, |(cx, command, args)| async move {
            answer(cx, command, args).await.log_on_error().await;
        })
        .await;
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    dotenv().ok();
    let teloxide_token = env::var("TELOXIDE_TOKEN")
        .expect("TELOXIDE_TOKEN must be set");

    teloxide::enable_logging!();
    log::info!("Starting workhours_bot!");

    let bot = Bot::new(teloxide_token);

    Dispatcher::new(bot).messages_handler(handle_commands).dispatch().await;
}

pub fn get_entries() -> Vec<LogEntry> {
    use schema::log_entry::dsl::*;

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    log_entry
        .limit(5)
        .load::<LogEntry>(&connection)
        .expect("Error loading posts")
}

pub fn create_entry(user_id: i32, hours: f32) -> LogEntry {
    use schema::log_entry;
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let conn = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    let new_entry = NewLogEntry {
        hours: hours,
        user_id: user_id,
    };

    diesel::insert_into(log_entry::table)
        .values(&new_entry)
        .get_result(&conn)
        .expect("Error saving new entry")
}