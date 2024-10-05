mod models;
mod routes;
mod db;
mod error; 
use crate::routes::import_data;
use warp::Filter;
use error::CustomError;

#[tokio::main]
async fn main() {
    let pool = db::create_pool().await;

    if let Err(e) = import_data(&pool).await {
        eprintln!("Error importing data: {:?}", e);
    }

    let api = routes::nodes_route(pool.clone())
        .recover(handle_rejection); 

    warp::serve(api)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(custom_err) = err.find::<CustomError>() {
        Ok(warp::reply::with_status(
            warp::reply::json(&custom_err.message),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else {
        Ok(warp::reply::with_status(
            warp::reply::json(&"Route not found"),
            warp::http::StatusCode::NOT_FOUND,
        ))
    }
}
