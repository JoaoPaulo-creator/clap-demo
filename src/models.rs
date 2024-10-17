use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct NewItem {
    pub name: String,
    pub descriprion: String,
}

#[derive(Deserialize, Debug)]
pub struct ItemId {
    pub id: u64,
}

#[derive(Deserialize, Debug)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub description: String,
}
