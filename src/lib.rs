use anyhow::Result;
use mongodb::Client;

pub async fn make_client() -> Result<Client> {
    // let client_uri = std::env::var("ATLAS_URI").expect("You must set environment var!");
    let client_uri = std::env::var("LOCAL_URI").expect("You must set environment var!");
    let client = Client::with_uri_str(client_uri).await?;
    Ok(client)
}
