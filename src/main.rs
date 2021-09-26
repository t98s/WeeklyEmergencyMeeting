extern crate ini;
use ini::Ini;

use serenity::framework::standard::{
  macros::{group, hook},
  StandardFramework,
};
use serenity::{
  async_trait,
  model::id::ChannelId,
  model::{channel::Message, gateway::Ready},
  prelude::*,
};

use tokio::sync::RwLock;
use tokio_schedule::{every, Job};

extern crate weeklyemergencymeeting as wem;
use wem::commands::{change_message::CHANGE_MESSAGE_COMMAND, show_message::SHOW_MESSAGE_COMMAND};
use wem::store::message_store::MessageStore;
use wem::utils::week_str_to_week_day;

use std::sync::Arc;

#[group]
#[commands(change_message, show_message)]
struct General;

#[hook]
async fn before(_: &Context, msg: &Message, command_name: &str) -> bool {
  println!(
    "Running command '{}' invoked by '{}'",
    command_name,
    msg.author.tag()
  );
  true
}

struct Handler;
#[async_trait]
impl EventHandler for Handler {
  async fn ready(&self, _: Context, _ready: Ready) {
    println!("client boot.");
  }
}

#[tokio::main]
async fn main() {
  println!("Hello, this is WeeklyEmergencyMeeting.");

  let conf = Ini::load_from_file("config.ini").unwrap();
  let section = conf.section(Some("Discord")).unwrap();
  let token = section.get("token").unwrap();
  let week_day = section.get("weekDay").unwrap();
  let c_id = section.get("channelId").unwrap();
  let c = ChannelId(c_id.parse().unwrap());
  let hour = section.get("hour").unwrap().parse().unwrap();
  let minute = section.get("minute").unwrap().parse().unwrap();

  // set initial message from conf
  let mes = section.get("message").unwrap();

  // set initial weekday
  let w = week_str_to_week_day(week_day);

  println!("announce: {}:{} on {:?}", hour, minute, w);

  let framework = StandardFramework::new()
    .configure(|c| c.with_whitespace(true).prefix("!"))
    .before(before)
    .group(&GENERAL_GROUP);
  let mut client = Client::builder(&token)
    .event_handler(Handler)
    .framework(framework)
    .await
    .expect("Err creating client.");
  let data = client.data.clone();
  {
    // init store
    let mut data = data.write().await;
    data.insert::<MessageStore>(Arc::new(RwLock::new(String::from(mes))));
  }

  let http = client.cache_and_http.http.clone();

  let job = every(1)
    .week()
    .on(w)
    .at(hour, minute, 00)
    .perform(|| async {
      let mes_lock = {
        let data_read = data.read().await;
        data_read
          .get::<MessageStore>()
          .expect("Expected MessageStore in TypeMap.")
          .clone()
      };
      let mes = mes_lock.read().await;
      println!("Start announce: {:?}", mes);
      match c.say(&http, mes).await {
        Ok(mes) => {
          println!("send message: {:?}", mes);
        }
        Err(err) => {
          println!("send message: {:?}", err);
        }
      }
    });

  tokio::select! {
    _ = job => {
      println!("schedule something");
    },
    Err(why) = client.start() => {
    eprintln!("Client error: {:?}", why);
    },
  }
}
