#[macro_use]
extern crate rocket;

use std::collections::HashMap;

use rocket::fairing::{self, AdHoc};
use rocket::fs::{FileServer, NamedFile};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio;
use rocket::{Build, Rocket, State};
use sqlx::migrate::MigrateDatabase;

mod db;

type UnixTime = i64;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Resource {
    name: String,
    description: String,
    reserved_until: UnixTime,
    reserved_by: String,
    other_fields: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResourceCreateReq {
    name: String,
    description: String,
    other_fields: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResourceUpdateReq {
    name: String,
    new_name: Option<String>,
    description: Option<String>,
    reserved_until: Option<UnixTime>,
    reserved_by: Option<String>,
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
        description: req.description.clone(),
        reserved_until: Default::default(),
        reserved_by: Default::default(),
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

fn create_clear_expired_reservations_worker(db: Db) {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            let _ = db::clear_expired_reservations(&db).await;
        }
    });
}

async fn init_db(rocket: Rocket<Build>) -> fairing::Result {
    let db_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");
    if let Err(e) = sqlx::Postgres::create_database(&db_url).await {
        info!("Could not create database: {}", e);
    } else {
        warn!("New database created");
    }
    let db = match db::new_resource_db(&db_url).await {
        Ok(db) => db,
        Err(_) => return Err(rocket),
    };
    if let Err(e) = sqlx::migrate!("./migrations").run(&db).await {
        error!("Failed to initialize SQLx database: {}", e);
        return Err(rocket);
    }
    create_clear_expired_reservations_worker(db.clone());
    Ok(rocket.manage(db))
}

fn sqlx_stage() -> AdHoc {
    AdHoc::on_ignite("sqlx stage", |rocket| async {
        rocket
            .attach(AdHoc::try_on_ignite("sqlx db", init_db))
            .mount("/", routes![update, get, delete, create])
    })
}

#[catch(404)]
pub async fn spa_handler() -> NamedFile {
    NamedFile::open("./public/index.html").await.unwrap()
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let rocket = rocket::build()
        .attach(sqlx_stage())
        .mount("/", FileServer::from("./public/"))
        .register("/", catchers![spa_handler]);

    #[cfg(feature = "dev_cors")]
    let rocket = {
        use rocket::http::Method;
        use rocket_cors::{AllowedHeaders, AllowedOrigins};

        let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:4200"]);
        let cors = rocket_cors::CorsOptions {
            allowed_origins,
            allowed_methods: vec![Method::Get, Method::Post, Method::Delete]
                .into_iter()
                .map(From::from)
                .collect(),
            allowed_headers: AllowedHeaders::all(),
            allow_credentials: true,
            ..Default::default()
        }
        .to_cors()?;

        rocket
            .mount("/", rocket_cors::catch_all_options_routes())
            .attach(cors.clone())
            .manage(cors)
    };

    if let Err(e) = rocket.launch().await {
        println!("Rocket didn't launch");
        // drop the error to get a Rocket-formatted panic.
        drop(e);
    };

    Ok(())
}
