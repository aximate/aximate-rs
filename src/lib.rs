use serde::{Deserialize, Serialize};
use std::clone::Clone;
pub mod addr;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Account {
  pub discord_id: String,
  pub name: String,
  pub ronin: String,
}
