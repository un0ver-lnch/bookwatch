use axum::{http::StatusCode, routing::get, Json, Router};
use futures::TryStreamExt;
use mongodb::{bson::doc, options::ClientOptions, Client};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Card {
    url: String,
    title: String,
    description: String,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new().route("/api/cards", get(cards));

    // run our app with hyper, listening globally on port 3001
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn cards() -> Result<(StatusCode, Json<Vec<Card>>), (StatusCode, String)> {
    let cards = vec![
        Card {
            url: "https://example.com".to_string(),
            title: "Example".to_string(),
            description: "An example card".to_string(),
        },
        Card {
            url: "https://example.com".to_string(),
            title: "Example".to_string(),
            description: "An example card".to_string(),
        },
    ];

    let client_options = match ClientOptions::parse("mongodb://fun:fun@localhost:27017").await {
        Ok(client_options) => client_options,
        Err(e) => {
            eprintln!("Failed to parse client options: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to connect to mongo".to_string(),
            ));
        }
    };

    let client = match Client::with_options(client_options) {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to create client: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create the client".to_string(),
            ));
        }
    };

    let db = client.database("bookwatch");

    let collection = db.collection::<Card>("cards");

    let mut cursor = match collection.find(doc! {}, None).await {
        Ok(cursor) => cursor,
        Err(e) => {
            eprintln!("Failed to find documents: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create a cursor".to_string(),
            ));
        }
    };

    while let Some(doc) = cursor.try_next().await.unwrap() {
        println!("{:?}", doc);
    }

    Ok((StatusCode::OK, Json(cards)))
}
