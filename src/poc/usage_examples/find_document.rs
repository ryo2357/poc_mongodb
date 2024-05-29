use anyhow::Result;
use mongodb::{bson::doc, Collection};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct Restaurant {
    name: String,
    cuisine: String,
}

pub async fn find() -> Result<()> {
    let client = poc_mongodb::make_client().await?;
    let my_coll: Collection<Restaurant> = client
        .database("sample_restaurants")
        .collection("restaurants");
    let result = my_coll
        .find_one(doc! { "name": "Tompkins Square Bagels" }, None)
        .await?;
    println!("{:#?}", result);
    Ok(())
}
