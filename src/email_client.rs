//use reqwest::Client;
//
////use crate::domain::SubscriberEmail;
//
//#[derive(Clone)]
//pub struct EmailClient {
//    http_client: Client,
//    base_url: String,
//    sender: SubscriberEmail,
//}
//
//impl EmailClient {
//
//    pub fn new(base_url: String, sender: SubscriberEmail) -> Self {
//        Self {
//            http_client: Client::new(),
//            base_url,
//            sender,
//        }
//    }
//
//    pub fn send_email(
//        &self,
//        recipient: SubscriberEmail,
//        subject: &str,
//        text_content: &str,
//        html_content: &str,
//    ) -> Result<(), String> {
//        // send email
//        todo!()
//    }
//}
