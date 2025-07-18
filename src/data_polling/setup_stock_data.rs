use std::collections::HashMap;

use crate::polygon_api::stock::StockData;


/**
 * On startup, gets any cache of stock data
 */
pub fn setup_stock_data<'a>(tickers:  &'a Vec<&'a str>)-> HashMap<String, StockData<'a> >{


  let mut res: HashMap<String, StockData<'a>> = HashMap::new();


  tickers.iter().for_each(|ticker | {
    let name = String::from(ticker.clone());
    let _ = &res.insert(ticker.to_string(), StockData::new(name, 50));
  });


  return res;
 } 