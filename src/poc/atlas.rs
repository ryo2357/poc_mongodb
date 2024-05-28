use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};

pub async fn tutorial() -> mongodb::error::Result<()> {
    // [Connect to MongoDB - Rust Driver v2.8](https://www.mongodb.com/docs/drivers/rust/current/quick-start/connect-to-mongodb/)
    let client_uri = std::env::var("ATLAS_URI").expect("You must set environment var!");
    // println!("{}", client_uri);
    let client = Client::with_uri_str(client_uri).await?;

    // Get a handle on the movies collection
    let database = client.database("sample_mflix");
    let my_coll: Collection<Document> = database.collection("movies");
    // Find a movie based on the title value
    let my_movie = my_coll
        .find_one(doc! { "title": "The Perils of Pauline" }, None)
        .await?;
    // Print the document
    println!("Found a movie:\n{:#?}", my_movie);
    Ok(())
}
