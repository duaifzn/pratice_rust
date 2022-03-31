use rocket::State;
use rocket::serde::json::Json;
use mongodb::error::Result;
use mongodb::results::InsertOneResult;
use mongodb::bson::doc;
use bcrypt::{DEFAULT_COST, hash, verify};
use crate::database::Mongo;
use crate::dto::request_dto::UserDto;
use crate::model::user_model::UserSchema;
use crate::dto::role::Role;

pub async fn have_one_user(db: &State<Mongo>, user: &Json<UserDto>) ->bool{
    let data = db.User.find_one(doc!{
        "email": user.email.to_string()
    }, None).await.unwrap();
    match data{
        Some(_) => true,
        None => false
    }

}

pub async fn insert_one_user(db: &State<Mongo>, user: Json<UserDto>) ->Result<InsertOneResult>{
    let hashed = hash(user.password.as_str(), DEFAULT_COST).unwrap();
    let new_user = UserSchema{
        id: None,
        email: user.email.to_string(),
        password: hashed,
        role: Role::User as u8,
    };
    let data = db.User.insert_one(new_user, None).await?;
    Ok(data)
}

pub async fn verify_one_user(db: &State<Mongo>, user: Json<UserDto>) ->Result<bool>{
    let old_user = db.User.find_one(doc!{
        "email": user.email.to_string()
    }, None).await?.unwrap();
    let valid = verify(user.password.as_str(), old_user.password.as_str()).unwrap();
    Ok(valid)
}