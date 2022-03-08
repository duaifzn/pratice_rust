use mongodb::bson::{doc, Document};
use crate::database::Mongo;
use crate::model::test_model;

pub async fn create_one_test(){
    let db = Mongo::connect().await;
    let todo = test_model::Test {
        name: "test".to_string(),
        done: "sss".to_string(),
    };
}