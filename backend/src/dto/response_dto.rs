use rocket::serde::json::Value;
use rocket::response;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket::http::{ContentType};
pub struct ApiResponse{
    pub json: Value
}

impl<'r> Responder<'r, 'static> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'static> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .header(ContentType::JSON)
            .ok()
    }
}
// #[derive(Responder)]
// #[response(status = 500, content_type = "json")]
// pub struct ApiError(pub String);