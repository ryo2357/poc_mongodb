use anyhow::Result;
use mongodb::{bson::doc, Collection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Restaurant {
    borough: String,
    cuisine: String,
    name: String,
}

pub async fn insert() -> Result<()> {
    let client = poc_mongodb::make_client().await?;
    let my_coll: Collection<Restaurant> = client
        .database("sample_restaurants")
        .collection("restaurants");
    let doc = Restaurant {
        name: "Sea Stone Tavern".to_string(),
        cuisine: "Greek".to_string(),
        borough: "Queens".to_string(),
    };
    // 重複データも挿入される
    // 6658217c1b9b6e50225ad068
    let res = my_coll.insert_one(doc, None).await?;
    println!("Inserted a document with _id: {}", res.inserted_id);
    Ok(())
}
