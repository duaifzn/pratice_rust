#![feature(proc_macro_hygiene, decl_macro)]



#[macro_use] extern crate rocket;
//use rocket::routes;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
mod controller;
mod database;
mod model;
mod service;
mod dto;
mod middleware;
mod util;
mod config;

#[launch]
async fn rocket() -> _ {
    rocket::build()
    .mount("/user", openapi_get_routes![
        controller::user_controller::hello1,
        controller::user_controller::create_todo,
        controller::user_controller::delete_todo,
        controller::user_controller::find_todo,
        controller::user_controller::update_todo,
        controller::user_controller::sign_user,
        controller::user_controller::login_user
        ])
    .mount("/admin", openapi_get_routes![
        controller::admin_controller::hello])
    .mount(
        "/swagger-ui/",
        make_swagger_ui(&SwaggerUIConfig {
            urls: vec![
                UrlObject::new("user", "../user/openapi.json"),
                UrlObject::new("admin", "../admin/openapi.json")
            ],
            ..Default::default()
        }),
    )
    .manage(database::Mongo::connect().await)
}