use std::collections::{HashMap, VecDeque};
use crate::data_polling::alert_cluster::{AlertCluster, AlertClusterBuilder};
use crate::data_polling::configs::{MAX_HOUR, MAX_MIN, MIN_HOUR, MIN_MINUTE};
use crate::polygon_api::stock_data_response::PriceDatum;
use crate::polygon_api::fetch_data::{NULL_STOCK_DATA_RESPONSE, EMPTY_RESPONSE_DOUBLE_FLAG};

type RawDataTableRow = HashMap<String, f64>;

/**
 * One unit of stock data, any data that belongs to a particular point should be kept here as opposed to on the larger StockData struct
 *
 */
pub struct StockDatum {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
    pub ema9: f64,
    pub ema20: f64,
    pub timestamp: u64,
    pub high_low_type: Option<HighLow>,
}

impl StockDatum {
  pub fn from_hashmap(dict: &HashMap<String, f64>) -> Self{
      return StockDatum{
        open: dict.get("open").copied().unwrap_or(0.0),
        high: dict.get("high").copied().unwrap_or(0.0),
        low: dict.get("low").copied().unwrap_or(0.0),
        close: dict.get("close").copied().unwrap_or(0.0),
        ema9: dict.get("ema9").copied().unwrap_or(0.0),
        ema20: dict.get("ema20").copied().unwrap_or(0.0),
        volume: dict.get("volume").copied().unwrap_or(0.0) as u64,
        timestamp: dict.get("timestamp").copied().unwrap_or(0.0) as u64,
        high_low_type: None
      }

  }
}

/**
 * Represents three time periods
 *
 * OPEN: the open first 1.5 hrs
 * MIDDAY: The middle of the day after the first 1.5 hrs
 * CLOSE: The last 1 hr
 */
pub enum MarketTimePeriod{
  OPEN,
  MIDDAY,
  CLOSE
}

/**
 * Represents attributes related to volume
 *  volume groups: a hashmap of 3 vecdeques which track the last 50 volumes per each
 *  volume cache: cached volume values
 */
pub struct VolumeAttr{ 
  pub volume_groups: HashMap<MarketTimePeriod, VecDeque<i64>>, 
  pub volume_cache: HashMap<MarketTimePeriod, i64>,
  pub volume_thresholds: HashMap<MarketTimePeriod, i64>
}

impl VolumeAttr {
  pub fn new() -> VolumeAttr{
    return VolumeAttr{
        volume_groups: HashMap::new(),
        volume_cache: HashMap::new(),
        volume_thresholds: HashMap::new()
    };
  }

}


/*
 * An internal representation of a data corresponding to a stock,
 * The main container to "hold" stock data over multiple loop iterations and timeframes
 *
 * name, & 'a str: The ticker name
 * stock_data: a piece of stock_data
 * daily_resistances, VecDeque<f64>: daily chart resistances, not mutable
 * daily_supports, VecDeque<f64>: daily chart supports, not mutable
 * high_low_queue, VecDeque<StockDatum> a smaller VecDeque  that contains only the points necessary to determine trend
 * current_trend, Trend : The current overall trend of the equity
 */
pub struct StockData<'a> {
    pub name: String,
    pub stock_data: VecDeque<StockDatum>,
    stock_data_capacity: u32,
    pub daily_resistances: VecDeque<f64>,
    pub daily_supports: VecDeque<f64>,
    pub high_low_queue: VecDeque<&'a StockDatum>,
    pub volume_attrs: VolumeAttr,
    pub current_trend: Trend,
    //Used for standard deviation calculations
    pub sum_volume: u64
}

