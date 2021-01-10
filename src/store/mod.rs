pub mod guild;

use std::{collections::HashMap, hash::Hash};
use bson::Bson;
use serde::{Serialize, Deserialize};
use mongodb::{
    bson::{self, doc},
    error::Result,
    options::FindOneAndReplaceOptions,
};
use serenity::prelude::*;

use crate::db::DbClient;

/// A type that can be stored in a Store
pub trait StoreItem: Default + Send + Sync + Serialize + for<'de> Deserialize<'de> {
    /// The key type to use to identify items of this type
    type Key: StoreItemKey;
}

/// A type that can be used as a database document id
pub trait StoreItemKey: Copy + Eq + Hash + Send + Sync {
    /// The effective id type stored in the database
    type DocumentId: Serialize + Into<Bson>;
    /// Returns the effective id to store in the database
    fn doc_id(&self) -> Self::DocumentId;
}

/// Manages permanent bot configuration like guild-scoped settings, etc.
pub struct Store<T: StoreItem> {
    db_client: DbClient,
    collection_name: String,
    cache: HashMap<T::Key, T>,
}

impl<T: StoreItem> Store<T> {
    pub fn new(db_client: DbClient, collection_name: impl Into<String>) -> Self {
        Self {
            db_client,
            collection_name: collection_name.into(),
            cache: HashMap::new(),
        }
    }

    /// Returns an immutable reference to the item identified by the given key.
    /// Fetches the item from the database, if not present in the local cache.
    pub async fn get(&mut self, id: T::Key) -> Result<&T> {
        if !self.cache.contains_key(&id) {

            // Fetch settings from database
            let opt = self.db_client.database()
                .collection(&self.collection_name)
                .find_one(doc! { "_id": id.doc_id().into() }, None)
                .await?;

            if let Some(doc) = opt {

                // Save to local cache
                self.cache.insert(id, bson::from_document(doc)
                    .expect("Could not deserialize document"));

            } else {

                // Insert default into both database and local cache
                let item = T::default();

                self.db_client.database()
                    .collection(&self.collection_name)
                    .insert_one(to_document(id.doc_id(), &item), None)
                    .await?;

                self.cache.insert(id, item);
            }
        }

        Ok(&self.cache[&id])
    }

    /// Calls the given callback with a mutable reference to the item identified
    /// by the given key and persists it to the database afterwards
    pub async fn with_mut<F>(&mut self, id: T::Key, f: F) -> Result<()>
    where
        F: FnOnce(&mut T) -> ()
    {
        // Insert if not exists
        if !self.cache.contains_key(&id) {
            self.cache.insert(id, Default::default());
        }

        // Pass to callback
        let item = self.cache.get_mut(&id).unwrap();
        f(item);

        // Persist to database
        self.db_client.database()
            .collection(&self.collection_name)
            .find_one_and_replace(
                doc! { "_id": id.doc_id().into() },
                to_document(id.doc_id(), item),
                FindOneAndReplaceOptions::builder()
                    .upsert(true)
                    .build())
            .await?;

        Ok(())
    }
}

impl<T: StoreItem + 'static> TypeMapKey for Store<T> {
    type Value = Self;
}

/// Converts the given value to a BSON document and attaches an `_id` attribute
/// with the specified id
fn to_document<T, U>(id: T, u: &U) -> bson::Document
where
    T: Into<bson::Bson>,
    U: Serialize,
{
    let mut doc = bson::to_document(u)
        .expect("Could not serialize value into document");

    doc.insert("_id", id);
    doc
}
