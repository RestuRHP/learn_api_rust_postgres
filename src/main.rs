#[macro_use]
extern crate rocket;

mod auth;
mod db;
mod models;
mod routes;
mod schema;
mod utils;

use crate::db::init_pool;
use crate::routes::{
    user::api_create_user::create_user,
    user::api_update_user::update_user,
    user::api_get_all_user::get_all_users,
    user::api_get_user_by_id::get_user_by_id,
    user::api_login::login,
};
use dotenv::dotenv;
use rocket::{Build, Rocket};

#[launch]
fn rocket() -> Rocket<Build> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");

    rocket::build().manage(init_pool(&db_url))
       .manage(secret_key)
       .mount(
        "/",
        routes![
            login,
            get_all_users,
            get_user_by_id,
            create_user,
            update_user,
        ],
    )
}
