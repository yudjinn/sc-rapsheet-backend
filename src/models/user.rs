use diesel::prelude::*;
use rocket::serde::Serialize;
use schemars::JsonSchema;

pub mod schema {
    rocket_sync_db_pools::diesel::table! {
        users {
            id -> Nullable<Integer>,
            password -> Varchar,
            name -> Varchar,
            active -> Bool,
        }
    }
}

#[derive(Serialize, Queryable, Debug, Clone, JsonSchema)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct User {
    pub id: Option<i32>,
    #[serde(skip_deserializing)]
    pub password: String,
    pub name: String,
    pub active: bool,
}
