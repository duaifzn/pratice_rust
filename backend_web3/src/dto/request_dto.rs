use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserDto{
    pub email: String,
    pub password: String,
}