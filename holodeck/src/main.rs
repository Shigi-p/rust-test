use holodeck::{init_db, todo_list_api};

#[tokio::main]
async fn main() {
    let database = init_db();
    warp::serve(todo_list_api(database))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
