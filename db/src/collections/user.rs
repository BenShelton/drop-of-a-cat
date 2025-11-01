use dto::collections::User;
use mongodb::{bson::doc, error::Error, Collection};

use crate::client::create_client;

async fn get_collection() -> Result<Collection<User>, Error> {
    let client = create_client().await?;
    Ok(client.collection::<User>("users"))
}

/// Inserts a new user into the database.
pub async fn insert(user: &User) -> Result<(), Error> {
    let collection = get_collection().await?;
    collection.insert_one(user).await?;
    Ok(())
}

/// Validates if a user exists in the database by their token.
pub async fn validate(token: &str) -> Result<bool, Error> {
    let collection = get_collection().await?;
    let count = collection.count_documents(doc! { "token": token }).await?;
    Ok(count > 0)
}
