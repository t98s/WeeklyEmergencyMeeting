use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct MessageStore;
impl TypeMapKey for MessageStore {
  type Value = Arc<RwLock<String>>;
}
