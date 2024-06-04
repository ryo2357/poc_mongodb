use anyhow::Result;
use mongodb::{
    bson::{doc, Document},
    Collection,
};

pub async fn distinct() -> Result<()> {
    let client = poc_mongodb::make_client().await?;

    let my_coll: Collection<Document> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let filter = doc! { "cuisine": "Turkish" };
    let boroughs = my_coll.distinct("borough", filter, None).await?;
    println!("List of field values for 'borough':");
    for b in boroughs.iter() {
        println!("{:?}", b);
    }
    Ok(())
}
