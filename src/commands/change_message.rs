use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::store::message_store::MessageStore;

#[command]
async fn change_message(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
  let new_message = match args.single_quoted::<String>() {
    Ok(x) => x,
    Err(_) => {
      msg.reply(ctx, "can not convert new message").await?;
      return Ok(());
    }
  };

  let message_lock = {
    let data_read = ctx.data.read().await;
    data_read
      .get::<MessageStore>()
      .expect("Expected MessageStore in TypeMap.")
      .clone()
  };
  {
    let mut mes = message_lock.write().await;
    *mes = new_message;
  }
  msg
    .reply(ctx, format!("new message: {}", message_lock.read().await))
    .await?;
  Ok(())
}
