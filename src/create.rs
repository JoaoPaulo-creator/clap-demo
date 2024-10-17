use crate::models::{ItemId, NewItem};
use reqwest::Client;

pub async fn create_item(client: &Client) -> Result<()> {
    let new_item = NewItem {
        name: "Item teste",
        description: "esse item é um teste",
    };

    Ok(())
}
