use rocket::serde::{Serialize, Deserialize, Serializer};
use rocket::serde::json::serde_json;
use mongodb::bson::oid::ObjectId;
use crate::dto::response_dto::TodoSchemaDto;

pub fn serialize_object_id<S>(object_id: &Option<ObjectId>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match object_id {
      Some(ref object_id) => serializer.serialize_some(object_id.to_string().as_str()),
      None => serializer.serialize_none()
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TodoSchema {
    #[serde(rename(serialize = "id", deserialize = "_id"),
    skip_serializing_if = "Option::is_none",
    serialize_with = "serialize_object_id")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub done: bool,
}

impl TodoSchema{
    pub fn okapi_dto(&self) -> TodoSchemaDto{
        let serialized = serde_json::to_string(self).unwrap();
        serde_json::from_str(&serialized).unwrap()
    }
}