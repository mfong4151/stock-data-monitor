mod aws_ses;
mod polygon_api;
mod data_polling;

use std::{collections::HashMap, io::Error};
use axum::{ routing::{ get }, Router };
use data_polling::setup_stock_data::setup_stock_data;
use polygon_api::stock::StockData;
use std::env;
use aws_ses::send_email::send_email;
use data_polling::*;



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
    let _ = tokio::spawn(async {
        
        let tickers = vec!["QQQ"];
        let mut initial_stock_data: HashMap<String, StockData> = setup_stock_data(&tickers);

        data_polling::polling_loop::monitor_stock_data(&mut initial_stock_data).await;
        // let email_res = send_email().await;
        // if let Err(e) = email_res {
        //     eprintln!("Failed to send email {}", e);
        // }
    });

    axum::serve(listener, app).await.unwrap();
  
    Ok(())
}