use anyhow::Result;
use mongodb::{
    bson::{doc, Document},
    Collection,
};

pub async fn update() -> Result<()> {
    let client = poc_mongodb::make_client().await?;

    let my_coll: Collection<Document> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let filter = doc! {
        "address.street": "Sullivan Street",
        "borough": "Manhattan"
    };
    let update = doc! { "$set": doc! { "near_me": true } };
    let res = my_coll.update_many(filter, update, None).await?;
    println!("Updated documents: {}", res.modified_count);
    Ok(())
}
