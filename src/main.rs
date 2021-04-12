use warp::{Filter, Rejection, Reply};

type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    let health_route = warp::path!("health").and_then(health_handler);
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));


    let routes = health_route.with(warp::cors().allow_any_origin()).or(hello);

    println!("Started server at localhost:50051");
    warp::serve(routes).run(([0, 0, 0, 0], 50051)).await;
}

async fn health_handler() -> Result<impl Reply> {
    Ok("OK")
}

