extern crate ini;
use ini::Ini;

use serenity::framework::standard::StandardFramework;
use serenity::{async_trait, model::gateway::Ready, model::id::ChannelId, prelude::*};

use tokio_schedule::{every, Job};

extern crate weeklyemergencymeeting as wem;
use wem::utils;

extern crate state;
use state::LocalStorage;

use std::cell::RefCell;

static MESSAGE: LocalStorage<RefCell<String>> = LocalStorage::new();

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
  MESSAGE.set(|| RefCell::new(String::from("")));

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
  *MESSAGE.get().borrow_mut() = mes.to_owned();

  // set initial weekday
  let w = utils::week_str_to_week_day(week_day);

  println!("announce: {}:{} on {:?}", hour, minute, w);

  let framework = StandardFramework::new();
  let mut client = Client::builder(&token)
    .event_handler(Handler)
    .framework(framework)
    .await
    .expect("Err creating client.");
  let http = client.cache_and_http.http.clone();

  let job = every(1)
    .week()
    .on(w)
    .at(hour, minute, 00)
    .perform(|| async {
      let mes = MESSAGE.get().take(); // take()じゃないとasync中に取り出せない
      println!("Start announce: {:?}", mes);
      // take()しちゃったのでもう一回収めるために、clone()を送る
      match c.say(&http, mes.clone()).await {
        Ok(mes) => {
          println!("send message: {:?}", mes);
        }
        Err(err) => {
          println!("send message: {:?}", err);
        }
      }
      *MESSAGE.get().borrow_mut() = mes; // take()しちゃったのでもう一回収める
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
