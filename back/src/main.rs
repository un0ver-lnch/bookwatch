use std::{sync::Arc, time::Duration};

use axum::{
    body::Bytes,
    extract::{MatchedPath, Path, State},
    http::{HeaderMap, Request, StatusCode},
    response::Response,
    routing::{delete, get, post},
    Json, Router,
};
use futures::{lock::Mutex, TryStreamExt};
use mongodb::{bson::doc, options::ClientOptions, Client, Collection};
use serde::{Deserialize, Serialize};
use tokio::{spawn, task::spawn_blocking};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Card {
    uuid: String,
    url: String,
    title: String,
    description: String,
}

#[derive(Clone)]
struct AppState {
    mongo_collection: Arc<Mutex<Collection<Card>>>,
    cards: Arc<Mutex<Vec<Card>>>,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Get credentials from the environment
    let username = std::env::var("DB_USER").unwrap();
    let password = std::env::var("DB_PASSWORD").unwrap();
    let host = std::env::var("DB_HOST").unwrap();

    let mongo_url = format!("mongodb://{}:{}@{}:27017", username, password, host);

    let client_options = match ClientOptions::parse(&mongo_url).await {
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

    let app_state = AppState {
        mongo_collection: Arc::new(Mutex::new(collection)),
        cards: Arc::new(Mutex::new(vec![])),
    };

    // build our application with a route
    let app = Router::new()
        .route("/api/cards", get(cards))
        .route("/api/card", post(add_card))
        .route("/api/card/:uuid", delete(delete_card))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // Log the matched route's path (with placeholders not filled in).
                    // Use request.uri() or OriginalUri if you want the real path.
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                    )
                })
                .on_request(|_request: &Request<_>, _span: &Span| {
                    // You can use `_span.record("some_other_field", value)` in one of these
                    // closures to attach a value to the initially empty field in the info_span
                    // created above.
                    println!("{} - {}", _request.method(), _request.uri());
                })
                .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                    // ...
                    println!(
                        "Response: {:?}, duration: {}",
                        _response.status(),
                        _latency.as_millis()
                    );
                })
                .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                    // ...
                })
                .on_eos(
                    |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                        // ...
                    },
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        // ...
                    },
                ),
        )
        .with_state(app_state.clone());

    let app_state_clone = app_state.clone();
    spawn(async move {
        let collection = app_state_clone.mongo_collection.lock().await;

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
            uuid: Uuid::new_v4().to_string(),
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
    spawn_blocking(move || async move {
        loop {
            let mut new_cards: Vec<Card> = vec![];
            {
                let collection = app_state_clone_recurrent.mongo_collection.lock().await;

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
            }

            {
                let mut cards = app_state_clone.cards.lock().await;
                *cards = new_cards;
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
        }
    });

    // run our app with hyper, listening globally on port 3001
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
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
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let uuid = Uuid::new_v4().to_string();
    // Modify the card to add an uuid
    let card = Card {
        uuid: (&uuid).to_string(),
        url: card.url,
        title: card.title,
        description: card.description,
    };

    let collection = state.mongo_collection.lock().await;

    match collection.insert_one(card, None).await {
        Ok(_) => println!("Inserted a document"),
        Err(e) => eprintln!("Failed to insert document: {}", e),
    }

    // Update the in-memory cards
    let mut new_cards: Vec<Card> = vec![];
    let mut cursor = match collection.find(doc! {}, None).await {
        Ok(cursor) => cursor,
        Err(e) => {
            eprintln!("Failed to find documents: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to find documents".to_string(),
            ));
        }
    };
    while let Some(doc) = cursor.try_next().await.unwrap() {
        new_cards.push(doc);
    }

    let mut cards = state.cards.lock().await;
    *cards = new_cards;

    Ok((StatusCode::CREATED, uuid))
}

async fn delete_card(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let collection = state.mongo_collection.lock().await;

    match collection.delete_one(doc! { "uuid": uuid }, None).await {
        Ok(_) => println!("Deleted a document"),
        Err(e) => {
            eprintln!("Failed to delete document: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to delete document".to_string(),
            ));
        }
    }

    // Update the in-memory cards
    let mut new_cards: Vec<Card> = vec![];
    let mut cursor = match collection.find(doc! {}, None).await {
        Ok(cursor) => cursor,
        Err(e) => {
            eprintln!("Failed to find documents: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to find documents".to_string(),
            ));
        }
    };
    while let Some(doc) = cursor.try_next().await.unwrap() {
        new_cards.push(doc);
    }

    let mut cards = state.cards.lock().await;
    *cards = new_cards;

    Ok(StatusCode::OK)
}
