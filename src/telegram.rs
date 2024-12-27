use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct StrctGetMe {
    pub ok: bool,
    pub result: Value,
}
#[derive(Serialize)]
pub struct SendBody {
    pub chat_id: i64,
    pub text: String,
}

#[derive(Deserialize, Debug, Clone)]
struct UpdateResp {
    pub ok: bool,
    pub result: Vec<UpdateUnit>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct UpdateUnit {
    pub message: Message,
    pub update_id: i64,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Message {
    pub message_id: i64,
    pub from: User,
    pub text: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub id: i64,
}
#[derive(Clone)]
pub struct Bot {
    token: String,
    link: String,
}

impl Bot {
    pub fn new(token: String) -> Bot {
        Bot {
            token: String::from(&token),
            link: format!("https://api.telegram.org/bot{}/", &token),
        }
    }
    async fn send(
        &self,
        method: &str,
        body: Option<Value>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let params = serde_urlencoded::to_string(body).unwrap();

        let main = &self.link;
        let link = format!("{main}{method}?{params}");

        Ok(reqwest::get(link).await?.text().await?)
    }

    pub async fn get_me(&self) -> Result<StrctGetMe, ()> {
        match self.send("getMe", None).await {
            Ok(a) => {
                let res: StrctGetMe = serde_json::from_str(&a).unwrap();
                Ok(res)
            }
            Err(_) => Err(()),
        }
    }
    pub async fn send_message(&self, body: SendBody) -> bool {
        let bd = serde_json::to_value(body).unwrap();

        match self.send("sendMessage", Some(bd)).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    pub async fn get_updates(&self) -> Result<UpdateUnit, ()> {
        let raw = self.send("getUpdates", None).await.unwrap();
        let res: UpdateResp = serde_json::from_str(&raw).unwrap();


        if res.result.len() > 0 {
            #[derive(Serialize)]
            struct UpdateOffset {
                offset: i64,
            }
            let bd = serde_json::to_value(UpdateOffset {
                offset: res.result[0].update_id + 1,
            })
            .unwrap();

            self.send("getUpdates", Some(bd))
                .await
                .expect("Failed update");
            let unit = &res.result[0];
            Ok(unit.clone())
        } else {
            Err(())
        }
    }
}
