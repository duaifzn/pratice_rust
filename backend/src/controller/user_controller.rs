use crate::{service::todo_service, database::Mongo, dto::response_dto::TodoSchemaDto};
use rocket::{serde::json::{Json, json}, State};
use crate::dto::response_dto::{ApiResponse, CreateTodoDto, DeleteTodoDto, SignDto, TokenDto, UpdateTodoDto};
use crate::dto::request_dto::{TodoDto, UpdateOneTodoDto, UserDto};
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
pub async fn create_todo(token: Token<'_>, db: &State<Mongo>, todo: Json<TodoDto>) -> Json<ApiResponse<CreateTodoDto>>{
    let data = todo_service::create_one_todo(db, todo).await;
     match data{
        Ok(result) => Json(ApiResponse{
            success: true,
            code: 200,
            json: Some(CreateTodoDto{
                    id: result.inserted_id.as_object_id().unwrap().to_string()
                }),
            message: None,
        }),
        Err(err) => Json(ApiResponse{
            success: false,
            code: 500,
            json: None,
            message: Some(err.to_string()),
        })
    }
}

#[openapi(tag="user")]
#[get("/todo/all")]
pub async fn find_todo(db: &State<Mongo>) -> Json<ApiResponse<Vec<TodoSchemaDto>>>{
    let data = todo_service::find_all_todo(db).await;
    match data{
        Ok(result) => {
            let new_result = result.into_iter()
                .map(|todo| 
                    todo.okapi_dto())
                .collect();
            Json(ApiResponse{
                success: true,
                code: 200,
                json: Some(new_result),
                message: None,
            })
        },
        Err(err) => Json(ApiResponse{
            success: true,
            code: 500,
            json: None,
            message: Some(err.to_string()),
        })
    }
}

#[openapi(tag="user")]
#[delete("/todo/<id>")]
pub async fn delete_todo(db: &State<Mongo>, id: String) -> Json<ApiResponse<DeleteTodoDto>>{
    let data = todo_service::delete_one_todo(db, id).await;
    match data{
        Ok(result) => Json(ApiResponse{
            success: true,
            code: 200,
            json: Some(DeleteTodoDto{
                delete_count: result.deleted_count
            }),
            message: None,
        }),
        Err(err) => Json(ApiResponse{
            success: false,
            code: 500,
            json: None,
            message: Some(err.to_string()),
        })
    }
}

#[openapi(tag="user")]
#[patch("/todo", format = "json", data = "<todo>")]
pub async fn update_todo(db: &State<Mongo>, todo: Json<UpdateOneTodoDto>) -> Json<ApiResponse<UpdateTodoDto>> {
    let data = todo_service::update_one_todo(db, todo).await;
    match data{
        Ok(result) => Json(ApiResponse{
            success: true,
            code: 200,
            json: Some(UpdateTodoDto{
                update_count: result.modified_count
            }),
            message: None,
        }),
        Err(err) => Json(ApiResponse{
            success: false,
            code: 500,
            json: None,
            message: Some(err.to_string()),
        })
    }
}

#[openapi(tag="user")]
#[post("/sign", format = "json", data = "<user>")]
pub async fn sign_user(db: &State<Mongo>, user: Json<UserDto>) ->Json<ApiResponse<SignDto>>{
    if have_one_user(db, &user).await == true{
        return Json(ApiResponse{
            success: false,
            code: 401,
            json: None,
            message: Some("duplicate user!".to_string()),
        })
    }
    let data = user_service::insert_one_user(db, user).await;
    match data{
        Ok(result) => Json(ApiResponse{
            success: true,
            code: 200,
            json: Some(SignDto{
                user_id: result.inserted_id.as_object_id().unwrap().to_string()
            }),
            message: None,
        }),
        Err(err) => Json(ApiResponse{
            success: false,
            code: 500,
            json: None,
            message: Some(err.to_string()),
        })
    }
}

#[openapi(tag="user")]
#[post("/login", format = "json", data = "<user>")]
pub async fn login_user(db: &State<Mongo>, user: Json<UserDto>) ->Json<ApiResponse<TokenDto>>{
    let data = user_service::verify_one_user(db, user).await;
    let token = auth_token_generate();
    match data{
        Ok(result) => {
            if result{
                return Json(ApiResponse{
                    success: true,
                    code: 200,
                    json: Some(TokenDto{
                        token: token
                    }),
                    message: None,
                })
            }
            else{
                return Json(ApiResponse{
                    success: false,
                    code: 401,
                    json: None,
                    message: None,
                })
            }
            
        },
        Err(err) => Json(ApiResponse{
            success: false,
            code: 500,
            json: None,
            message: Some(err.to_string()),
        })
    }
}