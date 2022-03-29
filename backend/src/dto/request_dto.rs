use rocket::serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TodoDto{
    pub name: String,
    pub done: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateTodoDto{
    pub id: String,
    pub name: String,
    pub done: bool,
}