use std::io::Cursor;
use rocket::serde::{Serialize, Deserialize, json};
use rocket::serde::json::{Value, Json};
use rocket::response;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket::http::{ContentType};
use rocket_okapi::JsonSchema;
use crate::model::todo_model::TodoSchema;
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct ApiResponse<T>{
    pub success: bool,
    pub code: u16,
    pub json: Option<T>,
    pub message: Option<String>
}

impl<'r, T: Serialize> Responder<'r, 'static> for ApiResponse<T>{
    fn respond_to(self, req: &Request) -> response::Result<'static> {
        let string = json::serde_json::to_string(&self).unwrap();
        Response::build()
            .sized_body(None, Cursor::new(string))
            .header(ContentType::JSON)
            .ok()
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct CreateTodoDto{
    pub id: String
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct TodoSchemaDto {
    pub id: String,
    pub name: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct DeleteTodoDto{
    pub delete_count: u64
} 

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct UpdateTodoDto{
    pub update_count: u64
} 

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct SignDto{
    pub user_id: String
} 

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct TokenDto{
    pub token: String
} 