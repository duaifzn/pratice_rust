use mongodb::results::{InsertOneResult};
use mongodb::error::Result;
use rocket::serde::json::{Json};
use crate::database::Mongo;
use crate::model::todo_model::Todo;

pub async fn create_one_todo(todo: Json<Todo>) -> Result<InsertOneResult>{
    let db = Mongo::connect().await?;
    let data = db.collection::<Todo>("todos")
        .insert_one(todo.into_inner(), None).await?;
    Ok(data)
}