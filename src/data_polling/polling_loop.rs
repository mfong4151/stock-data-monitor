use std::collections::HashMap;

use crate::{aws_ses::send_email::send_email, polygon_api::{fetch_data::fetch_data, stock::StockData}};


/**
 * Main polling procedure, should perform the steps in this order
 * 
 * 1. Fetch the data related to a current stock
 * 2. Perform analysis on it: get the standard deviation
 * 3. Determine whether the state of data warrants sending an alert. If so send the alert
 */
pub async fn monitor_stock_data(stock_data_map: &mut HashMap<String, StockData<'_>>) {

  
  // TODO put in real values here
  let timeframe: u32 = 15;  
  let timestamp_from: i64 = 15;
  let timestamp_to: i64 = 15;
  let api_key = "awgeagawa";
   
  let keys: Vec<String> = stock_data_map.keys().cloned().collect();

  for ticker in  keys {
    
    let stock_data =  stock_data_map.get_mut(&ticker).unwrap();
    

    let polygon_data = fetch_data(&ticker, &timeframe, &timestamp_from, &timestamp_to, api_key).await;

    // // TODO complete match statement
    // match(polygon_data){
      
    // }


    let temp_polygon_data = polygon_data.unwrap();
    stock_data.add_stock_data(&temp_polygon_data);
    let alert_cluster =  stock_data.analyze();
    

    if alert_cluster.is_alert_fireable(){

      let email_res = send_email().await;
      if let Err(e) = email_res {
          eprintln!("Failed to send email {}", e);
      }

    }


  }

}
