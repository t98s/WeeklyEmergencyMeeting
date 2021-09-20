extern crate ini;
use ini::Ini;

use serenity::framework::standard::StandardFramework;
use serenity::{async_trait, model::gateway::Ready, model::id::ChannelId, prelude::*};

use chrono::Weekday;
use tokio_schedule::{every, Job};

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
  let message = section.get("message").unwrap();
  let week_day = section.get("weekDay").unwrap();
  let c_id = section.get("channelId").unwrap();
  let c = ChannelId(c_id.parse().unwrap());
  let hour = section.get("hour").unwrap().parse().unwrap();
  let minute = section.get("minute").unwrap().parse().unwrap();

  let w = if week_day == "Mon" {
    Weekday::Mon
  } else if week_day == "Tue" {
    Weekday::Tue
  } else if week_day == "Wed" {
    Weekday::Wed
  } else if week_day == "Thu" {
    Weekday::Thu
  } else if week_day == "Fri" {
    Weekday::Fri
  } else if week_day == "Sat" {
    Weekday::Sat
  } else if week_day == "Sun" {
    Weekday::Sun
  } else {
    panic!("weekDay can set Mon,Tue,Wed,Thu,Fri,Sat,Sun in conf.");
  };

  let framework = StandardFramework::new();
  let mut client = Client::builder(&token)
    .event_handler(Handler)
    .framework(framework)
    .await
    .expect("Err creating client.");
  let http = client.cache_and_http.http.clone();

  let job = every(1).week().on(w).at(hour, minute).perform(|| async {
    println!("Start announce");
    match c.say(&http, message).await {
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
