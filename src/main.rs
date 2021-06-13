#[macro_use] extern crate rocket;

use std::collections::HashMap;

use rocket::{Rocket, Build, State};
use rocket::fs::{FileServer, relative};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use rocket::fairing::{self, AdHoc};

mod db;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Resource {
    name: String,
    status: String,
    description: String,
    other_fields: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResourceUpdateReq {
    name: String,
    status: Option<String>,
    description: Option<String>,
    other_fields: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResourceDeleteReq {
    name: String,
}

type ResourceList = Vec<Resource>;
type Db = sqlx::PgPool;

#[post("/resource", format = "json", data = "<req>")]
fn update(req: Json<ResourceUpdateReq>, db: &State<Db>) {
    println!("{:?}", req);
    todo!("")
}

#[get("/resource", format = "json")]
fn get(db: &State<Db>) -> Json<ResourceList> {
    // TODO: implement me
    Vec::<Resource>::new().into()
}

#[delete("/resource", format = "json", data = "<req>")]
fn delete(req: Json<ResourceDeleteReq>, db: &State<Db>) {
    todo!("")
}

async fn init_db(rocket: Rocket<Build>) -> fairing::Result {
    let db = match db::new_resource_db("TODO").await {
        Ok(db) => db,
        Err(_) => return Err(rocket),
    };
    if let Err(e) = sqlx::migrate!("./migrations").run(&db).await {
        error!("Failed to initialize SQLx database: {}", e);
        return Err(rocket);
    }
    Ok(rocket.manage(db))
}


fn sqlx_stage() -> AdHoc {
    AdHoc::on_ignite("sqlx stage", |rocket| async {
        rocket
            .attach(AdHoc::try_on_ignite("sqlx db", init_db))
            .mount("/", routes![update, get, delete])

    })
}

#[rocket::main]
async fn main() {
    env_logger::init();
    if let Err(e) = rocket::build()
        .attach(sqlx_stage())
        .mount("/", FileServer::from(relative!("public")))
        .launch()
        .await
    {
        println!("Rocket didn't launch");
        // drop the error to get a Rocket-formatted panic.
        drop(e);
    };
}
