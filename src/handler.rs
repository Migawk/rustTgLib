use crate::telegram::{Bot, Message, SendBody};

pub struct Handler {}
impl Handler {
    pub async fn handle(bot: &Bot, message: &Message) {
        let text: &str = &message.text;

        match text {
            "/start" => {
                let bd = SendBody {
                    chat_id: message.from.id,
                    text: String::from(
                        "Hello, there! I'm bot that written in Rust.\nMore info: /info",
                    ),
                };
                bot.send_message(bd).await;
            }
            "/info" => {
                let bd = SendBody {
                    chat_id: message.from.id,
                    text: String::from("@ewgenka - is author"),
                };
                bot.send_message(bd).await;
            }
            &_ => println!("Smthn else"),
        }
    }
}
