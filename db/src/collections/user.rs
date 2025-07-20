use dto::collections::User;
use mongodb::{error::Error, Collection};

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
