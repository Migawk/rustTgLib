extern crate dotenv;
use dotenv::dotenv;
use std::{env, thread::sleep, time::Duration};

mod telegram;
use telegram::Bot;

mod handler;
use handler::Handler;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("TOKEN").expect("Token not provided");

    let bot = Bot::new(token);

    let me = bot.get_me().await.unwrap();
    println!("{me:#?}");

    loop {
        match bot.get_updates().await {
            Ok(u) => {
                let msg = &u.message;

                Handler::handle(&bot, msg).await;
            }
            Err(_) => (),
        };

        sleep(Duration::from_secs(2));
    }
}
