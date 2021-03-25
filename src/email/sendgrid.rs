
use sendgrid::SGClient;
use sendgrid::{Destination, Mail};

use crate::email::MailProvider;
use crate::email::Email;


pub struct SendgridMailProvider {
    api_key: String
}


impl MailProvider for SendgridMailProvider {
    fn new<T: Into<String>>(api_key: T) -> Self {
        SendgridMailProvider {
            api_key: api_key.into()
        }
    }

    fn send(&self, email: &Email) -> bool {
        let sg = SGClient::new(self.api_key.clone());

        let mut mail_info = Mail::new()
            .add_from(&email.sender.email)
            .add_subject(&email.subject);

        if let Some(from_name) = &email.sender.name {
            mail_info = mail_info.add_from_name(&from_name);
        } 

        for recipient in &email.recipients {
            let mut destination = Destination {
                address: &recipient.email,
                name: ""
            };

            if let Some(name) = &recipient.name {
                destination.name = &name;
            }

            mail_info = mail_info.add_to(destination);
        }

        if let Some(html) = &email.html {
            mail_info = mail_info.add_html(&html);
        }   

        match sg.send(mail_info) {
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
