use std::{collections::{HashMap, VecDeque}, hash::Hash};

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
 * TODO: Refactor to make OHLC be VecDeq<PriceDatum> instead of individual VecDeq<f64>
 * In the future we can consider unfolding them to be individual queus
 *
 * An internal representation of a data corresponding to a stock
 * The highs, data for the OHLCV at index i all correspond to each otherj
 *
 * name, & 'a str: The ticker name
 * opens, Vec<f64> :  opens
 * highs, Vec<f64> :  highs
 * lows, Vec<f64> :   lows
 * closes, Vec<f64>:  closes
 * volumes, Vec<u32>: volumes
 * ema_9s, Vec<f64>: The ema 9s *does this need to be a single value instead?
 * ema_20s, Vec<f64>: The ema 20s *does this need to be a single value instead?
 * high_candles:
 * low_candles:
 * resistances, VecDeque<f64>: The resistances of the current timeframe
 * supports, VecDeque<f64>: The supports of the current timeframe
 * daily_resistances, VecDeque<f64>: daily chart resistances, not mutable
 * daily_supports, VecDeque<f64>: daily chart supports, not mutable
 * high_low_queue, VecDeque<StockDatum> a smaller VecDeque  that contains only the points necessary to determine trend
 * current_trend, Trend : The current overall trend of the equity
 */
pub struct StockData<'a> {
    pub name: &'a str,
    pub stock_data: VecDeque<StockDatum>,
    pub daily_resistances: VecDeque<f64>,
    pub daily_supports: VecDeque<f64>,
    pub high_low_queue: VecDeque<&'a StockDatum>,
    pub volume_attrs: VolumeAttr,
    pub current_trend: Trend,
}

impl<'a> StockData<'a> {
    /*
     *    Instantiates an empty new struct
     */
    pub fn new(name: &'a str) -> StockData<'a> {
        StockData {
            name,
            stock_data: VecDeque::new(),
            daily_resistances: VecDeque::new(),
            daily_supports: VecDeque::new(),
            high_low_queue: VecDeque::new(),
            volume_attrs: VolumeAttr::new(),
            current_trend: Trend::UNKNOWN,
        }
    }

    pub fn set_stock_data(&mut self, table: &Vec<RawDataTableRow>){
        let stock_data_points: Vec<StockDatum> =  table.iter().map(|row|{
          return StockDatum::from_hashmap(row)
        }).collect();
        self.stock_data = VecDeque::from(stock_data_points);
    }

    pub fn analyze(&mut self) {
        self.determine_high_low_type();
        self.determine_high_low_type();
        self.update_high_low_queue();
        self.current_trend = self.parse_trend();
    }

    fn determine_high_low_type(&mut self) {
        let last_idx = self.stock_data.len() - 1;
        let last_price_datum = &mut self.stock_data[last_idx];
        // if self.is_high(&self.stock_data) {
        //     last_price_datum.high_low_type = Some(HighLow::HIGH);
        // } else if self.is_low(&self.stock_data) {
        //     last_price_datum.high_low_type = Some(HighLow::LOW);
        // } else {
        //     last_price_datum.high_low_type = None;
        // }
    }
    fn update_high_low_queue(&mut self) {
        // ...
    }

    fn is_high(price_data: &VecDeque<StockDatum>) -> bool {
        return true;
    }

    fn is_low(price_data: &VecDeque<StockDatum>) -> bool {
        return true;
    }

    /**
     * Parses the trend of the stock
     *
     * @param high_low_queue a VecDeque<StockData> of the queue
     * @return a value in the Trend enum
     */
    fn parse_trend(&self) -> Trend {
        if self.high_low_queue.len() < 4 {
            return Trend::UNKNOWN;
        }

        let first = self.high_low_queue[0];
        let second = self.high_low_queue[1];
        let third = self.high_low_queue[2];
        let fourth: &StockDatum = self.high_low_queue[3]; //Fourth is mutable because we double account for trend, we update the trend here

        let mut trend_direction = Trend::SIDEWAYS;

        if first.high_low_type == Some(HighLow::HIGH) {
            if first.high < third.high && second.low < fourth.low {
                trend_direction = Trend::UPTREND;
            } else if first.high >= third.high && second.low >= fourth.low {
                trend_direction = Trend::DOWNTREND;
            }
        } else if first.high_low_type == Some(HighLow::LOW) {
            if first.low >= third.low && second.high >= fourth.high {
                trend_direction = Trend::DOWNTREND;
            } else if first.low < third.low && second.high < fourth.high {
                trend_direction = Trend::UPTREND;
            }
        }

        return trend_direction;
    }

    

    pub fn format_flat_analysis_data(flat_data: HashMap<String, f64>) {
      let early_volume: String =  "early_volume".to_string();
      let mid_volume: String =  "mid_volume".to_string();
      let end_volume: String =  "end_volume".to_string();
    
      


    }

}

/*
 * Represents a stock position that we hold
 */
pub struct StockPosition {
    pub name: String,
    pub entry: f64,
    pub stop: f64,
    pub long_short: LongShort,
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
