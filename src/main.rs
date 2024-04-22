#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

mod models;
mod schema;
mod repositories;
mod rocket_routes;

#[rocket::main]
async fn main() {
    let figment = rocket::Config::figment().merge(("port", 8002));

    let _ = rocket::build()
        .configure(figment)
        .mount(
            "/",
            routes![
               rocket_routes::rustaceans::get_rustaceans,
               rocket_routes::rustaceans::view_rustaceans,
               rocket_routes::rustaceans::crate_rustaceans,
               rocket_routes::rustaceans::update_rustaceans,
               rocket_routes::rustaceans::delete_rustaceans
            ],
        )
        // .register("/", catchers![not_found])
        // .attach(DbConn::fairing())
        // .attach(AdHoc::on_ignite("Running DB migrations", run_db_migrations))
        .launch()
        .await;
}
