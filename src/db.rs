use super::*;
use sqlx::types::Json;
use sqlx::{postgres::PgPool, Acquire, Connection};
use std::collections::HashMap;
use std::time;

/// Start a connection pool to the resource database at the provided url.
pub(crate) async fn new_resource_db(url: &str) -> Result<PgPool, ()> {
    Ok(PgPool::connect(url).await.map_err(|e| {
        log::error!("{}", e);
    })?)
}

/// Attempt to create a new resource in the database.
///
/// If the outer result is an `Err`, a server error occurred. If the inner result is an `Err`,
/// there is an error with the input. Currently the only caught input error is that a resource with
/// the same name already exists.
pub(crate) async fn create_resource(
    pool: &PgPool,
    res: &Resource,
) -> anyhow::Result<Result<(), String>> {
    Ok(
        match sqlx::query!(
            r#"
        INSERT INTO resources (name, status, description, other_fields)
        VALUES ($1,$2,$3,$4)
        "#,
            res.name,
            res.status,
            res.description,
            Json(&res.other_fields) as _,
        )
        .execute(pool)
        .await?
        .rows_affected()
        {
            0 => Err("Name already exists".into()),
            1 => Ok(()),
            _ => panic!("More than 1 row affected in create"),
        },
    )
}

/// Attempt to update an existing resource in the database
///
/// If the outer result is an `Err`, a server error occurred. If the inner result is an `Err`,
/// there is an error with the input. Currently the only caught input error is that a resource with
/// the provided name does not exist.
pub(crate) async fn update_resource(
    pool: &PgPool,
    req: ResourceUpdateReq,
) -> anyhow::Result<Result<(), String>> {
    let mut conn = pool.acquire().await?;
    let transaction_result: Result<_, sqlx::Error> = conn
        .transaction(|tx| {
            Box::pin(async move {
                if let Some(status) = req.status {
                    if sqlx::query!(
                        r#"UPDATE resources SET status = $1 WHERE name = $2"#,
                        &status,
                        &req.name
                    )
                    .execute(tx.acquire().await?)
                    .await?
                    .rows_affected()
                        == 0
                    {
                        return Ok(Err("Resource does not exist".to_owned()));
                    }
                }
                if let Some(description) = req.description {
                    sqlx::query!(
                        r#"UPDATE resources SET description = $1 WHERE name = $2"#,
                        &description,
                        &req.name
                    )
                    .execute(tx.acquire().await?)
                    .await?;
                }
                if let Some(reserved_until) = req.reserved_until {
                    sqlx::query!(
                        r#"UPDATE resources SET reserved_until = $1 WHERE name = $2"#,
                        reserved_until,
                        &req.name
                    )
                    .execute(tx.acquire().await?)
                    .await?;
                }
                if let Some(reserved_by) = req.reserved_by {
                    sqlx::query!(
                        r#"UPDATE resources SET reserved_by = $1 WHERE name = $2"#,
                        reserved_by,
                        &req.name
                    )
                    .execute(tx.acquire().await?)
                    .await?;
                }
                if let Some(other_fields) = req.other_fields {
                    sqlx::query!(
                        r#"UPDATE resources SET other_fields = $1 WHERE name = $2"#,
                        Json(other_fields) as _,
                        &req.name
                    )
                    .execute(tx.acquire().await?)
                    .await?;
                }
                if let Some(new_name) = req.new_name {
                    sqlx::query!(
                        r#"UPDATE resources SET name = $1 WHERE name = $2"#,
                        &new_name,
                        &req.name
                    )
                    .execute(tx.acquire().await?)
                    .await?;
                }
                Ok(Ok(()))
            })
        })
        .await;
    Ok(transaction_result?)
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
    reserved_until: i64,
    reserved_by: String,
    other_fields: Json<OtherFields>,
}

/// Get all of the resources from the database.
///
/// Returns an error if a server error occurs.
pub(crate) async fn list_resources(pool: &PgPool) -> anyhow::Result<Vec<Resource>> {
    let rows = sqlx::query_as!(
        ResourceRow,
        r#"
SELECT name, status, description, reserved_until, reserved_by, other_fields as "other_fields: Json<OtherFields>"
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
            reserved_until: row.reserved_until,
            reserved_by: row.reserved_by,
            other_fields,
        });
    }
    Ok(resources)
}

/// Delete the resource with the given name.
///
/// If the outer result is an `Err`, a server error occurred. If the inner result is an `Err`,
/// there is an error with the input. Currently the only caught input error is that a resource with
/// the provided name does not exist.
pub(crate) async fn delete_resource(
    pool: &PgPool,
    name: &str,
) -> anyhow::Result<Result<(), String>> {
    Ok(
        match sqlx::query!(r#"DELETE FROM resources WHERE name = $1"#, name)
            .execute(pool)
            .await?
            .rows_affected()
        {
            0 => Err("Resource does not exist".into()),
            1 => Ok(()),
            _ => panic!("More than 1 row affected in delete"),
        },
    )
}

/// Set the values of `reserved_until` and `reserved_by` to the appropriate values for being
/// unreserved for all the rows that have a `reserved_until` value that is in the past.
pub(crate) async fn clear_expired_reservations(pool: &PgPool) {
    let epoch_t = time::SystemTime::now().duration_since(time::UNIX_EPOCH).expect("could not get time since epoch").as_secs() as i64;
    match sqlx::query!(
        r#"
            UPDATE resources
            SET reserved_until = 0, reserved_by = ''
            WHERE reserved_until < $1 AND reserved_until != 0
        "#,
        epoch_t,
    ).execute(pool).await {
        Ok(v) => {
            let rows_affected = v.rows_affected();
            match rows_affected {
                0 => {},
                _ => log::warn!("Cleared {} expired reservation(s)", rows_affected),
            };
        },
        Err(e) => {
            log::error!("{:?}", e);
        }
    }
}
