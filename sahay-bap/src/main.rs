extern crate env_logger;
#[macro_use]
extern crate lazy_static;
extern crate log;

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::future;
use std::future::Future;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::Mutex;
use std::time::Instant;

use actix::{Actor, Addr, AsyncContext, Recipient, StreamHandler};
use actix_session::{Session, SessionMiddleware};
use actix_session::storage::CookieSessionStore;
// Dependencies
use actix_web::{App, cookie, HttpResponse, HttpServer, post, Responder, web};
use actix_web::cookie::Cookie;
use actix_web::dev::{Service, ServiceRequest};
use actix_web::http::header::Accept;
use actix_web::web::{Data, Json};
use actix_web_actors::ws;
use chrono::Utc;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel::sql_types::Uuid;
use futures::TryFutureExt;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, TokenData, Validation};
use log::{debug, error, info};
use rand::Rng;
use reqwest::{Client, StatusCode};
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::{to_string, Value};

use sahay_bap::model::{NewUser, User};
use sahay_bap::schema::users;

use crate::server::ChatServer;


mod server;
mod session;

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

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SelectRequest {
    bpp_uri: String,
    transaction_id: String,
    message_id: String,
    item_id: String
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct InitRequest {
    bpp_uri: String,
    transaction_id: String,
    mentorship_title: String,
    message_id: String,
    item_id: String,
    fullfillment_id: String,
    card: String,
    email_id: String,
    name: String
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
    srv: web::Data<Addr<server::ChatServer>>,
) -> impl Responder {
    info!("On Search API called {:?}", to_string(&on_search_request));
    let payload = format!("{:?}", to_string(&on_search_request));
    srv.do_send(server::OnSearch{
        id: 1,
        payload: to_string(&on_search_request).unwrap()
    });
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
// #[derive(Debug)]
// pub struct WsSession {
//     /// unique session id
//     pub id: usize,
//     // pub addr: Addr<MessageServer>,
//     pub srv : MessageServer,
// }

//
// impl Actor for WsSession {
//     type Context = ws::WebsocketContext<Self>;
//     fn started(&mut self, ctx: &mut Self::Context) {
//         let addr:Addr<MessageServer> = ctx.address();
//         let recipient:Recipient<WsMessage> = addr.recipient();
//         self.register(1, recipient);
//     }
// }

// /// Handler for ws::Message message
// impl StreamHandler<Result<actix_web_actors::ws::Message, ws::ProtocolError>> for WsSession {
//     fn handle(&mut self, msg: Result<actix_web_actors::ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
//         match msg {
//             Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
//             Ok(ws::Message::Text(text)) => ctx.text(text),
//             Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
//             _ => (),
//         }
//     }
// }
//
// impl actix::Handler<WsMessage> for WsSession {
//     type Result = ();
//
//     fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) -> Self::Result {
//         todo!()
//     }
// }

/// Entry point for our websocket route
async fn chat_route(
    req: actix_web::HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
) -> Result<HttpResponse, actix_web::Error> {
    info!("got ws request: {:?}", req);
    let result = ws::start(
        session::WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "main".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    );
    info!("after ws request: {:?}", req);
    result
}

/*
async fn ws_notification_handler(
    req: actix_web::HttpRequest,
    stream: web::Payload,
    session: Session,
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
    message_server: web::Data<MessageServer>,
) -> Result<HttpResponse, actix_web::Error>  {
    println!("got ws request: {:?}", req);
    let srv : WsSession = WsSession{ id: 1, srv: *message_server.get_ref() };
    let resp = ws::start(srv, &req, stream);
    resp
}*/


async fn on_confirm(
    db_pool: web::Data<DbPool>,
    on_status_request: web::Json<DSEPSearchRequest>,
    srv: web::Data<Addr<server::ChatServer>>,
) -> impl Responder {
    info!("On Confirm API called {:?}", to_string(&on_status_request));
    issue_credentials(&on_status_request, srv.clone()).await;
    return on_search(db_pool, on_status_request, srv).await
}

async fn get_certificate_pdf(
    db_pool: web::Data<DbPool>,
    path: web::Path<String>,
    srv: web::Data<Addr<server::ChatServer>>,
) -> impl Responder {
    let certificate_id = path.into_inner();
    info!("On Confirm API called: {}", certificate_id);
    let url =  env::var("REGISTRY_URL").unwrap_or("https://sahaay.xiv.in/registry/api/v1/ProofOfAssociation".to_string());
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("Accept", "application/pdf".parse().unwrap());
    headers.insert("template-key", "mentor".parse().unwrap());
    let response = client.get(format!("{}/{}", url, certificate_id))
        .headers(headers)
        .send()
        .await;

    match response {
        Ok(data) => {
            let x1 = data.bytes().await;
            match x1 {
                Ok(x) => {
                    HttpResponse::Ok().append_header(("Content-Type", "application/pdf")).body(x)
                }
                _ => {HttpResponse::Ok().body("")    }
            }


        }
        Err(_) => {HttpResponse::Ok().body("") }
    }

}

async fn issue_credentials (on_confirm_request: &Json<DSEPSearchRequest>, srv: Data<Addr<ChatServer>>) -> Result<(), Box<dyn std::error::Error>> {
    let map = USERMAP.lock().unwrap();
    let transaction_id = on_confirm_request.context.as_ref().unwrap().transaction_id.as_ref().unwrap();
    let user_data = map.get(transaction_id);
    let url =  env::var("REGISTRY_URL").unwrap_or("http://localhost:8081/api/v1/ProofOfAssociation".to_string());
    let json = format!(r#"{{ "name": "{}", "userId": "{}", "emailId": "{}", "type": "{}",
    "associatedFor": "{}", "agentName": "{}", "startDate": "{}", "endDate": "{}" }}"#, user_data.as_ref().unwrap().name, user_data.as_ref().unwrap().transactionId, user_data.as_ref().unwrap().emailId,
                       on_confirm_request.context.as_ref().unwrap().domain.as_ref().unwrap(),
                       user_data.as_ref().unwrap().mentorshipTitle,
                       on_confirm_request.message.as_ref().unwrap().order.as_ref().unwrap().fulfillments.as_ref().unwrap()[0].agent.as_ref().unwrap().person.as_ref().unwrap().name.as_ref().unwrap(),
                       on_confirm_request.message.as_ref().unwrap().order.as_ref().unwrap().fulfillments.as_ref().unwrap()[0].time.as_ref().unwrap().range.as_ref().unwrap().start.as_ref().unwrap(),
                       on_confirm_request.message.as_ref().unwrap().order.as_ref().unwrap().fulfillments.as_ref().unwrap()[0].time.as_ref().unwrap().range.as_ref().unwrap().end.as_ref().unwrap());
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    let response = client.post(url)
        .headers(headers)
        .body(json)
        .send()
        .await?;
    let status = response.status();
    let body = response.text().await?;
    info!("Response Status: {}", status);
    info!("Response Body: {}", body);
    srv.do_send(server::OnSearch{
        id: 1,
        payload: body
    });
    Ok(())
}

