pub mod sendgrid;


#[derive(Debug, Clone)]
pub struct Contact {
    email: String,
    name: Option<String>,
}

impl Contact {
    pub fn new<T: Into<String>>(email: T, name: T) -> Self {
        Contact { email: email.into(), name: Some(name.into()) }
    }
}

impl<T: Into<String>> From<T> for Contact {
    fn from(email: T) -> Self {
        Contact { email: email.into(), name: None }
    }
}



#[derive(Debug, Clone)]
pub struct Email {
    sender: Contact,
    recipients: Vec<Contact>,
    subject: String,
    html: Option<String>
}

impl Email {
    pub fn new(sender: Contact) -> Self {
        Email {
            sender,
            recipients: Vec::new(),
            subject: "".to_string(),
            html: None,
        }
    }

    pub fn add_recipient<T: Into<Contact>>(mut self, recipient: T) -> Self {
        self.recipients.push(recipient.into());
        self
    }

    pub fn set_subject<T: Into<String>>(mut self, subject: T) -> Self {
        self.subject = subject.into();
        self
    }

    pub fn set_html<T: Into<String>>(mut self, html: T) -> Self {
        self.html = Some(html.into());
        self
    }
}


#[async_trait]
pub trait MailProvider {
    fn new<T: Into<String>>(api_key: T) -> Self;

    async fn send(&self, email: &Email) -> bool;
}
