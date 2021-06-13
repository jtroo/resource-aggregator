#[macro_use] extern crate rocket;

use std::collections::HashMap;

use rocket::fs::{FileServer, relative};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;

mod db;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct ResourceUpdateRequest {
    name: String,
    status: Option<String>,
    description: Option<String>,
    other_fields: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Resource {
    name: String,
    status: String,
    description: String,
    other_fields: HashMap<String, String>,
}

type ResourceList = Vec<Resource>;

#[post("/resource", format = "json", data = "<req>")]
fn update(req: Json<ResourceUpdateRequest>) {
    println!("{:?}", req);
    todo!("")
}

#[get("/resource", format = "json")]
fn get() -> Json<ResourceList> {
    // TODO: implement me
    Vec::<Resource>::new().into()
}

#[delete("/resource")]
fn delete() {
    todo!("")
}

#[rocket::main]
async fn main() {
    env_logger::init();
    if let Err(e) = rocket::build()
        .mount("/", routes![update, get, delete])
        .mount("/", FileServer::from(relative!("public")))
        .launch()
        .await
    {
        println!("Rocket didn't launch");
        // drop the error to get a Rocket-formatted panic.
        drop(e);
    };
}
