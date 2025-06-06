use reqwest::get;
use std::error::Error;
use crate::polygon_api::{ stock_data_response::*, stock::{StockData,StockDatum}};


pub const NULL_STOCK_DATA_RESPONSE: PriceDatum = PriceDatum {
    timestamp: 0,
    volume: 0,
    high: -1.0,
    low: -1.0,
    close: -1.0,
    open: -1.0,
};

pub const EMPTY_RESPONSE_DOUBLE_FLAG: (f64, f64, f64, f64) = (0.0, 0.0, 0.0, 0.0);

/**
 *  TODO: Cleanup, a lot of this was taken from the algo
 *
 */
pub async fn fetch_data(
    ticker: &str,
    timeframe: &u32,
    timestamp_from: &i64,
    timestamp_to: &i64,
    api_key: &str
) -> Result<PriceDatum, Box<dyn Error>> {
    //Initialize with a default falsey response
    let mut stock_data_response: PriceDatum = NULL_STOCK_DATA_RESPONSE;

    let request_url = format!(
        "https://api.polygon.io/v2/aggs/ticker/{}/range/{}/minute/{}/{}/?apiKey={}",
        ticker,
        timeframe,
        timestamp_from,
        timestamp_to,
        api_key
    );
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

    if is_body_ok && !body.as_ref().unwrap().results.is_some() {
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
fn format_price_datum(polygon_response: &PolygonResponse) -> PriceDatum {
    let results = polygon_response.results.as_ref().unwrap();
    let stock_entry = &results[results.len() - 1];
    // println!("{:?}", polygon_response); //Debugger statement
    PriceDatum {
        volume: stock_entry.v as u64,
        high: stock_entry.h,
        low: stock_entry.l,
        close: stock_entry.c,
        open: stock_entry.o,
        timestamp: stock_entry.t,
    }
}