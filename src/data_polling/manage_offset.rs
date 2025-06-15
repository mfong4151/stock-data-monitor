use chrono::{Datelike, Local, Timelike, Weekday};

/**
 * Hard codes a time offset which allows us to test off hours 
 * 
 * The assumption is that the current
 */
pub fn manage_offset()-> (i64, bool){

  let now = Local::now();
  let curr_day_of_week = now.date_naive().weekday();
  let mut res;
  let now_hour = now.hour() as u32;
  const MINUTES_TO_MILIS: i64 = 60  * 1000;
  let is_testing_weekend: bool =  vec![Weekday::Mon, Weekday::Tue].contains(&curr_day_of_week);
  println!("{:?}, {:?}", now.hour(), now.minute());
  println!("{:?}", curr_day_of_week);
  

  //1. Go back 48 hours by default
  let default_offsetted_time = now.timestamp_millis() - MINUTES_TO_MILIS * 60 * 48;
  res = default_offsetted_time;

  //2. Set the weekday to two days ago  if we're testing on monday or tuesday
  res -= if is_testing_weekend {MINUTES_TO_MILIS * 60 * 48} else {0};

  //3. If we are testing after market hours (13) then automatically rewind 
  // by at least 6 hours
  // If we are at 19 hours, rewind by 12
  res -= if 13 < now_hour && now_hour < 19 {
    6
  } else if 19 < now_hour{
    12
  } else {
    0
  } * MINUTES_TO_MILIS * 60;
  let is_using_offest = default_offsetted_time == res;

  println!("Pushing time back at least 48 hours ago.\n If you've been pushed back to a holiday, you might need to hardcode a date around this.");

  (res, is_using_offest)
}