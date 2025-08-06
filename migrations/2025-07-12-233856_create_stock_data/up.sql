-- Your SQL goes here
CREATE TABLE stocks(
   id SERIAL PRIMARY KEY,
   ticker VARCHAR NOT NULL,
   open FLOAT NOT NULL,
   close FLOAT NOT NULL,
   high FLOAT NOT NULL,
   low FLOAT NOT NULL,
   ema_9 FLOAT NOT NULL,
   timestamp BIGINT NOT NULL,
   created_at TIMESTAMPTZ DEFAULT now()
)