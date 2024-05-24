use futures::TryStreamExt;
use mongodb::{bson::oid::ObjectId, Client};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Book {
    #[serde(rename = "_id", skip_serializing)]
    id: Option<ObjectId>,
    title: String,
    price: i32,
}

// const URI: &str = "mongodb://127.0.0.1:27017"; NG
// const URI: &str = "mongodb://mongo:27017"; NG
const URI: &str = "mongodb://root:example@mongo:27017";

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    // MongoDB サーバーに接続して Client インスタンスを取得
    let client = Client::with_uri_str(URI).await?;

    // データベースを列挙
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }

    // books コレクションのハンドルを取得する
    let books_coll = client.database("mydb2").collection::<Book>("books2");
    // コレクション内のドキュメントをすべて削除する
    books_coll.drop(None).await?;
    // コレクションにドキュメントを追加する
    let books = vec![
        Book {
            id: None,
            title: "Title-1".to_string(),
            price: 1000,
        },
        Book {
            id: None,
            title: "Title-2".to_string(),
            price: 2000,
        },
        Book {
            id: None,
            title: "Title-3".to_string(),
            price: 3000,
        },
    ];
    books_coll.insert_many(books, None).await?;

    // コレクション内のすべてのドキュメントを取得する
    let mut cursor = books_coll.find(None, None).await?;
    while let Some(book) = cursor.try_next().await? {
        println!("{:?}, {}, {}", book.id, book.title, book.price);
    }

    Ok(())
}
