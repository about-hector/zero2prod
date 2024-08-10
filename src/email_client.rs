use crate::domain::SubscriberEmail;
use aws_sdk_sesv2::{types::{Body, Content, Destination, EmailContent, Message}, Client};

#[derive(Clone)]
pub struct EmailClient {
    http_client: Client,
    //base_url: String,
    sender: SubscriberEmail,
}

impl EmailClient {

    pub async fn new(
        //base_url: String,
        sender: SubscriberEmail
    ) -> Self {
        let client = crate::aws_client::create_aws_client().await;
        Self {
            http_client: client,
            sender,
        }
    }

    pub async fn send_email(
        &self,
        recipient: String,//SubscriberEmail,
        subject: &str,
        //html_content: &str,
        text_content: &str,
    ) -> Result<(), String> {

        let mut dest: Destination = Destination::builder().build();
        dest.to_addresses = Some(vec![recipient]);

        let subject_content = Content::builder()
            .data(subject)
            .charset("UTF-8")
            .build()
            .expect("Failed to build subject content");

        let body_content = Content::builder()
            .data(text_content)
            .charset("UTF-8")
            .build()
            .expect("Failed to build body content");


        let body = Body::builder()
            .text(body_content)
            .build();

        let msg = Message::builder()
            .subject(subject_content)
            .body(body)
            .build();

        let email_content = EmailContent::builder().simple(msg).build();

        self.http_client
            .send_email()
            .from_email_address(self.sender.as_ref())
            .destination(dest)
            .content(email_content)
            .send()
            .await
            .map_err(|e| format!("Failed to send email: {:?}", e))?;

        Ok(())
    }
}
