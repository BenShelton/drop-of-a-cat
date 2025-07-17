use dotenvy_macro::dotenv;
use mongodb::Client;

pub async fn create_client() -> Result<mongodb::Database, mongodb::error::Error> {
    let uri = dotenv!("MONGODB_URI");
    let client = Client::with_uri_str(uri).await?;
    let database_name = dotenv!("MONGODB_DATABASE");
    let database = client.database(database_name);
    Ok(database)
}
