#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel;

// #[cfg(test)]
// mod tests;

mod db;
pub use db::DbConn;
mod models;
mod repos;
mod routers;

use rocket::{
    fairing::AdHoc,
    fs::{relative, FileServer},
};
use rocket_okapi::{
    mount_endpoints_and_merged_docs, openapi_get_routes,
    rapidoc::*,
    settings::{OpenApiSettings, UrlObject},
};
use routers::service as service_routes;
use routers::user as user_routes;

#[launch]
fn rocket() -> _ {
    let mut building_rocket = rocket::build()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_liftoff("Startup Check", |rocket| {
            Box::pin(async move {
                let _ = DbConn::get_one(rocket).await.unwrap();
            })
        }))
        .mount("/", FileServer::from(relative!("static")))
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                title: Some("API Documantation | RapiDoc".to_owned()),
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../v1/openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        );

    let openapi_settings = OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
        building_rocket, "/v1".to_owned(), openapi_settings,
        "/user" => user_routes::get_routes_and_docs(&openapi_settings),
        "/service" => service_routes::get_routes_and_docs(&openapi_settings),
    };

    building_rocket
}
