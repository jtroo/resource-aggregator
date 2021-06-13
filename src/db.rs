use sqlx::postgres::PgPool;
use super::*;
use sqlx::types::Json;
use std::collections::HashMap;

pub(crate) async fn new_resource_db(url: &str) -> Result<PgPool, ()> {
    Ok(PgPool::connect(url).await.map_err(|e| {
        log::error!("{}", e);
    })?)
}

/// If the outer result is an `Err`, it is a server error. If the inner result is an `Err`, it is a
/// client request error. Currently the only client request error possible is that a resource with
/// the same name already exists.
pub(crate) async fn create_resource(pool: &PgPool, res: &Resource) -> anyhow::Result<Result<(), String>> {
    let result = sqlx::query!(
        r#"
INSERT INTO resources (name, status, description, other_fields)
VALUES ($1,$2,$3,$4)
        "#,
        res.name, res.status, res.description, Json(&res.other_fields) as _,
    )
    .execute(pool)
    .await?;
    Ok(match result.rows_affected() {
        0 => Err("Name already exists".into()),
        1 => Ok(()),
        _ => panic!("More than 1 row affected in create"),
    })
}

pub(crate) async fn update_resource(pool: &PgPool, res: &Resource) -> anyhow::Result<String> {
    let rec = sqlx::query!(
        r#"
INSERT INTO resources (name, status, description, other_fields)
VALUES ($1,$2,$3,$4)
RETURNING name
        "#,
        res.name, res.status, res.description, Json(&res.other_fields) as _,
    )
    .fetch_one(pool)
    .await?;
    Ok(rec.name)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct OtherFields {
    #[serde(flatten)]
    fields: HashMap<String, String>,
}

struct ResourceRow {
    name: String,
    status: String,
    description: String,
    other_fields: Json<OtherFields>,
}

pub(crate) async fn list_resources(pool: &PgPool) -> anyhow::Result<Vec<Resource>> {
    let rows = sqlx::query_as!(
        ResourceRow,
        r#"
SELECT name, status, description, other_fields as "other_fields: Json<OtherFields>"
FROM resources
        "#
    )
    .fetch_all(pool)
    .await?;
    let mut resources = Vec::new();
    for row in rows.into_iter() {
        let other_fields = row.other_fields.fields.clone();
        resources.push(Resource {
            name: row.name,
            status: row.status,
            description: row.description,
            other_fields,
        });
    }
    Ok(resources)
}

pub(crate) async fn delete_resource(pool: &PgPool, name: &str) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
DELETE
FROM resources
WHERE name = ($1)
        "#,
        name
    )
    .execute(pool)
    .await?;
    Ok(())
}
