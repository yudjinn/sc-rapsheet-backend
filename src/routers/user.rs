use rocket::serde::json::Json;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec};

use crate::DbConn;

use crate::models::user::User;
use rocket::response::Debug;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: list]
}

#[openapi(tag = "Users")]
#[get("/")]
pub async fn list(conn: DbConn) -> Result<Json<Vec<User>>> {
    let objs = User::all(conn).await?;
    Ok(Json(objs))
}
