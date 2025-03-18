use reqwest::get;
use std::collections::VecDeque;
use std::error::Error;
use crate::polygon_api::stock::StockDatum;
use crate::polygon_api::{
                    stock_data_response::*,
                    stock::StockData
                    };

pub const NULL_STOCK_DATA_RESPONSE : PriceDatum = PriceDatum{
  timestamp: 0,
        volume: 0,
        high: -1.0,
        low: -1.0,
        close: -1.0,
        open: -1.0,
};

pub const EMPTY_RESPONSE_DOUBLE_FLAG: (f64, f64,  f64,  f64 ) =  (0.0, 0.0, 0.0 , 0.0);




/**
 *  TODO: Refactor to contain a retry policy
 */
pub async fn fetch_data(ticker: &str, timeframe: &u32, timestamp_from: &i64, timestamp_to: &i64, api_key: &str) -> Result<PriceDatum, Box<dyn Error>> {
    //Initialize with a default falsey response 
    let mut stock_data_response: PriceDatum =  NULL_STOCK_DATA_RESPONSE;

    let request_url =format!( "https://api.polygon.io/v2/aggs/ticker/{}/range/{}/minute/{}/{}/?apiKey={}", ticker, timeframe, timestamp_from, timestamp_to, api_key); 
    // println!("{:?}", request_url.clone()); // Debugger variable
    let res = get(request_url).await?;

    let is_res_ok = res.status().is_success();
    // let raw_json: serde_json::Value = res.json().await?;
    // println!("Raw JSON response: {:?}", raw_json); // debugger statement

    let body: Result<PolygonResponse, reqwest::Error> = res.json::<PolygonResponse>().await;

    let is_body_ok: bool = body.is_ok();

    if !is_res_ok {
        println!("Error collecting data due to issue connecting to data provider for symbol {}", ticker);
        return Ok(stock_data_response);
    } 

    if is_body_ok && !body.as_ref().unwrap().results.is_some(){

        println!("Request and response were succesful, but no body was found.");
        return Ok(stock_data_response);

    } else {

        stock_data_response = format_price_datum(&body.unwrap());

    } 
    Ok(stock_data_response)

}

/*
* Takes the last entry in the PolygonResponse, returns it as a PriceDatum type
*
* returns a single PriceDatum
*/
fn format_price_datum(polygon_response: &PolygonResponse) -> PriceDatum{

    let results = polygon_response.results.as_ref().unwrap();
    let stock_entry = &results[results.len() - 1];
    // println!("{:?}", polygon_response); //Debugger statement
    PriceDatum{
        volume: stock_entry.v as u64,
        high: stock_entry.h,
        low: stock_entry.l,
        close: stock_entry.c,
        open: stock_entry.o,
        timestamp: stock_entry.t
    }
}

/**
 * TODO add this to the StockData impl
 * Adds stock data to the stock data container
 * Returns a reference to the high low and close to make checking if the current high and low range over the 9ema/20ema
 *
 * @param param description.
 * @param param description.
 * @return return value description.
 */
pub fn add_stock_data( stock: &mut StockData,  stock_data_response: &PriceDatum ) -> (f64, f64, f64, f64){

    // In the case of bad responses return 0.0 *4, this is a flag that we shouldn't 
    if stock_data_response == &NULL_STOCK_DATA_RESPONSE{ 
      return EMPTY_RESPONSE_DOUBLE_FLAG; 
    }

    let last_close = &stock_data_response.close;
    let (ema_9, ema_20) = calculate_emas(&stock.stock_data, last_close);

    let incoming_data = StockDatum{
      open: stock_data_response.open,
      high: stock_data_response.high, 
      low: stock_data_response.low,
      close: stock_data_response.close,
      volume: stock_data_response.volume,
      ema9: ema_9,
      ema20: ema_20,
      timestamp: stock_data_response.timestamp,
      high_low_type : None
    };
    
    
    stock.stock_data.push_back(incoming_data);
    return (stock_data_response.high, stock_data_response.low, ema_9, ema_20);

}



/**
 * Encapsulates logic related to retrieving emas,
 * mostly broken into this separate function in order to keep edge case articulation cleaner.
 * 
 * Attempts to cover the following edge cases
 * 1. When we have a stock that we're analyizing that we just added to the the tickers to scan for and there is no length to the stock_data vec, i.e. to prevent underflow
 *
 * @param stock_data a veqdeque of stock data.
 * @param  last_close, a float representing the last close
 * 
 * @return return value description.
 */
 fn calculate_emas(stock_data: &VecDeque<StockDatum>, last_close:&f64) -> (f64, f64) {

    let mut ema_9: f64;
    let mut ema_20 : f64;
    let length_stock_data: usize = stock_data.len();

    //Ema 9   
    if stock_data.len() < 9{
      ema_9 = 0.0;

    } else {

      let last_stock_idx =  stock_data.len() - 1; // These values might underflow, in these cases we directly return 0.0
      let last_ema_9 = stock_data[last_stock_idx].ema9;
      if length_stock_data == 9{
        ema_9 = calculate_sma(stock_data, 9.0);
      }

      ema_9 = calculate_ema(9.0,last_close, &last_ema_9) ; 
    }

    //EMA 20
    if stock_data.len() < 20{

      ema_20 = 0.0;

    } else {

      let last_stock_idx =  stock_data.len() - 1; // These values might underflow, in these cases we directly return 0.0
      let last_ema_20 = stock_data[last_stock_idx].ema20;
      if length_stock_data == 20{
        ema_20 = calculate_sma(stock_data, 20.0);
      }
      ema_20 = calculate_ema(20.0,last_close, &last_ema_20);
    }


 

  return (ema_9, ema_20);
}


/**
 * Generically calculate the sma length
 * TODO: Refactor to use VecDeque.range, and compare performance.
 */
fn calculate_sma(stock_data: &VecDeque<StockDatum>, sma_length: f64) -> f64 {  
    let mut sum: f64 = 0.0;

    let length_stock_data: usize = stock_data.len();
    for idx in 0..length_stock_data{
        sum += stock_data[length_stock_data - 1 - idx].close;
    }

    return  sum/sma_length;
}

//smoothing: 2.0
pub fn calculate_ema(period: f64, close: &f64, prev_ema: &f64)-> f64 {
  close* (2.0/ (1.0 + period)) + prev_ema * (1.0  - (2.0/ (1.0 + period)))
}