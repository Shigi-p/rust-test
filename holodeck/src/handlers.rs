use warp::{Rejection, Reply};

use crate::{Database, Todo};

pub async fn list_todo_handler(db: Database) -> Result<impl Reply, Rejection> {
    let db = db.lock().await;
    let todo_list = db
        .clone()
        .into_iter()
        .map(|(_, v)| v)
        .collect::<Vec<Todo>>();
    Ok(warp::reply::json(&todo_list))
}

pub async fn get_todo_handler(db: Database, id: u64) -> Result<impl Reply, Rejection> {
    let db = db.lock().await;
    let user = db.get(&id);
    match user {
        None => Err(warp::reject::not_found()),
        Some(u) => Ok(warp::reply::json(&u)),
    }
}

pub async fn put_todo_handler(db: Database, id: u64, todo: Todo) -> Result<impl Reply, Rejection> {
    if id != todo.id() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&()),
            warp::http::StatusCode::BAD_REQUEST,
        ));
    }
    let mut db = db.lock().await;
    db.insert(todo.id(), todo.clone());
    Ok(warp::reply::with_status(
        warp::reply::json(&todo),
        warp::http::StatusCode::OK,
    ))
}
