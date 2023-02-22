use serde::{Deserialize, Serialize};
use std::sync::Arc;
use warp::{http::StatusCode, Filter, Rejection, Reply};

#[derive(Deserialize)]
struct Context {
    domain: String,
    country: String,
    state: String,
    city: String,
}

#[derive(Deserialize)]
struct IntentQueryLocation {
    lat: f64,
    long: f64,
}

#[derive(Deserialize)]
struct IntentQuery {
    text: String,
    category: String,
    location: Option<IntentQueryLocation>,
    distance: Option<f64>,
}

#[derive(Deserialize)]
struct Intent {
    // #[serde(rename = "type")]
    descriptor: Descriptor,
}

#[derive(Deserialize)]
struct SearchRequest {
    context: Context,
    message: Intent,
}

#[derive(Serialize,Deserialize)]
struct Descriptor {
    id: String,
    name: String,
    code: String,
}

#[derive(Serialize)]
struct Address {
    building: String,
    street: String,
    locality: String,
    city: String,
    state: String,
    country: String,
    pincode: String,
}

#[derive(Serialize)]
struct Contact {
    phone: String,
    email: Option<String>,
}

#[derive(Serialize)]
struct ItemUnit {
    id: String,
    descriptor: String,
}

#[derive(Serialize)]
struct Item {
    id: String,
    name: String,
    descriptor: String,
    image: String,
    category: String,
    price: f64,
    currency: String,
    tax: f64,
    unit: Option<ItemUnit>,
}

#[derive(Serialize)]
struct Catalog {
    provider: Descriptor,
    items: Vec<Item>,
}

#[derive(Serialize)]
struct SearchResponse {
    catalog: Catalog,
}

#[derive(Serialize)]
struct Error {
    message: String,
    code: String,
}

async fn search_handler(request: SearchRequest) -> Result<impl Reply, Rejection> {
    // Here you would perform the actual search based on the `request` parameter,
    // and return a `SearchResponse` or an `Error` in case of failure.

    let provider = Descriptor {
        id: "12345".to_string(),
        name: "Acme Inc.".to_string(),
        code: "23".to_string(),
    };

    let item_unit = ItemUnit {
        id: "each".to_string(),
        descriptor: "Each".to_string(),
    };

    let item = Item {
        id: "67890".to_string(),
        name: "Widget".to_string(),
        descriptor: "A useful widget".to_string(),
        image: "http://example.com/widget.jpg".to_string(),
        category: "Gadgets".to_string(),
        price: 10.0,
        currency: "USD".to_string(),
        tax: 1.0,
        unit: Some(item_unit),
    };

    let catalog = Catalog {
        provider,
        items: vec![item],
    };

    let response = SearchResponse { catalog };

    Ok(warp::reply::json(&response))
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    // if let Some(_) = err.find::warp::filters::body::BodyDeserializeError() {
    //     return Ok(warp::reply::with_status(
    //         "Invalid request body",
    //         StatusCode::BAD_REQUEST,
    //     ));
    // }
    println!("Error: {:?}", err);
    Ok(warp::reply::with_status(
        "Internal server error",
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

#[tokio::main]
async fn main() {
    let search_route = warp::post()
        .and(warp::path("search"))
        .and(warp::body::json())
        .and_then(search_handler);
    let routes = search_route.recover(handle_rejection).with(warp::log("api"));

    let port = 7080;

    println!("Listening on port {}", port);

    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}

