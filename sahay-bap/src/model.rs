use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub telegram_handle: String,
    pub otp: String,
    pub session_token:  String,
    pub verification_count: i32,
    pub is_verified: bool,
}

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub phone: &'a str,
    pub telegram_handle: &'a str,
    pub otp: String,
    pub session_token:  &'a str,
}
