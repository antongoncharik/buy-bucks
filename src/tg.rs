use dotenv::dotenv;
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
    #[command(description = "display this text.")]
    Help,
}

#[tokio::main]
pub async fn start() {
    dotenv().ok();

    let bot = Bot::from_env();
    let chat_ids: Arc<Mutex<Vec<i64>>> = Arc::new(Mutex::new(vec![457923379]));

    let bot_clone = bot.clone();
    let chat_ids_clone = chat_ids.clone();

    tokio::spawn(async move {
        loop {
            let nbrb_price = nbrb::get_price().await.unwrap();
            let bnb_price = bnb::get_price().await.unwrap();

            let buy = nbrb_price > bnb_price;
            let msg = format!("BUY: {}\nNBRN: {}\nBNB: {}", buy, nbrb_price, bnb_price);

            let ids = chat_ids_clone.lock().unwrap().clone();

            for &chat_id in ids.iter() {
                let chat_id = ChatId(chat_id);

                if let Err(e) = bot_clone.send_message(chat_id, &msg).await {
                    eprint!("Failed to send message to chat ID {}: {:?}", chat_id, e)
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
        println!("{}", msg.chat.id.0)
    }

    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
    };

    Ok(())
}
