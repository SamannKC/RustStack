mod models;
mod states;
mod routes;
mod handlers;

#[tokio::main]
async fn main(){

    let state = states::AppState::new();

    let app = routes::app(state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await.unwrap();

    println!("running on localhost:8000");

    axum::serve(listener, app)
        .await.unwrap();
}
