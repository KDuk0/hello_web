use warp::Filter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Greeting {
    greeting: String,
    id: String,
}

impl Greeting {
    fn new(greeting: &str, id: String) -> Self {
        Greeting {
            greeting: greeting.to_string(),
            id,
        }
    }
}

#[tokio::main]
async fn main() {
    let get_hello = warp::path!("hello" / String)
    .and(warp::get())
    .map(|id: String| format!("Hello {}!", id));

    let delete_hello = warp::path!("hello" / String)
    .and(warp::delete())
    .map(|id: String| format!("Goodbye {}!", id));

    let routes = get_hello.or(delete_hello);
    warp::serve(routes).bind(([0,0,0,0], 8080)).await;

}