use crate::schema::*;
use chrono::NaiveDateTime;

#[derive(diesel::Queryable)]
pub struct Rustaceans {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(diesel::Insertable)]
#[table_name = "rustaceans"]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}

#[derive(diesel::Queryable)]
pub struct Crate {
    pub id: i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub descruption: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "crates"]
pub struct NewCrate {
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub descruption: Option<String>,
}
