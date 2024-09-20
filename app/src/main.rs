use axum::routing::get;
use axum::Router;
#[tokio::main]
async fn main() {
    // build our application with a route
    let app: Router = Router::new()
        // `GET /` goes to `root`
        .route("/hello", get(hello_handler));
    // `POST /users` goes to `create_user`

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello_handler() -> String {
    String::from("Hello World!")
}
