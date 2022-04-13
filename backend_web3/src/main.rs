#[macro_use] extern crate rocket;

mod controller;
mod model;
mod dto;
mod database;
mod service;
mod util;
mod config;
mod middleware;

#[launch]
async fn rocket() -> _{
    rocket::build()
        .mount("/api", routes![
            controller::user_controller::index,
            controller::user_controller::signup_one_user,
            controller::user_controller::signin_one_user,
        ])
        .manage(database::Mongo::connect().await)
}
