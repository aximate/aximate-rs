use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub discord_id: &'static str,
    pub name: &'static str,
    pub ronin: &'static str,
}
