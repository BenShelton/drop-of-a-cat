use dto::collections::User;
use mongodb::error::Error;

use crate::client::create_client;

pub async fn insert(user: User) -> Result<User, Error> {
    let client = create_client().await?;
    let collection = client.collection::<User>("users");
    collection.insert_one(&user).await?;
    Ok(user)
}
