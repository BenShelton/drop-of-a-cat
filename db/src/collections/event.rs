use dto::collections::Event;
use futures::TryStreamExt;
use mongodb::{bson::doc, error::Error, Collection};
use uuid::Uuid;

use crate::client::create_client;

async fn get_collection() -> Result<Collection<Event>, Error> {
    let client = create_client().await?;
    Ok(client.collection::<Event>("events"))
}

/// Inserts a new event into the database and returns its UUID.
pub async fn insert(event: &mut Event) -> Result<String, Error> {
    let collection = get_collection().await?;
    let uuid = Uuid::new_v4().to_string();
    event.uuid = uuid.clone();
    event.accepted = false;
    event.accepted_by = None;
    collection.insert_one(event).await?;
    Ok(uuid)
}

/// Lists all events in the database.
pub async fn list() -> Result<Vec<Event>, Error> {
    let collection = get_collection().await?;
    let cursor = collection.find(doc! {}).await?;
    let events: Vec<Event> = cursor.try_collect().await?;

    Ok(events)
}

/// Finds an event by its UUID.
pub async fn find_by_id(uuid: &String) -> Result<Option<Event>, Error> {
    let collection = get_collection().await?;
    let filter = doc! { "uuid": uuid };
    let event = collection.find_one(filter).await?;
    Ok(event)
}
