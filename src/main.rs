#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;
pub mod db;
use teloxide::{prelude::*, utils::command::BotCommand};
use self::db::*;

use dotenv::dotenv;
use std::env;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "/log <hours> log hours from today.")]
    Log,
    #[command(description = "echo arguments")]
    Echo,
    #[command(description = "find entries")]
    Entries,
    #[command(description = "/project <name> - create a new project")]
    Project,
    #[command(description = "list your projects")]
    Projects,
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
        -1
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
            let entries = get_entries(user_id);
            let strs: Vec<String> = entries
                .into_iter()
                .map(|x| format!("{} hours for user {} at {}", x.hours, x.user_id, x.time))
                .collect();
            cx.answer(strs.join("\n")).send().await?
        },
        Command::Project => {
            let name = args.get(0);
            if let Some(x) = name {
                create_project(x, user_id);
                cx.answer(format!("Created project {}", x)).send().await?
            } else {
                cx.answer("Cannot create a group without a name").send().await?
            }
        },
        Command::Projects => {
            let projects = get_projects(user_id);
            let strs: Vec<String> = projects
                .into_iter()
                .map(|x| format!("{}", x.name))
                .collect();
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