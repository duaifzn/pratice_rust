use rocket::serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TodoSchema {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub done: bool,
}