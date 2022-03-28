use mongodb::bson::doc;
use mongodb::results::{InsertOneResult};
use mongodb::error::Result;
use rocket::State;
use rocket::serde::json::{Json};
use crate::database::Mongo;
use crate::model::todo_model::TodoSchema;

pub async fn create_one_todo(db: &State<Mongo>, todo: Json<TodoSchema>) ->Result<InsertOneResult>{
    let data = db.todo.insert_one(todo.into_inner(), None).await?;
    Ok(data)
}

// pub async fn find_one_todo(id: &str) ->Result<TodoSchema>{
//     let db = Mongo::connect().await?;
//     let data = db.collection::<Todo>("todos")
//         .find_one(doc! {
//             "_id": id.to_string()
//       }, None).await?.unwrap();
//     Ok(data)
// }