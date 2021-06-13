use sqlx::postgres::PgPool;
use super::*;
use rocket::serde::json::Json;

pub async fn new_db_storage(url: &str) -> Result<PgPool, ()> {
    Ok(PgPool::connect(url).await.map_err(|e| {
        log::error!("{}", e);
    })?)
}

pub async fn update_resource(pool: &PgPool, res: &Resource) -> anyhow::Result<i64> {
    let rec = sqlx::query!(
        r#"
INSERT INTO resources (name, status, description, other_fields)
VALUES ($1,$2,$3,$4)
RETURNING id
        "#,
        res.name, res.status, res.description, Json(res.other_fields) as _,
    )
    .fetch_one(pool)
    .await?;
    Ok(rec.id)
}

struct ResourceRow {
    name: String,
    status: String,
    description: String,
    other_fields: Json<HashMap<String, String>>,
}

async fn list_resources(pool: &PgPool) -> anyhow::Result<Vec<Resource>> {
    let rows = sqlx::query_as!(
        Row,
        r#"
SELECT name, status, description, other_fields as "other_fields: Json<Person>"
FROM resources
        "#
    )
    .fetch_all(pool)
    .await?;
    let mut resources = Vec::new();
    for row in rows {
        resources.push(Resource {
            name: row.name,
            status: row.status,
            description: row.description,
            other_fields: row.other_fields.into(),
        });
    }
    Ok(resources)
}
