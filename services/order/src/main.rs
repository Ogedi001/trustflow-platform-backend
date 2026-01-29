use order::app::build_app;

#[tokio::main]
async fn main() {
    let app = build_app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:6060").await.unwrap();
    println!(
        "Booking service listening on {}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
