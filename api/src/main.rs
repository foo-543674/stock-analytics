use stock_analytics::{modules::root::RootModule, routes::router::router};

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  let module = RootModule::new();

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, router(module)).await.unwrap();
}