impl<'a> StockData<'a> {
    /*
     *    Instantiates an empty new struct
     */
    pub fn new(name: String, stock_data_capacity: u32) -> StockData<'a> {
        StockData {
            name,
            stock_data: VecDeque::new(),
            stock_data_capacity,
            daily_resistances: VecDeque::new(),
            daily_supports: VecDeque::new(),
            high_low_queue: VecDeque::new(),
            volume_attrs: VolumeAttr::new(),
            current_trend: Trend::UNKNOWN,
            sum_volume: 0
        }
    }

    pub fn set_stock_data(&mut self, table: &Vec<RawDataTableRow>){
        let stock_data_points: Vec<StockDatum> =  table.iter().map(|row|{
          return StockDatum::from_hashmap(row)
        }).collect();
        self.stock_data = VecDeque::from(stock_data_points);
    }

    /**
     * Analyzes stock data. 
     * @maybe_volume: a volume if it was evicted from the queue alongside its stock
     * 
     * @return As a side effect fires off an alert cluster, which tells us if immediately we need t o 
     */
    pub fn analyze(& self, now_hour: u32, now_min:u32) -> AlertCluster {
      
      let is_time_to_fire_volume_spike =  (now_hour <= MIN_HOUR  && now_min <= MIN_MINUTE) || (now_hour >= MAX_HOUR && now_min >= MAX_MIN);

      return AlertClusterBuilder::new()
                .set_is_volume_spike(self.is_volume_spike(3.0))  
                .filter_volume_spike(is_time_to_fire_volume_spike)
                .build();
    }

    fn cache_volume (){

    }

    pub fn calculate_standard_deviation(&mut self){
      

    }

    /**
     * TODO refactor to return a reference to self.
     */

    pub fn add_stock_data(
      &mut self, 
      stock_data_response: &PriceDatum
  ) -> (f64, f64, f64, f64) {
      // In the case of bad responses return 0.0 *4, this is a flag that we shouldn't include the data point 
      if stock_data_response == &NULL_STOCK_DATA_RESPONSE {
          return EMPTY_RESPONSE_DOUBLE_FLAG;
      }
  
      let last_close = &stock_data_response.close;
      let (ema_9, ema_20) = Self::calculate_emas(&self.stock_data, last_close);
  
      let incoming_data = StockDatum {
          open: stock_data_response.open,
          high: stock_data_response.high,
          low: stock_data_response.low,
          close: stock_data_response.close,
          volume: stock_data_response.volume,
          ema9: ema_9,
          ema20: ema_20,
          timestamp: stock_data_response.timestamp,
          high_low_type: None,
      };

      self.stock_data.push_back(incoming_data);

      //Add volume to cached volume
      self.sum_volume += stock_data_response.volume;
  
      
      return (stock_data_response.high, stock_data_response.low, ema_9, ema_20);
  }

    /**
     * Pops from the front of the StockDataVeqDeque if theres 
     */
    pub fn pop_front_if_at_capacity(&mut self) -> Option<StockDatum> {

      if self.stock_data.len() as u32 > self.stock_data_capacity {
         return self.stock_data.pop_front();
      }
      None
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
  fn calculate_emas(stock_data: &VecDeque<StockDatum>, last_close: &f64) -> (f64, f64) {
    let mut ema_9: f64;
    let mut ema_20: f64;
    let length_stock_data: usize = stock_data.len();

    //Ema 9

    if stock_data.len() < 9 {
        ema_9 = 0.0;
    } else {
        let last_stock_idx = stock_data.len() - 1; // These values might underflow, in these cases we directly return 0.0
        let last_ema_9 = stock_data[last_stock_idx].ema9;
        if length_stock_data == 9 {
            ema_9 = Self::calculate_sma(stock_data, 9.0);
        }

        ema_9 = Self::calculate_ema(9.0, last_close, &last_ema_9);
    }

    //EMA 20
    if stock_data.len() < 20 {
        ema_20 = 0.0;
    } else {
        let last_stock_idx = stock_data.len() - 1; // These values might underflow, in these cases we directly return 0.0
        let last_ema_20 = stock_data[last_stock_idx].ema20;
        if length_stock_data == 20 {
            ema_20 = Self::calculate_sma(stock_data, 20.0);
        }
        ema_20 = Self::calculate_ema(20.0, last_close, &last_ema_20);
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
    for idx in 0..length_stock_data {
        sum += stock_data[length_stock_data - 1 - idx].close;
    }

    return sum / sma_length;
  }

  //smoothing: 2.0
  pub fn calculate_ema(period: f64, close: &f64, prev_ema: &f64) -> f64 {
    close * (2.0 / (1.0 + period)) + prev_ema * (1.0 - 2.0 / (1.0 + period))
  }



  /**
  * Checks for abnormal spikes in volume. Systematically ignores the first 3 candles, 
  * and last candle of the day as these are typically higher by nature, delegates to other alerts for this
  */
  fn is_volume_spike(&self, k: f64) -> bool {

        


    if k > 3.0 {
        panic!("Value of k should never be over 3");
    }

    let avg = self.sum_volume as f64 / self.stock_data.len() as f64;
    let volumes: Vec<f64> = self.stock_data.iter()
                                          .map(|datum| datum.volume)
                                          .map(|volume| volume as f64)
                                          .collect();

    let variance = self.get_variance(&volumes, avg);
    let std_dev = variance.sqrt();

    let upper_threshold = avg + k * std_dev;

    let curr_vol = *volumes.last().expect("Volumes array should not be empty");

    curr_vol > upper_threshold
  }

  fn get_average(&self, volumes: &[f64]) -> f64 {
      let sum: f64 = volumes.iter().sum();
      sum / volumes.len() as f64
  }

  fn get_variance(&self, volumes: &[f64], average: f64) -> f64 {
      let mut res = 0.0;
      let length = volumes.len() as f64;

      for volume in volumes {
          let squared_diff = (volume - average).powi(2);
          res += squared_diff;
      }

      res / length
  }

  /**
   * Manages eviction behavior:
   * 1. Removes a price datum from the queue.
   * 2. For the cached sum_volume, decrements it by the volume
   * 
   * TODO refactor stock_data_capacity to be a usize
   */
  pub fn maybe_evict_if_over_cap(&mut self) -> Option<StockDatum> {
    if self.stock_data.len() > self.stock_data_capacity as usize {
      let price_datum =  self.stock_data.pop_front().unwrap();
      self.sum_volume -= price_datum.volume;
      return Some(price_datum);
    }

    return None;
  }

}


/*
 * Represents the value of a high or low.
 */
pub struct HighLowData {
    pub value: f64,
    pub close: f64,
    pub high_low: HighLow,
}

#[derive(PartialEq)]
pub enum HighLow {
    HIGH,
    LOW,
    NEUTRAL,
}
pub enum LongShort {
    LONG,
    SHORT,
}

pub enum Trend {
    SIDEWAYS,
    UPTREND,
    DOWNTREND,
    UNKNOWN,
}