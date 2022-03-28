use crate::{service::todo_service, database::Mongo};
use rocket::{serde::json::{Json, json}, State};
use crate::model::todo_model::TodoSchema;
use crate::dto::response_dto::ApiResponse;

#[get("/hello1/<name>/<done>")]
pub async fn hello1(name: String, done: bool) -> String{
    //let data  = todo_service::create_one_todo().await.unwrap();
    format!("Hello, {} year old named {}!", name, done)
}

#[post("/todo", format = "json", data = "<todo>")]
pub async fn create_todo_one(db: &State<Mongo>, todo: Json<TodoSchema>) -> ApiResponse {
    let data = todo_service::create_one_todo(db, todo).await;
     match data {
        Ok(result) => ApiResponse{
            json: json!({
                "id": result.inserted_id.as_object_id().unwrap().to_string()
            })
        },
        Err(result) => ApiResponse{
            json: json!({
                "error": result.to_string()
            })
        },
    }
}

// #[get("/todo/<id>")]
// pub async fn find_one_todo(id: &str) -> String{
    
// }
