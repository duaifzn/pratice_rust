use mongodb::{Client, options::ClientOptions, Database};
use mongodb::error::Result;

pub struct Mongo;

impl Mongo{
    pub async fn connect() -> Result<Database>{
        // Parse a connection string into an options struct.
        let mut client_options = ClientOptions::parse("mongodb://root:mongopassword@localhost:27023").await?;
    
        // Manually set an option.
        client_options.app_name = Some("MyApp".to_string());
    
        // Get a handle to the deployment.
        let client = Client::with_options(client_options)?;
        let db = client.database("herodotus-dev");
        // List the names of the databases in that deployment.
        // for db_name in client.list_database_names(None, None).await? {
        //     println!("{}", db_name);
        // }
        Ok(db)
    }
}