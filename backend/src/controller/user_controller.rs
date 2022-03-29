use crate::{service::todo_service, database::Mongo};
use rocket::{serde::json::{Json, json}, State};
use crate::dto::response_dto::ApiResponse;
use crate::dto::request_dto::{TodoDto, UpdateTodoDto};

#[get("/hello1/<name>/<done>")]
pub async fn hello1(name: String, done: bool) -> String{
    //let data  = todo_service::create_one_todo().await.unwrap();
    format!("Hello, {} year old named {}!", name, done)
}

#[post("/todo", format = "json", data = "<todo>")]
pub async fn create_todo(db: &State<Mongo>, todo: Json<TodoDto>) -> ApiResponse {
    let data = todo_service::create_one_todo(db, todo).await;
     match data{
        Ok(result) => ApiResponse{
            json: json!({
                "id": result.inserted_id.as_object_id().unwrap().to_string()
            })
        },
        Err(err) => ApiResponse{
            json: json!({
                "error": err.to_string()
            })
        },
    }
}

#[get("/todo/all")]
pub async fn find_todo(db: &State<Mongo>) -> ApiResponse{
    let data = todo_service::find_all_todo(db).await;
    match data{
        Ok(result) => ApiResponse{
            json: json!({
                "todos": result
            })
        },
        Err(err) => ApiResponse{
            json: json!({
                "error": err.to_string()
            })
        }
    }
}

#[delete("/todo/<id>")]
pub async fn delete_todo(db: &State<Mongo>, id: String) -> ApiResponse{
    let data = todo_service::delete_one_todo(db, id).await;
    match data{
        Ok(result) => ApiResponse{
            json: json!({
                "deleteCount": result.deleted_count
            })
        },
        Err(err) => ApiResponse{
            json: json!({
                "error": err.to_string()
            })
        },
    }
}

#[patch("/todo", format = "json", data = "<todo>")]
pub async fn update_todo(db: &State<Mongo>, todo: Json<UpdateTodoDto>) -> ApiResponse {
    let data = todo_service::update_one_todo(db, todo).await;
     match data{
        Ok(result) => ApiResponse{
            json: json!({
                "id": result
            })
        },
        Err(err) => ApiResponse{
            json: json!({
                "error": err.to_string()
            })
        },
    }
}