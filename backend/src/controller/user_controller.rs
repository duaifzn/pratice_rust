use crate::{service::todo_service, database::Mongo};
use rocket::{serde::json::{Json, json}, State};
use crate::dto::response_dto::ApiResponse;
use crate::dto::request_dto::{TodoDto, UpdateTodoDto, UserDto};
use crate::service::user_service;
use crate::service::user_service::have_one_user;
use crate::util::auth::auth_token_generate;
use crate::middleware::auth_guard::Token;
use rocket_okapi::openapi;

#[openapi(tag="user")]
#[get("/hello1/<name>/<done>")]
pub async fn hello1(name: String, done: bool) -> String{
    //let data  = todo_service::create_one_todo().await.unwrap();
    format!("Hello, {} year old named {}!", name, done)
}

#[openapi(tag="user")]
#[post("/todo", format = "json", data = "<todo>")]
pub async fn create_todo(token: Token<'_>, db: &State<Mongo>, todo: Json<TodoDto>) -> Json<ApiResponse> {
    let data = todo_service::create_one_todo(db, todo).await;
     match data{
        Ok(result) => Json(ApiResponse{
            json: json!({
                "id": result.inserted_id.as_object_id().unwrap().to_string()
            })
        }),
        Err(err) => Json(ApiResponse{
            json: json!({
                "error": err.to_string()
            })
        }),
    }
}

#[openapi(tag="user")]
#[get("/todo/all")]
pub async fn find_todo(token: Token<'_>, db: &State<Mongo>) -> Json<ApiResponse>{
    let data = todo_service::find_all_todo(db).await;
    match data{
        Ok(result) => Json(ApiResponse{
            json: json!({
                "todos": result
            })
        }),
        Err(err) => Json(ApiResponse{
            json: json!({
                "error": err.to_string()
            })
        })
    }
}

#[openapi(tag="user")]
#[delete("/todo/<id>")]
pub async fn delete_todo(db: &State<Mongo>, id: String) -> Json<ApiResponse>{
    let data = todo_service::delete_one_todo(db, id).await;
    match data{
        Ok(result) => Json(ApiResponse{
            json: json!({
                "deleteCount": result.deleted_count
            })
        }),
        Err(err) => Json(ApiResponse{
            json: json!({
                "error": err.to_string()
            })
        }),
    }
}

#[openapi(tag="user")]
#[patch("/todo", format = "json", data = "<todo>")]
pub async fn update_todo(db: &State<Mongo>, todo: Json<UpdateTodoDto>) -> Json<ApiResponse> {
    let data = todo_service::update_one_todo(db, todo).await;
     match data{
        Ok(result) => Json(ApiResponse{
            json: json!({
                "id": result
            })
        }),
        Err(err) => Json(ApiResponse{
            json: json!({
                "error": err.to_string()
            })
        }),
    }
}

#[openapi(tag="user")]
#[post("/sign", format = "json", data = "<user>")]
pub async fn sign_user(db: &State<Mongo>, user: Json<UserDto>) ->Json<ApiResponse>{
    if have_one_user(db, &user).await == true{
        return Json(ApiResponse{
            json: json!({
                "error": "duplicate".to_string()
            })
        })
    }
    let data = user_service::insert_one_user(db, user).await;
    match data{
        Ok(result) => Json(ApiResponse{
            json: json!({
                "id": result
            })
        }),
        Err(err) => Json(ApiResponse{
            json: json!({
                "error": err.to_string()
            })
        })
    }
}

#[openapi(tag="user")]
#[post("/login", format = "json", data = "<user>")]
pub async fn login_user(db: &State<Mongo>, user: Json<UserDto>) ->Json<ApiResponse>{
    let data = user_service::verify_one_user(db, user).await;
    let token = auth_token_generate();
    match data{
        Ok(result) => Json(ApiResponse{
            json: json!({
                "verify": result,
                "token": token
            })
        }),
        Err(err) => Json(ApiResponse{
            json: json!({
                "error": err.to_string()
            })
        })
    }
}