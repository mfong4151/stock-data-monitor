mod aws_ses;
mod polygon_api;

use std::io::Error;
use axum::{
  routing::{get},
  Router
};
use std::env;
async fn test() -> &'static str {
  "Your server works"
}


#[tokio::main]
async fn main() -> Result<(), Error> {

  let app = Router::new().route("/", get(test ));
  let port =  env::var("PORT")
                            .unwrap_or_else(|_| "5000".to_string())
                            .parse::<u16>()
                            .expect("PORT must be a number");


  let address  = if port == 5000 {  format!( "127.0.0.1:{}", port) } else{  format!("0.0.0.0:{}", port) };

  let listener = tokio::net::TcpListener::bind(address)
  .await
  .unwrap();

  axum::serve(listener, app).await.unwrap();
  // let sent_email = send_email().await;

  Ok(())
}
