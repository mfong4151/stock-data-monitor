use std::collections::HashMap;

use crate::polygon_api::stock::StockData;


 pub fn setup_stock_data<'a>(tickers:  &'a Vec<&'a str>)-> HashMap<String, StockData<'a> >{


  let mut res: HashMap<String, StockData<'a>> = HashMap::new();

  // TODO fix ownership issie with String::from(ticker.clone())

  // tickers.iter().for_each(|ticker | {
  //   &res.insert(ticker.to_string(), StockData::new( &String::from(ticker.clone())));
  // });

  return res;
  
 } 