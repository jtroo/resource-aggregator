#[macro_use]
extern crate rocket;

use std::collections::HashMap;

use rocket::fairing::{self, AdHoc};
use rocket::fs::{relative, FileServer};
use rocket::http::Method;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{Build, Rocket, State};
use rocket_cors::{AllowedHeaders, AllowedOrigins};

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
pub struct ResourceCreateReq {
    name: String,
    status: String,
    description: String,
    other_fields: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResourceUpdateReq {
    name: String,
    new_name: Option<String>,
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

#[derive(Responder)]
enum Response {
    ResourceList(Json<ResourceList>),
    #[response(status = 200)]
    OK(()),
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 500)]
    ServerError(String),
}

#[post("/resource/new", format = "json", data = "<req>")]
async fn create(req: Json<ResourceCreateReq>, db: &State<Db>) -> Response {
    let new_resource = Resource {
        name: req.name.clone(),
        status: req.status.clone(),
        description: req.description.clone(),
        other_fields: req.other_fields.clone().unwrap_or_default(),
    };
    match db::create_resource(db, &new_resource).await {
        Ok(Ok(())) => Response::OK(()),
        Ok(Err(msg)) => Response::BadRequest(msg),
        Err(e) => {
            log::error!("{:?}", e);
            Response::ServerError("Could not create resource".into())
        }
    }
}

#[post("/resource", format = "json", data = "<req>")]
async fn update(req: Json<ResourceUpdateReq>, db: &State<Db>) -> Response {
    match db::update_resource(db, req.clone()).await {
        Ok(Ok(())) => Response::OK(()),
        Ok(Err(msg)) => Response::BadRequest(msg),
        Err(e) => {
            log::error!("{:?}", e);
            Response::ServerError("Could not update resource".into())
        }
    }
}

#[get("/resource", format = "json")]
async fn get(db: &State<Db>) -> Response {
    match db::list_resources(db).await {
        Ok(v) => Response::ResourceList(v.into()),
        Err(e) => {
            log::error!("{:?}", e);
            Response::ServerError("Could not retrieve data".into())
        }
    }
}

#[delete("/resource", format = "json", data = "<req>")]
async fn delete(req: Json<ResourceDeleteReq>, db: &State<Db>) -> Response {
    match db::delete_resource(db, &req.name).await {
        Ok(_) => Response::OK(()),
        Err(e) => {
            log::error!("{:?}", e);
            Response::ServerError("Database error".into())
        }
    }
}

async fn init_db(rocket: Rocket<Build>) -> fairing::Result {
    let db = match db::new_resource_db(
        &std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set"),
    )
    .await
    {
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
            .mount("/", routes![update, get, delete, create])
    })
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:8080"]);
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    if let Err(e) = rocket::build()
        .attach(sqlx_stage())
        .attach(cors)
        .mount("/", FileServer::from(relative!("public")))
        .launch()
        .await
    {
        println!("Rocket didn't launch");
        // drop the error to get a Rocket-formatted panic.
        drop(e);
    };
    Ok(())
}
