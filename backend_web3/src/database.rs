use crate::model::user_model::UserSchema;
use mongodb::{Client, options::ClientOptions};
use mongodb::Collection;
use crate::config::Config;
pub struct Mongo{
    pub User: Collection<UserSchema>,
}

impl Mongo{
    pub async fn connect() -> Self{
        let config = Config::load();
        let mongo_server = format!("mongodb://{}:{}@{}", config.mongodb_username, config.mongodb_password, config.mongodb_address);
        // Parse a connection string into an options struct.
        let mut client_options = ClientOptions::parse(mongo_server).await.unwrap();
    
        // Manually set an option.
        client_options.app_name = Some("MyApp".to_string());
    
        // Get a handle to the deployment.
        let client = Client::with_options(client_options).unwrap();
        let db = client.database(config.mongodb_database_name);
        //List the names of the databases in that deployment.
        // for db_name in client.list_database_names(None, None).await.unwrap() {
        //     println!("{}", db_name);
        // }
        Mongo{
            User: db.collection::<UserSchema>("users"),
        }
    }
}