use rocket_okapi::openapi;

#[openapi(tag="admin")]
#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello, admion outside world!"
}