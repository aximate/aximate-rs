use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub discord_id: String,
    pub name: String,
    pub ronin: String,
}
