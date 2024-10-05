use warp::Filter;
use sqlx::Pool;
use sqlx::Postgres;
use crate::models::Node;
use crate::models::ExternalNode;
use crate::error::CustomError;
use chrono::{TimeZone, Utc};

pub fn nodes_route(pool: Pool<Postgres>) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("nodes")
        .and(warp::get())
        .and(with_db(pool))
        .and_then(get_nodes)
}

fn with_db(pool: Pool<Postgres>) -> impl Filter<Extract = (Pool<Postgres>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

async fn get_nodes(pool: Pool<Postgres>) -> Result<impl warp::Reply, warp::Rejection> {
    let nodes: Vec<Node> = sqlx::query_as!(
        Node,
        r#"
        SELECT public_key, alias, capacity::text, first_seen
        FROM nodes
        "#)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        warp::reject::custom(CustomError {
            message: format!("Database error: {:?}", e),
        })
    })?;
    
    Ok(warp::reply::json(&nodes))
}

pub async fn import_data(pool: &Pool<Postgres>) -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://mempool.space/api/v1/lightning/nodes/rankings/connectivity")
        .await?
        .json::<Vec<ExternalNode>>()
        .await?;

    for node in response {
        let capacity_in_btc = node.capacity as f64 / 100_000_000.0;
        let first_seen_str = Utc.timestamp_opt(node.first_seen, 0)
            .single()
            .expect("Invalid timestamp")
            .to_rfc3339();

        if let Err(e) = sqlx::query!(
            r#"
            INSERT INTO nodes (public_key, alias, capacity, first_seen)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (public_key) DO NOTHING
            "#,
            node.public_key,
            node.alias,
            capacity_in_btc.to_string(),
            first_seen_str
        )
        .execute(pool)
        .await
        {
            eprintln!("Failed to insert node into DB: {:?}", e);
        }
    }
    Ok(())
}
