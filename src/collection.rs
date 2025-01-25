use crate::UiSchemaInputError;
use serde::{de::DeserializeOwned, Serialize};
use uuid::Uuid;

pub trait UiSchemaCollections
where
    Self: DeserializeOwned + Serialize + std::fmt::Display,
{
    type Error: std::fmt::Display;

    #[allow(async_fn_in_trait)]
    async fn resolve(
        &self,
        env_id: Uuid,
        id: &str,
        property: &str,
    ) -> Result<serde_json::Value, UiSchemaInputError<Self::Error>>;
}
