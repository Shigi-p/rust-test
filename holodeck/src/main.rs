// https://caddi.tech/archives/416
use holodeck::Todo;
use warp::Filter;

fn todo() -> warp::filters::BoxedFilter<()> {
    warp::path("todo").boxed()
}

fn item() -> warp::filters::BoxedFilter<(Todo,)> {
    warp::path::param().boxed()
}

async fn greet_handler(todo: Todo) -> Result<impl warp::Reply, warp::Rejection> {
    let reply = format!("Todo: {}", todo);
    Ok(warp::reply::html(reply))
}

// 今回tokioのランタイムを利用する
// 非同期ランタイムの上で実行されるためmain関数はasyncをつけて定義します
#[tokio::main]
async fn main() {
    let todo = todo().and(item()).and_then(greet_handler);
    warp::serve(todo).run(([127, 0, 0, 1], 3030)).await;
}
