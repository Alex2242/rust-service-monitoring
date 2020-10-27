extern crate lettre;
extern crate yaml_rust;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use yaml_rust::Yaml;

use crate::messaging::message;

pub struct Mailer {
    sender_address: String,
    recipient_address: String,
    transport: SmtpTransport,
}

pub fn new(mailer_config: &Yaml) -> Mailer {
        let username = String::from(*(&mailer_config["username"].as_str().unwrap()));
        let password = String::from(*(&mailer_config["password"].as_str().unwrap()));
        let relay = *(&mailer_config["relay"].as_str().unwrap());

        let creds = Credentials::new(username, password);
        
        Mailer {
            sender_address: String::from(*(&mailer_config["sender_address"].as_str().unwrap())),
            recipient_address: String::from(*(&mailer_config["recipient_address"].as_str().unwrap())),
            transport: SmtpTransport::starttls_relay(relay)
                .unwrap()
                .credentials(creds.clone())
                .build(),
        }
    }

impl Mailer {
    pub fn send_message(&self, message: message::Message) {
        if !message.body.is_ascii() {
            panic!("Non ascii body");
        }

        let email = Message::builder()
            .from(self.sender_address.as_str().parse().unwrap())
            .to(self.recipient_address.as_str().parse().unwrap())
            .subject(format!("[Monitoring/{}/{}] {} {}", message.severity.to_str(),
                message.probe.to_str(), message.service, message.header))
            .body(message.to_str().to_ascii_lowercase())
            .unwrap();


        self.transport.send(&email).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tools::config::read_conf_file;

    #[test]
    fn test_config() {
        let yaml = read_conf_file(String::from("tests/ressources/tests.yaml"));

        let mail_config = &yaml["notifications"]["email"];

        let _ = new(mail_config);
    }
}