extern crate telegram_bot_raw;
extern crate telegram_bot_tokio;
extern crate tokio_core;
extern crate futures;

use std::env;

use futures::{Stream};
use tokio_core::reactor::Core;
use telegram_bot_tokio::Bot;
use telegram_bot_raw::{MessageKind, UpdateKind, CanReplySendMessage};

fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();

    let mut core = Core::new().unwrap();
    let bot = Bot::from_token(&core.handle(), &token).unwrap();

    let future = bot.stream().for_each(|update| {
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text {ref data, ..} = message.kind {
                let text = format!("Got the message: '{}'", data);
                bot.spawn(message.text_reply(text));
            }
        }
        Ok(())
    });

    core.run(future).unwrap();
}
