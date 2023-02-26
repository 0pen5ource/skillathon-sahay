use std::env;
use std::error::Error;
use std::future;
use std::future::Future;
// Dependencies
use actix_web::{App, cookie, HttpResponse, HttpServer, post, Responder, web};
use actix_web::dev::{Service, ServiceRequest};
use chrono::Utc;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel::sql_types::Uuid;
use log::{debug, error, info};
use serde_json::{to_string, Value};

extern crate log;
extern crate env_logger;

use rand::Rng;
use reqwest::{Client, StatusCode};
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use sahay_bap::schema::users;
use sahay_bap::model::{User, NewUser};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, TokenData, Algorithm};
use actix_session::{Session, SessionMiddleware};
use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Cookie;
use futures::TryFutureExt;
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;


// Database connection pool
type DbPool = Pool<ConnectionManager<PgConnection>>;


pub fn generate_otp() -> String {
    let mut rng = rand::thread_rng();
    let otp: u16 = rng.gen_range(1000..=9999);
    otp.to_string()
}
// Request structs
#[derive(Debug, Serialize, Deserialize)]
struct UserRegisterRequest {
    name: String,
    email: String,
    phone: String,
    telegram: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserSigninRequest {
    otp: String,
    session_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MentorshipSearchRequest {
    query: String,
}

// Response structs
#[derive(Debug, Serialize, Deserialize)]
struct UserRegisterResponse {
    status: String,
    message: String,
    #[serde(rename(serialize = "sessionToken"))]
    session_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserSigninResponse {
    status: String,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MentorshipSearchResponse {
    mentors: Vec<String>,
}


#[derive(Deserialize, Debug, Serialize)]
struct DSEPSearchRequest {
    context: Option<Context>,
    message: Option<Message>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchRequest {
    session_title: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Intent {
    item: Option<Item>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Context {
    domain: Option<String>,
    action: Option<String>,
    bap_id: Option<String>,
    bap_uri: Option<String>,
    bpp_id: Option<String>,
    bpp_uri: Option<String>,
    timestamp: Option<String>,
    ttl: Option<String>,
    version: Option<String>,
    message_id: Option<String>,
    transaction_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    catalog: Option<Catalog>,
    intent: Option<Intent>,
    order: Option<Order>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Order {
    id: Option<String>,
    state: Option<String>,
    r#type: Option<String>,
    provider: Option<Provider>,
    items: Option<Vec<Item>>,
    fulfillments: Option<Vec<Fulfillment>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Catalog {
    providers: Option<Vec<Provider>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Provider {
    id: Option<String>,
    categories: Option<Vec<Category>>,
    descriptor: Option<Descriptor>,
    items: Option<Vec<Item>>,
    fulfillments: Option<Vec<Fulfillment>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Fulfillment {
    language: Option<Vec<String>>,
    id: Option<String>,
    time: Option<Time>,
    r#type: Option<String>,
    tags: Option<Vec<Tag>>,
    agent: Option<Agent>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Person {
    name: Option<String>,
    id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Agent {
    person: Option<Person>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Time {
    range: Option<Range>,
    label: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Range {
    start: Option<String>,
    end: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Category {
    id: Option<String>,
    descriptor: Option<Descriptor>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Descriptor {
    code: Option<String>,
    name: Option<String>,
    short_desc: Option<String>,
    long_desc: Option<String>,
    images: Option<Vec<Image>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Image {
    url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Item {
    quantity: Option<Quantity>,
    price: Option<Price>,
    id: Option<String>,
    category_ids: Option<Vec<String>>,
    descriptor: Option<Descriptor>,
    fulfillment_ids: Option<Vec<String>>,
    tags: Option<Vec<Tag>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Quantity {
    available: Option<Count>,
    allocated: Option<Count>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Count {
    count: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Price {
    value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Tag {
    display: Option<bool>,
    descriptor: Option<Descriptor>,
    code: Option<String>,
    name: Option<String>,
    list: Option<Vec<List>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct List {
    descriptor: Option<Descriptor>,
    code: Option<String>,
    name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Ack {
    status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ResponseMessage {
    ack: Option<Ack>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ResponseError {
    #[serde(rename = "type")]
    error_type: Option<String>,
    code: Option<String>,
    path: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    message: Option<ResponseMessage>,
    error: Option<ResponseError>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SearchResponse {
    message_id: String,
    transaction_id: String
}

async fn send_otp_to_telegram(telegram_handle: &str, otp: &str, bot_token: &str) -> Result<(), reqwest::Error> {
    // Construct the message to be sent to Telegram
    let text = format!("Your OTP code is {}", otp);
    let chat_id = telegram_handle;
    let api_url = format!("https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}", bot_token, chat_id, text);

    // Send the message to Telegram using the Bot API
    let client = Client::new();
    let res = client.post(&api_url).send().await?;
    // let status = res.status();

    // Check the response status and return a result indicating success or failure
    Ok(())
    /*if status.is_success() { //todo check error
        Ok(())
    } else {
        Err(reqwest::Error::from(std::error::Error::from(status)))
    }*/
}
// API endpoints
// #[post("/register")]
async fn user_register(
    db_pool: web::Data<DbPool>,
    user: web::Json<UserRegisterRequest>,
) -> impl Responder {
    // Generate OTP and send to user's Telegram handle using Telegram API
    let bot_token= "1234567890:ABCDEFabcdef1234567890abcdef1234567890";
    let otp = generate_otp();


    // Store user info and OTP in database
    let mut conn = db_pool.get().unwrap();
    let session_token = uuid::Uuid::new_v4().to_string();
    let new_user = NewUser {
        name: &user.name,
        email: &user.email,
        phone: &user.phone,
        telegram_handle: &user.telegram,
        otp: otp.clone(),
        session_token: session_token.as_str()
    };
    let res = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn);
    match res {
        Ok(_) => info!("User registered successfully"),
        Err(e) => error!("Error registering user: {}", e),
    }

    send_otp_to_telegram(&user.telegram, &otp, bot_token);

    // Return success response
    HttpResponse::Ok().json(UserRegisterResponse {
        status: "success".to_string(),
        message: "Registration successful. Please check your Telegram for OTP".to_string(),
        session_token,
    })
}

async fn health_check( db_pool: web::Data<DbPool>) -> impl Responder {
    info!("Health API called");
    HttpResponse::Ok().json(Ack {
        status: Option::from("UP".to_string())
    })
}

async fn on_search(
    db_pool: web::Data<DbPool>,
    on_search_request: web::Json<DSEPSearchRequest>,
) -> impl Responder {
    info!("On Search API called {:?}", to_string(&on_search_request));
    HttpResponse::Ok().json(Response {
        message: Option::from(ResponseMessage { ack: Option::from(Ack { status: Option::from("ACK".to_string()) }) }),
        error: Option::from(ResponseError {
            error_type: Option::from("".to_string()),
            code: Option::from("".to_string()),
            path: Option::from("".to_string()),
            message: Option::from("".to_string())
        })
    })
}
async fn search(
    db_pool: web::Data<DbPool>,
    search_request: web::Json<SearchRequest>,
) -> impl Responder {
    info!("On Search API called {:?}", to_string(&search_request));
    let url =  env::var("GATEWAY_URL").unwrap_or("https://gateway.becknprotocol.io/bg/search".to_string());
    let now = Utc::now();
    let message_id: String = uuid::Uuid::new_v4().to_string();
    let transaction_id: String = uuid::Uuid::new_v4().to_string();
    let request_body = DSEPSearchRequest {
        context: Option::from(Context {
            domain: Option::from(String::from("dsep:mentoring")),
            action: Option::from(String::from("search")),
            bap_id: Option::from(String::from("https://sahaay.xiv.in/bap")),
            bap_uri: Option::from(String::from("https://sahaay.xiv.in/bap")),
            bpp_id: None,
            bpp_uri: None,
            timestamp: Option::from(String::from(now.to_rfc3339())),
            message_id: Option::from(String::from(&message_id)),
            version: Option::from(String::from("1.0.0")),
            ttl: Option::from(String::from("PT10M")),
            transaction_id: Option::from(String::from(&transaction_id)),
        }),
        message: Option::from(Message {
            catalog: None,
            intent: Some(Intent {
                item: Option::from(Item {
                    quantity: None,
                    price: None,
                    id: None,
                    category_ids: None,
                    descriptor: Some(Descriptor {
                        code: None,
                        name: Option::from(search_request.session_title.to_string()),
                        short_desc: None,
                        long_desc: None,
                        images: None
                    }),
                    fulfillment_ids: None,
                    tags: None
                }),
            }),
            order: None
        }),
    };
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );

    let response = client
        .post(url)
        .headers(headers)
        .json(&request_body)
        .send()
        .await;

    // info!("Gateway search response: {:?}", response);
    match response {
        Ok(v) => {
            info!("Gateway search response: {:?}", v.json::<Response>().await)
        }
        Err(e) => {
            error!("Gateway search error: {:?}", e)
        }
    }
    HttpResponse::Ok().json(SearchResponse {
        message_id,
        transaction_id
    })
}

// #[post("/api/verify")]
async fn user_signin(
    db_pool: web::Data<DbPool>,
    user: web::Json<UserSigninRequest>,
    session: Session
) -> impl Responder {
    println!("{:?}", user);
    // Retrieve user info from database
    let mut conn = &mut db_pool.get().unwrap();
    let results = users::table
        .filter(users::session_token.eq(&user.session_token))
        .load::<User>(conn)
        .unwrap();

    // Verify OTP and activate account
    if results.len() == 1 {
        let db_user = &results[0];
        if db_user.otp == user.otp && db_user.otp != "" {
            diesel::update(users::table.find(db_user.id))
                // .set(users::is_verified.eq(true))
                .set(users::otp.eq(""))
                .execute(conn)
                .unwrap();
            session.insert("token", signed_token(db_user)).unwrap();
            HttpResponse::Ok().json(UserRegisterResponse {
                status: "success".to_string(),
                message: "Account activated successfully".to_string(),
                session_token: "".to_string(),
            })
        } else {
            if db_user.verification_count< 3 {
                diesel::update(users::table.find(db_user.id))
                    .set(users::verification_count.eq(db_user.verification_count + 1))
                    .execute(conn)
                    .unwrap();
                HttpResponse::Ok().json(UserSigninResponse {
                    status: "error".to_string(),
                    message: "Error: OTP".to_string() })
            } else {
                diesel::update(users::table.find(db_user.id))
                    .set((users::is_verified.eq(false), users::otp.eq("")))
                    .execute(conn)
                    .unwrap();
                HttpResponse::Ok().json(UserSigninResponse { status: "error".to_string(), message: "Error: Regenerate OTP".to_string() })
            }
        }
    } else {
        HttpResponse::Ok().json(UserSigninResponse {
            status: "error".to_string(),
            message: "Error: OTP".to_string(),
        })
    }
}

fn signed_token(users: &User) -> String {
    "sahay".to_string()
}

/*
// Define the API routes for user registration and login
#[post("/api/register")]
async fn register(
    db: DbPool,
    req: Json<RegisterRequest>,
) -> Result<Json<UserRegisterResponse>, Status> {
    // Validate the user input
    let errors = req.validate();
    if !errors.is_empty() {
        return Err(Status::new(Code::InvalidArgument, errors.join(", ")));
    }

    // Check if the user already exists in the database
    let conn = db.get().await.map_err(|e| {
        println!("Failed to get DB connection: {}", e);
        Status::new(Code::Unavailable, "Failed to get DB connection")
    })?;
    let user_exists = conn
        .try_query_by_email(&req.email)
        .await
        .map_err(|e| {
            println!("Failed to query user by email: {}", e);
            Status::new(Code::Internal, "Failed to query user by email")
        })?
        .is_some();
    if user_exists {
        return Err(Status::new(
            Code::AlreadyExists,
            format!("User with email {} already exists", &req.email),
        ));
    }

    // Generate and send the OTP to the user's Telegram handle
    let otp = generate_otp();
    let message = format!("Your OTP is: {}", otp);
    send_otp_to_telegram(&req.telegram_handle, &otp, &req.telegram_bot_token).await.map_err(|e| {
        println!("Failed to send OTP to Telegram: {}", e);
        Status::new(Code::Internal, "Failed to send OTP to Telegram")
    })?;

    // Save the user details and OTP in the database
    let user = User::from_register_request(&req, &otp);
    conn.try_insert_user(&user).await.map_err(|e| {
        println!("Failed to insert user: {}", e);
        Status::new(Code::Internal, "Failed to insert user")
    })?;

    Ok(Json(RegisterResponse {
        message: "OTP sent successfully".to_string(),
    }))
} */
/*
#[post("/api/verify")]
async fn verify(
    db: DbPool,
    req: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, Status> {
    // Validate the user input
    let errors = req.validate();
    if !errors.is_empty() {
        return Err(Status::new(Code::InvalidArgument, errors.join(", ")));
    }

    // Get the user details from the database
    let conn = db.get().await.map_err(|e| {
        println!("Failed to get DB connection: {}", e);
        Status::new(Code::Unavailable, "Failed to get DB connection")
    })?;
    let user = conn
        .try_query_by_email(&req.email)
        .await.map_err(|e| {
        println!("Failed to query user by email: {}", e);
        Status::new(Code::Internal, "Failed to query user by email")
    })?
        .ok_or_else(|| {
            Status::new(
                Code::NotFound,
                format!("User with email {} not found", &req.email),
            )
        })?;

    // Verify the OTP sent to the user's Telegram handle
    if !verify_otp(&user.otp, &req.otp) {
        return Err(Status::new(
            Code::InvalidArgument,
            "Invalid OTP. Please try again".to_string(),
        ));
    }

    // Generate and return the user access token
    let token = generate_access_token(&user.id);
    Ok(Json(LoginResponse { access_token: token }))
}
*/

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
   sub: String,
   company: String
}
/*
pub fn jwt_auth_middleware(
    req: ServiceRequest,
    credentials: DecodingKey,
) -> impl Future<Output = Result<ServiceRequest, dyn Error>> {
    let auth_header = req.headers().get("Authorization");

    match auth_header {
        Some(header_value) => {
            let bearer_token = header_value.to_str().unwrap().replace("Bearer ", "");

            let token = match jsonwebtoken::decode::<Claims>(
                &bearer_token,
                &credentials,
                &Validation::default()
            ) {
                Ok(token) => token,
                Err(_) => {
                    return Box::pin(futures::future::err(HttpResponse::Unauthorized().finish(), ));
                },
            };
            req.extensions_mut().insert(token.claims);
        }
        None => {
            return Box::pin(futures::future::err(HttpResponse::Unauthorized().finish()));
        }
    }

    Box::pin(futures::future::ok(req))
}

 */
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn ws_notification_handler(
    req: actix_web::HttpRequest,
    stream: web::Payload,
    session: Session,
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, actix_web::Error>  {
    println!("got ws request: {:?}", req);
    let resp = ws::start(MyWs {}, &req, stream);
    resp
}

// Define the API routes for mentorship search
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let database_url =  env::var("DATABASE_URL").unwrap_or("postgres://postgres:postgres@localhost/sahay".to_string());
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager).unwrap();

    let key = b"234234a";
    let encoding_key = EncodingKey::from_secret(key);
    let decoding_key = DecodingKey::from_secret(key);

    // Set up the Actix Web server and register the routes
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(
                // create cookie based session middleware
                SessionMiddleware::builder(CookieSessionStore::default(), cookie::Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .build()
            )
            .wrap_fn(move |req, srv| {
               // jwt_auth_middleware(req, decoding_key.clone())
               //     .map(|req| srv.call(req))
               //     .map_err(|e| e.into())
                srv.call(req)
            })
            .service(web::scope("/api")
                .route("/register", web::post().to(user_register))
                .route("/verify", web::post().to(user_signin))
                .route("/on_search", web::post().to(on_search))
                .route("/on_select", web::post().to(on_search))
                .route("/on_confirm", web::post().to(on_search))
                .route("/on_init", web::post().to(on_search))
                .route("/on_status", web::post().to(on_search))
                .route("/on_cancel", web::post().to(on_search))
                .route("/search", web::post().to(search))
                .route("/health", web::get().to(health_check))
                .route("/ws", web::get().to(ws_notification_handler))
            )
    })
        .bind("0.0.0.0:6080")?
        .run()
        .await
}
