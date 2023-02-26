use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, Filter, Rejection, Reply};

#[derive(Serialize, Deserialize)]
struct Context {
    domain: String,
    action: String,
    bap_id: String,
    bap_uri: String,
    bpp_id: String,
    bpp_uri: String,
    timestamp: String,
    ttl: String,
    version: String,
    message_id: String,
    transaction_id: String,
}

#[derive(Serialize, Deserialize)]
struct IntentQueryLocation {
    lat: f64,
    long: f64,
}

#[derive(Serialize, Deserialize)]
struct IntentQuery {
    text: String,
    category: String,
    location: Option<IntentQueryLocation>,
    distance: Option<f64>,
}

#[derive(Serialize, Deserialize)]
struct Intent {
    descriptor: Descriptor,
}

#[derive(Serialize, Deserialize)]
struct DSEPMessage {
    catalog: Catalog,
    intent: Option<Intent>,
}

#[derive(Serialize, Deserialize)]
struct SearchRequest {
    context: Context,
    message: DSEPMessage,
}

#[derive(Serialize, Deserialize)]
struct Address {
    building: String,
    street: String,
    locality: String,
    city: String,
    state: String,
    country: String,
    pincode: String,
}

#[derive(Serialize, Deserialize)]
struct Contact {
    phone: String,
    email: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ItemUnit {
    id: String,
    descriptor: String,
}

#[derive(Serialize, Deserialize)]
struct CountValue {
    count: i64,
}

#[derive(Serialize, Deserialize)]
struct PriceValue {
    value: String,
}

#[derive(Serialize, Deserialize)]
struct Quantity {
    available: CountValue,
    allocated: CountValue,
}

#[derive(Serialize, Deserialize)]
struct Tag {
    display: bool,
    discriptor: Descriptor,
    code: String,
    name: String,
    list: Vec<Descriptor>,
}

#[derive(Serialize, Deserialize)]
struct Item {
    quantity: Quantity,
    price: PriceValue,
    id: String,
    category_ids: Vec<String>,
    descriptor: Descriptor,
    fulfillment_ids: Vec<String>,
    tags: Vec<Tag>,
}

#[derive(Serialize, Deserialize)]
struct Descriptor {
    code: String,
    name: String,
    short_desc: String,
    long_desc: String,
    images: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Category {
    id: String,
    discriminator: Descriptor,
}

#[derive(Serialize, Deserialize)]
struct TimeRange {
    start: String,
    end: String,
}

#[derive(Serialize, Deserialize)]
struct TimeRangeWithLabel {
    range: TimeRange,
    label: String,
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    id: String,
}

#[derive(Serialize, Deserialize)]
struct Agent {
    person: Person,
}

#[derive(Serialize, Deserialize)]
struct Fulfillment {
    language: Vec<String>,
    id: String,
    time: TimeRangeWithLabel,
    #[serde(rename = "type")]
    fulfillment_type: String,
    tags: Vec<Tag>,
    agent: Agent,
}

#[derive(Serialize, Deserialize)]
struct Provider {
    id: String,
    categories: Vec<Category>,
    discriminator: Descriptor,
    items: Vec<Item>,
    fulfillments: Vec<Fulfillment>,
}

#[derive(Serialize, Deserialize)]
struct Catalog {
    providers: Vec<Provider>,
}

#[derive(Serialize, Deserialize)]
struct SearchResponse {
    context: Context,
    message: DSEPMessage,
}

#[derive(Serialize, Deserialize)]
struct Error {
    message: String,
    code: String,
}

async fn search_handler(_: SearchRequest) -> Result<impl Reply, Rejection> {
    // Here you would perform the actual search based on the `request` parameter,
    // and return a `SearchResponse` or an `Error` in case of failure

    let response = SearchResponse {
        context: Context {
            domain: "dsep:mentoring".to_string(),
            action: "on_search".to_string(),
            bap_id: "https://sahaay.xiv.in/bap".to_string(),
            bap_uri: "https://sahaay.xiv.in/bap".to_string(),
            bpp_id: "https//sahaay.xiv.in/bpp".to_string(),
            bpp_uri: "https//sahaay.xiv.in/bpp".to_string(),
            timestamp: "2023-02-26T04:39:58.316Z".to_string(),
            ttl: "PT10M".to_string(),
            version: "1.0.0".to_string(),
            message_id: "d3a075b2-a5f6-4d01-9e09-2fb356e1a12c".to_string(),
            transaction_id: "ebb98d27-1b73-4d20-acdc-dd8bef91de24".to_string(),
        },
        message: DSEPMessage {
            catalog: Catalog {
                providers: vec![Provider {
                    id: "63d103e62d52ec96cf85efa5".to_string(),
                    categories: vec![Category {
                        id: "63d103e62d52ec96cf85efa5".to_string(),
                        discriminator: Descriptor {
                            code: "4229d975-43ae-e560-3ed5-439283800560".to_string(),
                            name: "Master of science".to_string(),
                            short_desc: "".to_string(),
                            long_desc: "".to_string(),
                            images: vec![],
                        },
                    }],
                    discriminator: Descriptor {
                        code: "sahaj".to_string(),
                        name: "Sahaj Gurukul".to_string(),
                        short_desc: "Sahaj Gurukul internship program".to_string(),
                        long_desc: "Sahaj Gurukul internship program, accelerated, intense learning program".to_string(),
                        images: vec![],
                    },
                    items: vec![Item {
                        quantity: Quantity { available: CountValue { count: 5 }, allocated: CountValue { count: 10 } },
                        price: PriceValue { value: "∞".to_string() },
                        id: "63fa069b23df0828569386ea".to_string(),
                        category_ids: vec!["4229d975-43ae-e560-3ed5-439283800560".to_string()],
                        descriptor: Descriptor {
                            code: "mentorship_for_solution_consultant".to_string(),
                            name: "Mentorship for Solution Consultant".to_string(),
                            short_desc: "Mentorship for Solution Consultant for qualified students".to_string(),
                            long_desc: "Mentorship for Solution Consultant for qualified students, applicable with terms as mentioned on the application form".to_string(),
                            images: vec![],
                        },
                        fulfillment_ids: vec![],
                        tags: vec![],
                    }],
                    fulfillments: vec![Fulfillment {
                        language: vec!["English".to_string()],
                        id: "90f0d8db-5b5b-4d25-847f-5383ab627dcb".to_string(),
                        time: TimeRangeWithLabel {
                            range: TimeRange {
                                start: "2023-02-25T18:59:00".to_string(),
                                end: "2023-02-26T17:59:00".to_string(),
                            },
                            label: "Session Timing".to_string(),
                        },
                        fulfillment_type: "ONLINE".to_string(),
                        tags: vec![Tag {
                            display: true,
                            discriptor: Descriptor {
                                code: "timeZone".to_string(),
                                name: "timeZone".to_string(),
                                short_desc: "".to_string(),
                                long_desc: "".to_string(),
                                images: vec![],
                            },
                            code: "timeZone".to_string(),
                            name: "timeZone".to_string(),
                            list: vec![Descriptor {
                                code: "Asia/Calcutta".to_string(),
                                name: "Asia/Calcutta".to_string(),
                                short_desc: "".to_string(),
                                long_desc: "".to_string(),
                                images: vec![],
                            }],
                        }],
                        agent: Agent { person: Person { name: "Tejash".to_string(), id: "63fa05f362820fd9e6beb82a".to_string() } },
                    }],
                }, ],
            },
            intent: Option::None,
        },
    };

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

