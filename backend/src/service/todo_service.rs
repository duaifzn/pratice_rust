use mongodb::{bson, bson::doc};
use mongodb::bson::oid::ObjectId;
use mongodb::results::{InsertOneResult, DeleteResult, UpdateResult};
use mongodb::error::Result;
use rocket::State;
use rocket::futures::{TryStreamExt};
use rocket::serde::json::{Json};
use crate::database::Mongo;
use crate::model::todo_model::TodoSchema;
use crate::dto::request_dto::{TodoDto, UpdateTodoDto};

pub async fn create_one_todo(db: &State<Mongo>, todo: Json<TodoDto>) ->Result<InsertOneResult>{
    let new_todo = TodoSchema{
        id: None,
        name: todo.name.to_string(),
        done: todo.done,
    };
    let data = db.Todo.insert_one(new_todo, None).await?;
    Ok(data)
}

pub async fn delete_one_todo(db: &State<Mongo>, id: String) ->Result<DeleteResult>{
    let object_id = ObjectId::parse_str(id).unwrap();
    let data = db.Todo.delete_one(doc!{"_id": object_id}, None).await?;
    Ok(data)
}

pub async fn find_all_todo(db: &State<Mongo>) ->Result<Vec<TodoSchema>>{
    let cursor = db.Todo.find(None, None).await?;
    let data = cursor.try_collect::<Vec<TodoSchema>>().await?;
    Ok(data)
}

pub async fn update_one_todo(db: &State<Mongo>, todo: Json<UpdateTodoDto>) ->Result<UpdateResult>{
    let data = db.Todo.update_one(
        doc!{
            "_id": todo.id.to_string()
        }, 
        doc!{
            "$set":{
                "name": todo.name.to_string(),
                "done": todo.done,
            }
        }, None).await?;
    Ok(data)
}