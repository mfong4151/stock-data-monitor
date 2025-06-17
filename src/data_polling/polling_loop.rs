use std::{collections::HashMap, thread, time::Duration};

use dotenv::dotenv;
use chrono::{Local, Timelike};
use std::env;

use crate::{aws_ses::send_email::send_email, data_polling::manage_offset::{manage_offset, parse_offsetted_time}, polygon_api::{fetch_data::fetch_data, stock::StockData}};


/**
 * Main polling procedure, should perform the steps in this order
 * 
 * 1. Fetch the data related to a current stock
 * 2. Perform analysis on it: get the standard deviation
 * 3. Determine whether the state of data warrants sending an alert. If so send the alert
 */
pub async fn monitor_stock_data(stock_data_map: &mut HashMap<String, StockData<'_>>) {

  // TODO put in real values here
  let timeframe: u32  = 5;  
      dotenv().ok();

  let api_key =  env::var("POLYGON_API_KEY") 
        .expect("Expecting POLYGON_API_KEY to be set"); // TODO  move API key out of here, make global.
  let is_using_timestamp = false;
  let mut prev_fetched_min: u32  = 61; //First value is 61 because we will never be at 61 minutes  in a traditional clock
  const MINUTES_TO_MILIS: i64 = 60 * 1000;
  let keys: Vec<String> = stock_data_map.keys().cloned().collect(); 
  
  parse_offsetted_time(manage_offset());

  loop {
    let now = Local::now();
    let timestamp_to: i64 = if is_using_timestamp  {now.timestamp_millis() } else {manage_offset()};
    let timestamp_from: i64 = timestamp_to -  timeframe as i64 * MINUTES_TO_MILIS;

    let now_min =  now.minute();
    
    let is_market_closed =  is_market_closed(&now);  
    let is_already_fetched = now_min - now_min % timeframe == prev_fetched_min;
 
    if is_market_closed || is_already_fetched {
      println!("{:?}", if is_market_closed { "Waiting for market to open"} else { "Waiting for data to become availible before fetching"});
      thread::sleep(Duration::from_secs(60));
      continue;
    }

    for ticker in &keys {
      
      let stock_data =  stock_data_map.get_mut(ticker).unwrap();
      let polygon_data = fetch_data(&ticker, &timeframe, &timestamp_from, &timestamp_to, &api_key).await;
      

      stock_data.add_stock_data(&polygon_data.unwrap());
      stock_data.maybe_evict_if_over_cap();
      let is_alert_fireable: bool = stock_data.analyze().is_alert_fireable();

      // match stock_data.pop_front_if_at_capacity() {
      //     // TODO create a method that adds stock data to a database. 
      //     Some(data)  => {},
      //     None =>  {},
      // };

      
      // if is_alert_fireable{
      //   let email_res = send_email().await;
      //   if let Err(e) = email_res {
      //       eprintln!("Failed to send email {}", e);
      //   }
      // }

      prev_fetched_min  = now_min - now_min % timeframe;


        // TODO remove when confirmed
      for row in stock_data.stock_data.iter(){
        println!("ticker: {:?}, close: {:?}, open: {:?}, high:  {:?}, low: {:?} \n", ticker, row.close, row.open, row.high, row.low);
      }


    }
  }
}

fn is_market_closed(now: &chrono::DateTime<Local>)-> bool{
  now.hour() * 60 + now.minute() < 390 || now.hour() > 13
}
