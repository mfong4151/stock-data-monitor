mod aws_ses;
mod polygon_api;

use std::io::Error;
use axum::{ routing::{ get }, Router };
use std::env;
use aws_ses::send_email::send_email;

async fn test() -> &'static str {
    "Your server works"
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = Router::new().route("/", get(test));
    let port = env
        ::var("PORT")
        .unwrap_or_else(|_| "5000".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    let address = if port == 5000 {
        format!("127.0.0.1:{}", port)
    } else {
        format!("0.0.0.0:{}", port)
    };

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    let email_task = tokio::spawn(async {
        let email_res = send_email().await;
        if let Err(e) = email_res {
            eprintln!("Failed to send email {}", e);
        }
    });

    tokio::select! {
        _ = axum::serve(listener, app) => {
        eprintln!("Server stopped unexpectedly")
    },
      _ = email_task => {
        eprintln!("Email sent");
      }
  }
    Ok(())
}
