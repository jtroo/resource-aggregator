use std::collections::HashMap;

use sqlx::migrate::MigrateDatabase;
use serde::{Serialize, Deserialize};

use axum::prelude::*;
use std::net::SocketAddr;

mod db;

type UnixTime = i64;

prae::define! {
    pub Name: String
    adjust |u| *u = u.trim().to_string()
    ensure |u| u.len() < 50
}

prae::define! {
    pub Description: String
    adjust |u| *u = u.trim().to_string()
    ensure |u| u.len() < 100
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    name: String,
    description: String,
    reserved_until: UnixTime,
    reserved_by: String,
    other_fields: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceCreateReq {
    name: Name,
    description: Description,
    other_fields: Option<HashMap<Name, Description>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUpdateReq {
    name: Name,
    new_name: Option<Name>,
    description: Option<Description>,
    reserved_until: Option<UnixTime>,
    reserved_by: Option<Name>,
    other_fields: Option<HashMap<Name, Description>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceDeleteReq {
    name: Name,
}

type ResourceList = Vec<Resource>;
type Db = sqlx::PgPool;

// #[derive(Responder)]
// enum Response {
//     ResourceList(Json<ResourceList>),
//     #[response(status = 200)]
//     OK(()),
//     #[response(status = 400)]
//     BadRequest(String),
//     #[response(status = 500)]
//     ServerError(String),
// }

// #[post("/resource/new", format = "json", data = "<req>")]
// async fn create(req: Json<ResourceCreateReq>, db: &State<Db>) -> Response {
//     let new_resource = Resource {
//         name: req.name.clone().into_inner(),
//         description: req.description.clone().into_inner(),
//         reserved_until: Default::default(),
//         reserved_by: Default::default(),
//         other_fields: req
//             .clone()
//             .other_fields
//             .unwrap_or_default()
//             .into_iter()
//             .map(|(k, v)| (k.into_inner(), v.into_inner()))
//             .collect(),
//     };
//     match db::create_resource(db, &new_resource).await {
//         Ok(Ok(())) => Response::OK(()),
//         Ok(Err(msg)) => Response::BadRequest(msg),
//         Err(e) => {
//             log::error!("{:?}", e);
//             Response::ServerError("Could not create resource".into())
//         }
//     }
// }

// #[post("/resource", format = "json", data = "<req>")]
// async fn update(req: Json<ResourceUpdateReq>, db: &State<Db>) -> Response {
//     match db::update_resource(db, req.clone()).await {
//         Ok(Ok(())) => Response::OK(()),
//         Ok(Err(msg)) => Response::BadRequest(msg),
//         Err(e) => {
//             log::error!("{:?}", e);
//             Response::ServerError("Could not update resource".into())
//         }
//     }
// }

// #[get("/resource", format = "json")]
// async fn get(db: &State<Db>) -> Response {
//     match db::list_resources(db).await {
//         Ok(v) => Response::ResourceList(v.into()),
//         Err(e) => {
//             log::error!("{:?}", e);
//             Response::ServerError("Could not retrieve data".into())
//         }
//     }
// }

// #[delete("/resource", format = "json", data = "<req>")]
// async fn delete(req: Json<ResourceDeleteReq>, db: &State<Db>) -> Response {
//     match db::delete_resource(db, &req.name).await {
//         Ok(_) => Response::OK(()),
//         Err(e) => {
//             log::error!("{:?}", e);
//             Response::ServerError("Database error".into())
//         }
//     }
// }

fn create_clear_expired_reservations_worker(db: Db) {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            let _ = db::clear_expired_reservations(&db).await;
        }
    });
}

// async fn init_db(rocket: Rocket<Build>) -> fairing::Result {
//     let db_url =
//         std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");
//     if let Err(e) = sqlx::Postgres::create_database(&db_url).await {
//         info!("Could not create database: {}", e);
//     } else {
//         warn!("New database created");
//     }
//     let db = match db::new_resource_db(&db_url).await {
//         Ok(db) => db,
//         Err(_) => return Err(rocket),
//     };
//     if let Err(e) = sqlx::migrate!("./migrations").run(&db).await {
//         error!("Failed to initialize SQLx database: {}", e);
//         return Err(rocket);
//     }
//     create_clear_expired_reservations_worker(db.clone());
//     Ok(rocket.manage(db))
// }

// fn sqlx_stage() -> AdHoc {
//     AdHoc::on_ignite("sqlx stage", |rocket| async {
//         rocket
//             .attach(AdHoc::try_on_ignite("sqlx db", init_db))
//             .mount("/", routes![update, get, delete, create])
//     })
// }

// #[catch(404)]
// pub async fn spa_handler() -> NamedFile {
//     NamedFile::open("./public/index.html").await.unwrap()
// }

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    use tracing_subscriber::EnvFilter;
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // // TODO: add SPA handler
    // let rocket = rocket::build()
    //     .attach(sqlx_stage())
    //     .mount("/", FileServer::from("./public/"))
    //     .register("/", catchers![spa_handler]);

    // // TODO: add CORs
    // #[cfg(feature = "dev_cors")]
    // let rocket = {
    //     use rocket::http::Method;
    //     use rocket_cors::{AllowedHeaders, AllowedOrigins};

    //     let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:4200"]);
    //     let cors = rocket_cors::CorsOptions {
    //         allowed_origins,
    //         // TODO: add routes
    //         allowed_methods: vec![Method::Get, Method::Post, Method::Delete]
    //             .into_iter()
    //             .map(From::from)
    //             .collect(),
    //         allowed_headers: AllowedHeaders::all(),
    //         allow_credentials: true,
    //         ..Default::default()
    //     }
    //     .to_cors()?;

    //     rocket
    //         .mount("/", rocket_cors::catch_all_options_routes())
    //         .attach(cors.clone())
    //         .manage(cors)
    // };

    // if let Err(e) = rocket.launch().await {
    //     println!("Rocket didn't launch");
    //     // drop the error to get a Rocket-formatted panic.
    //     drop(e);
    // };

    // build our application with a route
    let app = route("/hello_world", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}", addr);
    Ok(hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?)
}

async fn handler() -> response::Html<&'static str> {
    response::Html("<h1>Hello, World!</h1>")
}
