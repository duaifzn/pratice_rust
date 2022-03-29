#![feature(proc_macro_hygiene, decl_macro)]



#[macro_use] extern crate rocket;
use rocket::routes;

mod controller;
mod database;
mod model;
mod service;
mod dto;

#[launch]
async fn rocket() -> _ {
    rocket::build()
    .mount("/user", routes![
        controller::user_controller::hello1,
        controller::user_controller::create_todo,
        controller::user_controller::delete_todo,
        controller::user_controller::find_todo,
        controller::user_controller::update_todo,
        controller::user_controller::sign_user,
        controller::user_controller::login_user])
    .mount("/admin", routes![
        controller::admin_controller::hello])
    .manage(database::Mongo::connect().await)
}