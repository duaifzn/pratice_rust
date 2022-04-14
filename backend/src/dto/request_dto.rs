use rocket::serde::{Serialize, Deserialize};
use rocket_okapi::JsonSchema;
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct TodoDto{
    pub name: String,
    pub done: bool,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct UpdateOneTodoDto{
    pub id: String,
    pub name: String,
    pub done: bool,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct UserDto{
    pub email: String,
    pub password: String,
}