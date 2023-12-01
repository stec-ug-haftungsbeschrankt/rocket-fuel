
use sendgrid::SGClient;
use sendgrid::v3::{Personalization, Message, Content, Sender};
use sendgrid::{Destination, Mail};

use crate::email::MailProvider;
use crate::email::Email;


pub struct SendgridMailProvider {
    api_key: String
}

#[async_trait]
impl MailProvider for SendgridMailProvider {
    fn new<T: Into<String>>(api_key: T) -> Self {
        SendgridMailProvider {
            api_key: api_key.into()
        }
    }

    async fn send(&self, email: &Email) -> bool {
        let sg = SGClient::new(self.api_key.clone());

        let mut mail_info = Mail::new()
            .add_from(&email.sender.email)
            .add_subject(&email.subject);

        if let Some(from_name) = &email.sender.name {
            mail_info = mail_info.add_from_name(from_name);
        } 

        for recipient in &email.recipients {
            let mut destination = Destination {
                address: &recipient.email,
                name: ""
            };

            if let Some(name) = &recipient.name {
                destination.name = name;
            }

            mail_info = mail_info.add_to(destination);
        }

        if let Some(html) = &email.html {
            mail_info = mail_info.add_html(html);
        }   

        match sg.send(mail_info).await {
            Err(err) => {
                error!("{}", err);
                false
            },
            Ok(body) => {
                debug!("{:?}", body);
                true
            }
        }
    }
}




pub struct SendgridV3MailProvider {
    api_key: String
}

#[async_trait]
impl MailProvider for SendgridV3MailProvider {
    fn new<T: Into<String>>(api_key: T) -> Self {
        SendgridV3MailProvider {
            api_key: api_key.into()
        }
    }

    async fn send(&self, email: &Email) -> bool {
        let recipients: Vec<sendgrid::v3::Email> = email.recipients.iter().map(|r| sendgrid::v3::Email::new(&r.email).set_name(r.name.as_ref().unwrap_or(&"".to_string()))).collect();
        let mut personalization = Personalization::new(recipients[0].clone());
    
        for recipient in &recipients[1..] {
            personalization = personalization.add_to(recipient.clone());
        }
    
        let message = Message::new(sendgrid::v3::Email::new(&email.sender.email).set_name(email.sender.name.as_ref().unwrap()))
            .set_subject(&email.subject)
            .add_content(
                Content::new()
                    .set_content_type("text/html")
                    .set_value(email.html.as_ref().unwrap()),
            )
            .add_personalization(personalization);
    
        let sender = Sender::new(self.api_key.clone());
        let response = sender.send(&message).await;

        match response {
            Err(err) => {
                error!("{}", err);
                false
            },
            Ok(body) => {
                debug!("{:?}", body);
                true
            }
        }
    }
}



