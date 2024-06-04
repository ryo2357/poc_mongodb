use anyhow::Result;
use mongodb::{bson::doc, Collection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Restaurant {
    borough: String,
    cuisine: String,
    name: String,
}

pub async fn replace() -> Result<()> {
    let client = poc_mongodb::make_client().await?;
    let my_coll: Collection<Restaurant> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let filter = doc! { "name": "Landmark Coffee Shop" };
    let replacement = Restaurant {
        borough: "Brooklyn".to_string(),
        cuisine: "Café/Coffee/Tea".to_string(),
        name: "Harvest Moon Café".to_string(),
    };
    // 他のフィールドも消える。オブジェクトごと更新される
    let res = my_coll.replace_one(filter, replacement, None).await?;
    println!("Replaced documents: {}", res.modified_count);
    Ok(())
}