async fn select(
    db_pool: web::Data<DbPool>,
    select_request: web::Json<SelectRequest>,
) -> impl Responder {
    info!("Select API called {:?}", to_string(&select_request));
    let url =  format!("{}/select", select_request.bpp_uri);
    let now = Utc::now();
    let message_id = Option::from(String::from(&select_request.message_id));
    let transaction_id = Option::from(String::from(&select_request.transaction_id));
    let body = format!(r#"{{
    "context": {{
        "domain": "dsep:mentoring",
        "action": "select",
        "bap_id": "https://sahaay.xiv.in/bap",
        "bap_uri": "https://sahaay.xiv.in/bap",
        "timestamp": "2023-02-25T10:07:42.877Z",
        "message_id": "{}",
        "version": "1.0.0",
        "ttl": "PT10M",
        "transaction_id": "{}"
    }},
    "message": {{
        "order": {{
            "item": {{
                "id": "{}"
            }}
        }}
    }}
}}"#, select_request.message_id, select_request.transaction_id, select_request.item_id);
    /*let request_body = DSEPSearchRequest {
        context: Option::from(Context {
            domain: Option::from(String::from("dsep:mentoring")),
            action: Option::from(String::from("select")),
            bap_id: Option::from(String::from("https://sahaay.xiv.in/bap")),
            bap_uri: Option::from(String::from("https://sahaay.xiv.in/bap")),
            bpp_id: None,
            bpp_uri: None,
            timestamp: Option::from(String::from(now.to_rfc3339())),
            message_id: message_id.clone(),
            version: Option::from(String::from("1.0.0")),
            ttl: Option::from(String::from("PT10M")),
            transaction_id: transaction_id.clone(),
        }),
        message: Option::from(Message {
            catalog: None,
            intent: None,
            order: Option::from(Order {
                id: None,
                state: None,
                r#type: None,
                provider: None,
                items: Option::from(vec![Item {
                    quantity: None,
                    price: None,
                    id: Option::from(String::from(&select_request.item_id)),
                    category_ids: None,
                    descriptor: None,
                    fulfillment_ids: None,
                    tags: None
                }]),
                fulfillments: None
            })
        }),
    };*/
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );

    let response = client
        .post(url)
        .headers(headers)
        .body(body)
        .send()
        .await;

    HttpResponse::Ok().json(SearchResponse {
        message_id: message_id.unwrap().clone(),
        transaction_id: transaction_id.unwrap().clone()
    })
}
async fn init(
    db_pool: web::Data<DbPool>,
    init_request: web::Json<InitRequest>,
) -> impl Responder {
    info!("Init API called {:?}", to_string(&init_request));
    let url =  format!("{}/init", init_request.bpp_uri);
    let now = Utc::now();
    let message_id = Option::from(String::from(&init_request.message_id));
    let transaction_id = Option::from(String::from(&init_request.transaction_id));
    let body = format!(r#"{{
    "context": {{
        "domain": "dsep:mentoring",
        "action": "init",
        "bap_id": "https://sahaay.xiv.in/bap",
        "bap_uri": "https://sahaay.xiv.in/bap",
        "timestamp": "2023-02-25T10:07:42.877Z",
        "message_id": "{}",
        "version": "1.0.0",
        "ttl": "PT10M",
        "transaction_id": "{}"
    }},
    "message": {{
        "order": {{
            "items": [{{
                "id": "{}"
            }}],
            "fulfillments": [{{
                "id": "{}"
            }}],
            "billing": {{
                "card": "{}",
                "name": "{}",
                "phone": "881-311-2951 x01508",
                "email": "{}",
                "time": {{
                    "timezone": "IST"
                }}
            }}
        }}
    }}
}}"#, init_request.message_id, init_request.transaction_id, init_request.item_id, init_request.fullfillment_id, init_request.card, init_request.name, init_request.email_id);
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );

    let response = client
        .post(url)
        .headers(headers)
        .body(body)
        .send()
        .await;

    HttpResponse::Ok().json(SearchResponse {
        message_id: message_id.unwrap().clone(),
        transaction_id: transaction_id.unwrap().clone()
    })
}
struct UserData {
    name: String,
    emailId: String,
    messageId: String,
    transactionId: String,
    mentorshipTitle: String,
}
lazy_static! {
    static ref USERMAP: Mutex<HashMap<String, UserData>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

async fn confirm(
    db_pool: web::Data<DbPool>,
    init_request: web::Json<InitRequest>,
) -> impl Responder {
    info!("Confirm API called {:?}", to_string(&init_request));
    let mut map = USERMAP.lock().unwrap();
    map.insert(init_request.transaction_id.clone(), UserData{
        name: init_request.name.to_string(),
        emailId: init_request.email_id.to_string(),
        messageId: init_request.message_id.to_string(),
        transactionId: init_request.transaction_id.to_string(),
        mentorshipTitle: init_request.mentorship_title.to_string()
    });
    let url =  format!("{}/confirm", init_request.bpp_uri);
    let now = Utc::now();
    let message_id = Option::from(String::from(&init_request.message_id));
    let transaction_id = Option::from(String::from(&init_request.transaction_id));
    let body = format!(r#"{{
    "context": {{
        "domain": "dsep:mentoring",
        "action": "confirm",
        "bap_id": "https://sahaay.xiv.in/bap",
        "bap_uri": "https://sahaay.xiv.in/bap",
        "timestamp": "2023-02-25T10:07:42.877Z",
        "message_id": "{}",
        "version": "1.0.0",
        "ttl": "PT10M",
        "transaction_id": "{}"
    }},
    "message": {{
        "order": {{
            "items": [{{
                "id": "{}"
            }}],
            "fulfillments": [{{
                "id": "{}"
            }}],
            "billing": {{
                "card": "{}",
                "name": "{}",
                "phone": "881-311-2951 x01508",
                "email": "{}",
                "time": {{
                    "timezone": "IST"
                }}
            }}
        }}
    }}
}}"#, init_request.message_id, init_request.transaction_id, init_request.item_id, init_request.fullfillment_id, init_request.card, init_request.name, init_request.email_id);
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );

    let response = client
        .post(url)
        .headers(headers)
        .body(body)
        .send()
        .await;

    HttpResponse::Ok().json(SearchResponse {
        message_id: message_id.unwrap().clone(),
        transaction_id: transaction_id.unwrap().clone()
    })
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
    // start chat server actor
    let app_state = Arc::new(AtomicUsize::new(0));
    let server = server::ChatServer::new(app_state.clone()).start();

    // Set up the Actix Web server and register the routes
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(server.clone()))
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
                .route("/on_status", web::post().to(on_search))
                .route("/on_init", web::post().to(on_search))
                .route("/on_confirm", web::post().to(on_confirm))
                .route("/on_cancel", web::post().to(on_search))
                .route("/search", web::post().to(search))
                .route("/select", web::post().to(select))
                .route("/init", web::post().to(init))
                .route("/confirm", web::post().to(confirm))
                .route("/health", web::get().to(health_check))
                .route("/pdf/{certificate_id}", web::get().to(get_certificate_pdf))
                .route("/ws", web::get().to(chat_route))
            )
    })
        .bind("0.0.0.0:6080")?
        .run()
        .await
}
