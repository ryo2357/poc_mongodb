use anyhow::Result;
use futures::TryStreamExt;
use mongodb::{bson::doc, Collection};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct Restaurant {
    name: String,
    cuisine: String,
}
pub async fn multi_find() -> Result<()> {
    let client = poc_mongodb::make_client().await?;
    let my_coll: Collection<Restaurant> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let mut cursor = my_coll.find(doc! { "cuisine": "French" }, None).await?;

    while let Some(doc) = cursor.try_next().await? {
        println!("{:?}", doc);
    }
    Ok(())
}
