use rocket::http::Status;
use rocket::serde::json::{serde_json::json, Value};
use rocket_sync_db_pools::database;

#[database("postgres")]
struct DbConn(diesel::PgConnection);


#[get("/rustaceans")]
async fn get_rustaceans(db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::find_multiple(c, 100)
            .map(|rustacean| json!(rustacean))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[get("/rustaceans/<id>")]
async fn view_rustaceans(id: i32, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::find(c, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}


#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn crate_rustaceans(
    db: DbConn,
    new_rustacean: Json<NewRustacean>,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::create(c, new_rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}


#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustaceans(
    id: i32,
    db: DbConn,
    rustacean: Json<Rustacean>,
) -> Result<Value, Custom<Value>> {
    // json!({"id": id, "name": "John", "email":"John@example.com"})
    db.run(move |c| {
        RustaceanRepository::save(c, id, rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustaceans(id: i32, db: DbConn) -> Result<status::NoContent, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::delete(c, id)
            .map(|_| status::NoContent)
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}
