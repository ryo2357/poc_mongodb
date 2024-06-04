use anyhow::Result;
use mongodb::{
    bson::{doc, Document},
    Collection,
};

pub async fn count() -> Result<()> {
    let client = poc_mongodb::make_client().await?;

    let my_coll: Collection<Document> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let ct = my_coll.estimated_document_count(None).await?;
    println!("Number of documents: {}", ct);
    let ct = my_coll
        .count_documents(doc! { "Sunset": doc! { "$regex": "Sunset" } }, None)
        .await?;
    println!("Number of matching documents: {}", ct);
    Ok(())
}
