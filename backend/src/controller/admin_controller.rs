#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello, admion outside world!"
}