use crate::models::user::{schema::users as Schema, User as Model};
use crate::DbConn;
use diesel::prelude::*;
use diesel::result::QueryResult;
use rocket::response::Debug;

use Schema::dsl::users as all_users;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

impl Model {
    pub async fn all(conn: DbConn) -> Result<Vec<Model>> {
        let ret = conn.run(|c| Schema::table.load::<Model>(c)).await?;
        Ok(ret)
    }
}
