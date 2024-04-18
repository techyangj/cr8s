use crate::schema::*;
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Rustaceans {
    id: i32,
    name: String,
    email: String,
    created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "rustaceans"]
pub struct NewRustacean {
    name: String,
    email: String,
}

#[derive(Queryable, Associations)]
pub struct Crate {
    id: i32,
    rustacean_id: i32,
    code: String,
    name: String,
    version: String,
    description: Option<String>,
    created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "crates"]
pub struct NewCrate {
    rustacean_id: i32,
    code: String,
    name: String,
    version: String,
    description: Option<String>,
}
