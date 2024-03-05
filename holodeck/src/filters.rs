use warp::{Filter, Rejection, Reply};

use crate::{get_todo_handler, list_todo_handler, put_todo_handler, Database};

pub fn todo_list_api(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_todo(db.clone()).or(list(db.clone())).or(put_user(db))
}

fn todo_list() -> warp::filters::BoxedFilter<()> {
    warp::path("todo_list").boxed()
}

fn todo_id() -> warp::filters::BoxedFilter<(u64,)> {
    warp::path::param().boxed()
}

fn list(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    todo_list()
        .and(warp::get())
        .and_then(move || list_todo_handler(db.clone()))
}

fn get_todo(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    todo_list()
        .and(todo_id())
        .and(warp::get())
        .and_then(move |id| get_todo_handler(db.clone(), id))
}

fn put_user(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    todo_list()
        .and(todo_id())
        .and(warp::put())
        .and(warp::body::json())
        .and_then(move |id, body| put_todo_handler(db.clone(), id, body))
}
