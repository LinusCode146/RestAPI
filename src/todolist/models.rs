use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CreateEntryData {
    pub(crate) title: String,
    pub(crate) date: i64,
}
#[derive(Deserialize, Clone)]
pub struct UpdateEntryData {
    pub title: String
}