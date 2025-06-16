use chrono::{DateTime, Datelike, Local, Timelike, Weekday, NaiveDateTime};

/**
 * Hard codes a time offset which allows us to test off hours 
 * 
 * This method is used to get around a 48 hour delayed data limitation with apis
 * As such, this should only be used with testing.
 */
pub fn manage_offset()-> i64 {

  let now = Local::now();
  let curr_day_of_week = now.date_naive().weekday();
  let mut offsetted_time;
  let now_hour = now.hour() as u32;
  const MINUTES_TO_MILIS: i64 = 60  * 1000;
  let is_testing_weekend: bool =  vec![Weekday::Mon, Weekday::Tue].contains(&curr_day_of_week);

  //1. Go back 48 hours by default
  let default_offsetted_time = now.timestamp_millis() - MINUTES_TO_MILIS * 60 * 48;
  offsetted_time = default_offsetted_time;

  //2. Set the weekday to two days ago  if we're testing on monday or tuesday
  offsetted_time -= if is_testing_weekend {MINUTES_TO_MILIS * 60 * 48} else {0};

  //3. If we are testing after market hours (13) then automatically rewind 
  // by at least 6 hours
  // If we are at 19 hours, rewind by 12

  offsetted_time -= if 13 <= now_hour && now_hour < 19 {
    6
  } else if 19 < now_hour{
    12
  } else {
    0
  } * MINUTES_TO_MILIS * 60;

  println!("Pushing time back at least 48 hours ago.\n If you've been pushed back to a holiday, you might need to hardcode a date around this.");
  parse_offsetted_time(offsetted_time);

  return offsetted_time;
}


fn parse_offsetted_time(offsetted_time: i64) {
    let naive = NaiveDateTime::from_timestamp_millis(offsetted_time)
        .expect("Invalid timestamp in milliseconds");

    let datetime: DateTime<Local> = DateTime::<Local>::from_utc(naive, *Local::now().offset());

    println!("Offsetted time (local): {:?} {:?}/{:?},  {:?}",
      datetime.date_naive().weekday(),
      datetime.month(),
      datetime.day(),  datetime.time());
}
