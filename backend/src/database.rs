use crate::model::todo_model::TodoSchema;
use crate::model::test_model::TestSchema;
use mongodb::{Client, options::ClientOptions};
use mongodb::Collection;

pub struct Mongo{
    pub todo: Collection<TodoSchema>,
    pub test: Collection<TestSchema>,
}

impl Mongo{
    pub async fn connect() -> Self{
        // Parse a connection string into an options struct.
        let mut client_options = ClientOptions::parse("mongodb://root:mongopassword@localhost:27023").await.unwrap();
    
        // Manually set an option.
        client_options.app_name = Some("MyApp".to_string());
    
        // Get a handle to the deployment.
        let client = Client::with_options(client_options).unwrap();
        let db = client.database("herodotus-dev");
        //List the names of the databases in that deployment.
        // for db_name in client.list_database_names(None, None).await.unwrap() {
        //     println!("{}", db_name);
        // }
        Mongo{
            todo: db.collection::<TodoSchema>("todos"),
            test: db.collection::<TestSchema>("tests"),
        }
    }
}