use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::store::message_store::MessageStore;

#[command]
async fn show_message(ctx: &Context, msg: &Message) -> CommandResult {
  let message_lock = {
    let data_read = ctx.data.read().await;
    data_read
      .get::<MessageStore>()
      .expect("Expected MessageStore in TypeMap.")
      .clone()
  };
  let mes = message_lock.read().await;
  if mes.len() == 0 {
    msg.reply(ctx, "<!not set message!>").await?;
  } else {
    msg.reply(ctx, mes).await?;
  }
  Ok(())
}
