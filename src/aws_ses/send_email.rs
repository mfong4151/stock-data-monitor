use aws_config::BehaviorVersion;
use aws_sdk_sesv2::{types::{Body, Content, Destination, EmailContent, Message}, Client, Error };
use dotenv::dotenv;

const EMAIL_ADDRESS:&str = "mfong415@gmail.com";
pub async fn send_email() -> Result<(), Error> {

  dotenv().ok();

  let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
  let client = Client::new(&config);
  let destination = Destination::builder().to_addresses(EMAIL_ADDRESS).build();


  let subject = Content::builder().data("Hellow world").charset("UTF-8").build().unwrap();
  let body_content = Content::builder().data("hi").charset("UTF-8").build().unwrap();    

  let message = Message::builder()
    .subject(subject)
    .body(Body::builder().text(body_content).build())
    .build();


  let email_content = EmailContent::builder().simple(message).build();


  client
    .send_email()
    .from_email_address(EMAIL_ADDRESS) 
    .destination(destination)
    .content(email_content)
    .send()
    .await?;

  println!("Succesfully sent message about stock movement");
  Ok(())

}