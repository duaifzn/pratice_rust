use std::io::Cursor;
use rocket::serde::{Serialize, Deserialize, json};
use rocket::response;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket::http::{ContentType};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiResponse<T>{
    pub success: bool,
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateOneUserDto{
    pub id: String,

}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SigninOneUserDto{
    pub token: String,
}