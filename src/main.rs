use teloxide::{prelude::*, utils::command::BotCommand};

use dotenv::dotenv;
use std::env;


#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "log hours from today.")]
    Log,
    #[command(description = "echo arguments")]
    Echo,
}


async fn answer(
    cx: DispatcherHandlerCx<Message>,
    command: Command,
    args: Vec<String>
) -> ResponseResult<()> {
    let name = if let Some(x) = cx.update.from() {
        &x.first_name
    } else {
        "anonymous"
    };
    
    match command {
        Command::Help => cx.answer(Command::descriptions()).send().await?,
        Command::Log => {
            let hours = args.get(0);
            if let Some(x) = hours {
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