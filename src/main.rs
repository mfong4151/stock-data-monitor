
mod aws_ses;
mod polygon_api;

use std::io::Error;

use crate::aws_ses::send_email::send_email;


#[tokio::main]
async fn main() -> Result<(), Error> {

  let sent_email = send_email().await;

  Ok(())
}
