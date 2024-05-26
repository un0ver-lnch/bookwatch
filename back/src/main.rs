use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use futures::{lock::Mutex, TryStreamExt};
use mongodb::{bson::doc, options::ClientOptions, Client};
use serde::{Deserialize, Serialize};
use tokio::spawn;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Card {
    url: String,
    title: String,
    description: String,
}

#[derive(Clone)]
struct AppState {
    mongo_url: Arc<Mutex<String>>,
    cards: Arc<Mutex<Vec<Card>>>,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // Get credentials from the environment
    let username = std::env::var("DB_USER").unwrap();
    let password = std::env::var("DB_PASSWORD").unwrap();
    let host = std::env::var("DB_HOST").unwrap();

    let mongo_url = format!("mongodb://{}:{}@{}:27017", username, password, host);

    let app_state = AppState {
        mongo_url: Arc::new(Mutex::new(mongo_url)),
        cards: Arc::new(Mutex::new(vec![])),
    };

    let app_state_clone = app_state.clone();

    // build our application with a route
    let app = Router::new()
        .route("/api/cards", get(cards))
        .route("/api/cards", post(add_card))
        .with_state(app_state.clone());

    spawn(async move {
        let mongo_url = app_state_clone.mongo_url.lock().await;
        let client_options = match ClientOptions::parse(&*mongo_url).await {
            Ok(client_options) => client_options,
            Err(e) => {
                eprintln!("Failed to parse client options: {}", e);
                return;
            }
        };

        let client = match Client::with_options(client_options) {
            Ok(client) => client,
            Err(e) => {
                eprintln!("Failed to create client: {}", e);
                return;
            }
        };

        let db = client.database("bookwatch");

        let collection = db.collection::<Card>("cards");

        let mut cursor = match collection.find(doc! {}, None).await {
            Ok(cursor) => cursor,
            Err(e) => {
                eprintln!("Failed to find documents: {}", e);
                return;
            }
        };

        let mut card_count = 0;

        while let Some(_) = cursor.try_next().await.unwrap() {
            card_count += 1;
        }

        if card_count > 0 {
            println!("Found {} cards", card_count);
            return;
        }

        let card = Card {
            url: "https://example.com".to_string(),
            title: "Example".to_string(),
            description: "An example card".to_string(),
        };

        match collection.insert_one(card, None).await {
            Ok(_) => println!("Inserted a document"),
            Err(e) => eprintln!("Failed to insert document: {}", e),
        }
    });

    let app_state_clone_recurrent = app_state.clone();
    spawn(async move {
        let mongo_url = app_state_clone_recurrent.mongo_url.lock().await;
        let client_options = match ClientOptions::parse(&*mongo_url).await {
            Ok(client_options) => client_options,
            Err(e) => {
                eprintln!("Failed to parse client options: {}", e);
                return;
            }
        };

        let client = match Client::with_options(client_options) {
            Ok(client) => client,
            Err(e) => {
                eprintln!("Failed to create client: {}", e);
                return;
            }
        };

        let db = client.database("bookwatch");

        let collection = db.collection::<Card>("cards");
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            let mut new_cards: Vec<Card> = vec![];

            let mut cursor = match collection.find(doc! {}, None).await {
                Ok(cursor) => cursor,
                Err(e) => {
                    eprintln!("Failed to find documents: {}", e);
                    return;
                }
            };

            while let Some(doc) = cursor.try_next().await.unwrap() {
                new_cards.push(doc);
            }

            {
                let mut cards = app_state_clone.cards.lock().await;
                *cards = new_cards;
            }
        }
    });

    // run our app with hyper, listening globally on port 3001
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn cards(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Card>>), (StatusCode, String)> {
    let cards = state.cards.lock().await;
    Ok((StatusCode::OK, Json(cards.clone())))
}

async fn add_card(
    State(state): State<AppState>,
    Json(card): Json<Card>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mongo_url = state.mongo_url.lock().await;

    let client_options = match ClientOptions::parse(&*mongo_url).await {
        Ok(client_options) => client_options,
        Err(e) => {
            eprintln!("Failed to parse client options: {}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    };

    let client = match Client::with_options(client_options) {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to create client: {}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    };

    let db = client.database("bookwatch");

    let collection = db.collection::<Card>("cards");

    match collection.insert_one(card, None).await {
        Ok(_) => println!("Inserted a document"),
        Err(e) => eprintln!("Failed to insert document: {}", e),
    }
    println!("Added card");
    Ok(StatusCode::CREATED)
}
