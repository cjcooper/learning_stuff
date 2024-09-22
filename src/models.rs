use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset, Selectable};
use crate::schema::books;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, AsChangeset, Selectable)]
#[diesel(table_name = books)]
pub struct Book {
    #[serde(default)]
    pub id: i32,
    pub title: String,
    pub primary_author: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable)]
#[diesel(table_name = books)]
pub struct NewBook {
    #[serde(default)]
    pub title: String,
    pub primary_author: String,
}