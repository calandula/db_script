pub mod config;

use mongodb::IndexModel;
use mongodb::{Client, options::ClientOptions};
use mongodb::bson::{doc, Document};
use tokio;
use envy;
use config::Configuration;

async fn create_materialized_view() {
    let conf = envy::from_env::<Configuration>().unwrap();
    let mut client_options = ClientOptions::parse(&conf.db_path).await.expect("Client error");
    client_options.app_name = Some(conf.app_name);
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(&conf.db_name.as_str());
    let pipeline = vec![
        doc! {
            "$lookup": {
                "from": "users",
                "localField": "userId",
                "foreignField": "_id",
                "as": "user"
            }
        },
        doc! {
            "$lookup": {
                "from": "users",
                "localField": "managerId",
                "foreignField": "_id",
                "as": "manager"
            }
        },
        doc! {
            "$lookup": {
                "from": "scores",
                "localField": "userId",
                "foreignField": "userId",
                "as": "scores"
            }
        },
        doc! {
            "$lookup": {
                "from": "reminders",
                "localField": "_id",
                "foreignField": "appId",
                "as": "reminders"
            }
        },
        doc! {
            "$lookup": {
                "from": "providersnapshots",
                "localField": "_id",
                "foreignField": "appId",
                "as": "providersnapshots"
            }
        },
        doc! {
            "$merge": {
                "into": &conf.view_name,
                "on": "_id",
                "whenMatched": "replace",
                "whenNotMatched": "insert"
            }
        },
        ];
    
    let app_col = db.collection::<Document>("applications");

    app_col
        .aggregate(pipeline, None)
        .await
        .expect("Aggregation error");

    let mat_view = db.collection::<Document>(&conf.view_name);

    mat_view
        .create_indexes(vec![
            IndexModel::builder()
            .keys(doc! {"createdAt": -1})
            .build()
        ], None)
        .await
        .expect("Index creation error");

}

#[tokio::main]
async fn main() {
    create_materialized_view().await;
}
