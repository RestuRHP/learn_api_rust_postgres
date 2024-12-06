#[macro_use]
extern crate rocket;

mod db;
mod models;
mod routes;
mod schema;

use crate::db::init_pool;
use crate::routes::user::{create_user, delete_user, get_all_users, get_user_by_id, update_user};
use rocket::{Build, Rocket};
use dotenv::dotenv;

#[launch]
fn rocket() -> Rocket<Build> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    rocket::build()
        .manage(init_pool(&db_url))
        .mount(
            "/",
            routes![
                get_all_users,
                get_user_by_id,
                create_user,
                update_user,
                delete_user
            ],
        )
}
