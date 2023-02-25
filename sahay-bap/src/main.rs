use std::error::Error;
// Dependencies
use actix_web::{App, HttpResponse, HttpServer, post, Responder, web};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool};
use log::{error, info};
use rand::Rng;
use reqwest::{Client, StatusCode};
use sahay_bap::schema::users;
use sahay_bap::model::{User, NewUser};
use serde::{Deserialize, Serialize};

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
#[serde(rename_all = "camelCase")]
struct Context {
    domain: String,
    action: String,
}

#[derive(Serialize)]
struct Catalog {
    provider: Descriptor,
    items: Vec<Item>,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Message {
    catalog: Catalog,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OnSearchRequest {
    context: Context,
    message: Message,
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

async fn on_search(
    db_pool: web::Data<DbPool>,
    user: web::Json<OnSearchRequest>,
) -> impl Responder {

}

// #[post("/api/verify")]
async fn user_signin(
    db_pool: web::Data<DbPool>,
    user: web::Json<UserSigninRequest>,
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
// Define the API routes for mentorship search
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = "postgres://postgres@localhost/sahay";
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager).unwrap();

    // Set up the Actix Web server and register the routes
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/api")
                .route("/register", web::post().to(user_register))
                .route("/verify", web::post().to(user_signin))
                .route("/on_search", web::post().to(on_search))
            )
    })
        .bind("127.0.0.1:6080")?
        .run()
        .await
}