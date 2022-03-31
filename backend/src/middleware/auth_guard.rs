use rocket::http::Status;
use rocket::request::{self, Outcome, Request, FromRequest};
use crate::util::auth::auth_token_is_valid;

pub struct Token<'r>(&'r str);

#[derive(Debug)]
pub enum TokenError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token<'r> {
    type Error = TokenError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("Authorization") {
            None => Outcome::Failure((Status::BadRequest, TokenError::Missing)),
            Some(token) if auth_token_is_valid(token) => Outcome::Success(Token(token)),
            Some(_) => Outcome::Failure((Status::BadRequest, TokenError::Invalid)),
        }
    }
}