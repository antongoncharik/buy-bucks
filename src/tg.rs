use dotenv::dotenv;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use teloxide::{prelude::*, utils::command::BotCommands};
use tokio::time::sleep;

use crate::bnb;
use crate::nbrb;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    Start,
    Status,
}

#[tokio::main]
pub async fn start() {
    dotenv().ok();

    let bot = Bot::from_env();
    let chat_ids: Arc<Mutex<HashSet<i64>>> =
        Arc::new(Mutex::new(vec![457923379].into_iter().collect()));

    let bot_clone = bot.clone();
    let chat_ids_clone = chat_ids.clone();

    let ids = chat_ids_clone.lock().unwrap().clone();

    let msg = format!("Start");

    for &chat_id in ids.iter() {
        let chat_id = ChatId(chat_id);

        if let Err(e) = bot_clone.send_message(chat_id, &msg).await {
            eprint!("Failed to send message to chat ID {}: {:?}", chat_id, e)
        }
    }

    tokio::spawn(async move {
        loop {
            let nbrb_price = nbrb::get_price().await.unwrap();
            let bnb_price = bnb::get_price().await.unwrap();

            let buy = nbrb_price > bnb_price;

            if buy {
                let msg = format!("NBRN: {}\nBNB: {}", nbrb_price, bnb_price);

                let ids = chat_ids_clone.lock().unwrap().clone();

                for &chat_id in ids.iter() {
                    let chat_id = ChatId(chat_id);

                    if let Err(e) = bot_clone.send_message(chat_id, &msg).await {
                        eprint!("Failed to send message to chat ID {}: {:?}", chat_id, e)
                    }
                }
            }

            sleep(Duration::from_secs(60 * 15)).await;
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
    chat_ids: Arc<Mutex<HashSet<i64>>>,
) -> ResponseResult<()> {
    {
        let mut ids = chat_ids.lock().unwrap();
        ids.insert(msg.chat.id.0);
    }

    let start_msg = format!("Start");
    let status_msg = format!("Alive");

    match cmd {
        Command::Start => bot.send_message(msg.chat.id, start_msg).await?,
        Command::Status => bot.send_message(msg.chat.id, status_msg).await?,
    };

    Ok(())
}
