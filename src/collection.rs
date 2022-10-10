use crate::UiSchemaInputError;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
#[cfg(feature = "uuid1")]
use uuid::Uuid;
#[cfg(feature = "uuid08")]
use uuid08::Uuid;

#[async_trait]
pub trait UiSchemaCollections
where
    Self: DeserializeOwned + Serialize + std::fmt::Display,
{
    type Error: std::fmt::Display;

    async fn resolve(
        &self,
        env_id: Uuid,
        id: &str,
        property: &str,
    ) -> Result<serde_json::Value, UiSchemaInputError<Self::Error>>;
}
