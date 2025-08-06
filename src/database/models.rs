use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::stocks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Stock{
  pub id: i32,
  pub ticker: String,
  pub open: f64,
  pub close: f64,
  pub high: f64,
  pub low: f64,
  pub ema_9: f64,
  pub timestamp: i64,
}