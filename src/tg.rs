use dotenv::dotenv;
// use std::sync::{Arc, Mutex};
use std::time::Duration;
use teloxide::{prelude::*, utils::command::BotCommands};
use tokio::time::sleep;

#[tokio::main]
pub async fn start() {
    dotenv().ok();

    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();
    // let chat_ids = Arc::new(Mutex::new(Vec::new()));
    let chat_ids: Vec<i64> = vec![457923379];

    let bot_clone = bot.clone();
    // let chat_ids_clone = chat_ids.clone();

    // Spawn a new thread to send a message every minute
    // let bot_clone = bot.clone();
    tokio::spawn(async move {
        loop {
            // let ids = chat_ids_clone.lock().unwrap();
            // let ids = chat_ids;

            for &chat_id in &*chat_ids {
                let chat_id = ChatId(chat_id);

                if let Err(e) = bot_clone
                    .send_message(chat_id, "This is a periodic message.")
                    .await
                {
                    log::error!("Failed to send message to chat ID {}: {:?}", chat_id, e);
                }
            }

            sleep(Duration::from_secs(60)).await;
        }
    });

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    // let chat_ids = chat_ids.clone();
    // let mut ids = chat_ids.lock().unwrap();
    // ids.push(msg.chat.id);

    println!("{}", msg.chat.id);

    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Username(username) => {
            bot.send_message(msg.chat.id, format!("Your username is @{username}."))
                .await?
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(
                msg.chat.id,
                format!("Your username is @{username} and age is {age}."),
            )
            .await?
        }
    };

    Ok(())
}
