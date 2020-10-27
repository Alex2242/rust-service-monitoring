extern crate openssl;
extern crate yaml_rust;
extern crate chrono;

use openssl::ssl::{SslMethod, SslConnector};
use openssl::asn1::Asn1Time;
use std::net::TcpStream;

use std::time::SystemTime;
use chrono::prelude::*;
use std::convert::TryInto;

use yaml_rust::Yaml;

use crate::probes::*;
use crate::messaging::message::*;

#[derive(Debug)]
pub struct Https {
    pub host: String,
    pub service: String
}

pub fn new(service_name: String, conf: &Yaml) -> Https {
    Https {
        host: String::from(*(&conf["host"].as_str().unwrap())),
        service: service_name
    }
}

fn days_before_expiration(cert: &openssl::x509::X509) -> i32 {
    let unix_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let asn1_time = Asn1Time::from_unix(unix_time.try_into().unwrap()).unwrap();

    let not_after = cert.not_after();

    return asn1_time.diff(not_after).unwrap().days;
}

impl Probe for Https {
    fn run(&self) -> Message {
        let mut message = Message::new();

        message.probe = Probes::Https;
        message.date = Utc::now().to_rfc3339();
        message.service = self.service.clone();

        let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
        let stream = TcpStream::connect(format!("{}:443", self.host)).unwrap();

        let mut stream = match connector.connect(self.host.as_str(), stream) {
            Ok(result) => result,
            Err(err) => {
                message.severity = Severity::Error;
                message.header = String::from("critical error while connecting to https service");
                message.body = String::from(format!("{}",err));

                return message;
            }
        };

        let cert = stream.ssl().peer_certificate().unwrap();

        stream.shutdown().unwrap();

        let days = days_before_expiration(&cert);

        if days > 0 && days < 7 {
            message.severity = Severity::Warning;
            message.header = String::from(format!("Cerficate expires soon, {} days remaining", days));
        } else if days > 0 && days < 2  {
            message.severity = Severity::Error;
            message.header = String::from(format!("Cerficate is about to expire, {} days remaining", days));
        } else if days < 0 {
            message.severity = Severity::Error;
            message.header = String::from(format!("Certificate expired {} days ago", days));
        } else {
            message.severity = Severity::Info;
            message.header = String::from(format!("Cerficate expires in {} days", days));
        }

        return message;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_asn1_time() {
        let a100 = Asn1Time::from_unix(100i64.try_into().unwrap()).unwrap();
        let a1000 = Asn1Time::from_unix(1000i64.try_into().unwrap()).unwrap();
        let d1 = a100.diff(&a1000).unwrap().secs;

        assert_eq!(d1, 900);
    }

    #[test]
    fn test_expiration() {
        let mut file = File::open("tests/ressources/test-cert.pem").unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let cert = openssl::x509::X509::from_pem(contents.as_bytes()).unwrap();
        days_before_expiration(&cert);
    }
}