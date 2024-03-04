// https://dev.to/rogertorres/rest-api-with-rust-warp-1-introduction-342e

#[cfg(test)]
mod tests {
    use super::filters;
    use warp::http::StatusCode;
    use warp::test::request;

    #[tokio::test]
    async fn try_list() {
        let api = filters::list();

        let response = request().method("GET").path("/holodeck").reply(&api).await;

        assert_eq!(response.status(), StatusCode::OK);
    }
}

mod filters {
    use super::handlers;
    use warp::Filter;

    pub fn list() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("holodeck")
            .and(warp::get())
            .and_then(handlers::handle_list)
    }
}

mod handlers {
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn handle_list() -> Result<impl warp::Reply, Infallible> {
        Ok(StatusCode::OK)
    }
}

/// 名前を表す型の定義
#[derive(Clone, Debug)]
pub struct Todo(String);

impl Todo {
    /// 値のチェックを行った上でNameを作成する
    /// 今回はサンプルのため作成の失敗をString型で表現している
    pub fn new(todo: &str) -> Result<Self, String> {
        let size = todo.chars().count();
        if size < 1 || size > 50 {
            return Err("TODOは50文字以内の簡潔なものにしてください".to_string());
        }

        Ok(Todo(todo.to_string()))
    }
}

/// 文字列からの変換を表す
/// このtraitの実装をwarp::path::params()関数が要求する
impl std::str::FromStr for Todo {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Todo::new(s)
    }
}

/// handlerでformatを行うために要求される
impl std::fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[test]
fn test_todo() {
    let ok_value: &str = "Foo";
    assert!(Todo::new(ok_value).is_ok());

    let ng_value: &str = "";
    assert!(Todo::new(ng_value).is_err());

    let mut ng_value_length: String = String::new();
    while ng_value_length.len() < 50 {
        ng_value_length.push_str('a'.to_string().as_str());
    }
    assert!(Todo::new(ng_value).is_err());
}
