#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;
pub mod db;

use teloxide::{prelude::*, utils::command::BotCommand};
use chrono::Duration;

use dotenv::dotenv;
use std::env;

use self::db::*;

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
    #[command(description = "get total hours from single project")]
    Hours,
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
            handle_log(user_id, cx, args).await?
        },
        Command::Echo => {
            if args.is_empty() {
                cx.answer(format!("I'm afraid I can't do that, {}", name)).send().await?
            } else {
                cx.answer(args.join(" ")).send().await?
            }
        },
        Command::Entries => {
            let project = args.get(0).unwrap();
            let entries = get_entries(user_id, project);
            let strs: Vec<String> = entries
                .into_iter()
                .map(|x| format!("{} hours at {}", x.hours, x.time + Duration::hours(2)))
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
            let projects = hours_by_project(user_id);
            let strs: Vec<String> = projects
                .into_iter()
                .map(|x| format!("{} {}", x.name, x.hours))
                .collect();
            cx.answer(strs.join("\n")).send().await?
        },
        Command::Hours => {
            let name = args.get(0);
            if let Some(x) = name {
                let hours = get_project_hours(x).unwrap();
                cx.answer(format!("{} hours in project {}", hours, x)).send().await?
            } else {
                cx.answer("Needs project name as argument").send().await?
            }
        }
    };

    Ok(())
}

async fn handle_commands(rx: DispatcherHandlerRx<Message>) {
    // Only iterate through commands in a proper format:
    rx.commands::<Command, &str>("workhours")
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

    // hours_by_project(0);    
    teloxide::enable_logging!();
    log::info!("Starting workhours_bot!");

    let bot = Bot::new(teloxide_token);

    Dispatcher::new(bot).messages_handler(handle_commands).dispatch().await;
}

async fn handle_log(
    user_id: i32, 
    cx: DispatcherHandlerCx<Message>, 
    args: Vec<String>
) -> Result<Message, RequestError> {
    if args.len() == 0 {
        return cx.answer("usage: /log <hours> <project>").send().await;
    }
    let hours = args.get(0);
    let project = args.get(1);
    if let Some(x) = hours {
        create_entry(user_id, x.parse::<f32>().unwrap(), project);
        cx.answer(format!("Logged {} hours", x)).send().await
    } else {
        cx.answer("expected hours as argument").send().await
    }
}