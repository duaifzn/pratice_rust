use crate::service::todo_service;
use rocket::serde::json::Json;
use crate::model::todo_model::Todo;

#[get("/hello1/<name>/<done>")]
pub async fn hello1(name: String, done: bool) -> String{
    //let data  = todo_service::create_one_todo().await.unwrap();
    format!("Hello, {} year old named {}!", name, done)
}

#[post("/todo", format = "json", data = "<todo>")]
pub async fn create_todo_one(todo: Json<Todo>) -> String {
    let data = todo_service::create_one_todo(todo).await.unwrap();
    data.inserted_id.to_string()
}
