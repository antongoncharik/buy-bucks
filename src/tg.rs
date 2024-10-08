use dotenv::dotenv;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use teloxide::{prelude::*, utils::command::BotCommands};
use tokio::time::sleep;

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
}

#[tokio::main]
pub async fn start() {
    dotenv().ok();

    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();
    let chat_ids: Arc<Mutex<Vec<i64>>> = Arc::new(Mutex::new(vec![]));

    let bot_clone = bot.clone();
    let chat_ids_clone = chat_ids.clone();

    tokio::spawn(async move {
        loop {
            let ids = chat_ids_clone.lock().unwrap().clone();

            for &chat_id in ids.iter() {
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

    Command::repl(bot, move |bot, msg, cmd| {
        let chat_ids_clone = chat_ids.clone();
        async move { answer(bot, msg, cmd, chat_ids_clone).await }
    })
    .await;
}

async fn answer(
    bot: Bot,
    msg: Message,
    cmd: Command,
    chat_ids: Arc<Mutex<Vec<i64>>>,
) -> ResponseResult<()> {
    {
        let mut ids = chat_ids.lock().unwrap();
        ids.push(msg.chat.id.0);
    }

    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Username(username) => {
            bot.send_message(msg.chat.id, format!("Your username is @{username}."))
                .await?
        }
    };

    Ok(())
}
