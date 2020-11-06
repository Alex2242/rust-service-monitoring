extern crate yaml_rust;
extern crate chrono;

use yaml_rust::Yaml;
use chrono::prelude::*;

use std::process::Command;

use crate::messaging::message::*;
use crate::probes::*;

pub struct Ping {
    pub host: String,
    pub service: String,
}

pub fn new(service_name: String, conf: &Yaml) -> Ping {
    Ping {
        host: String::from(*(&conf["host"].as_str().unwrap())),
        service: service_name
    }
}

impl Probe for Ping {
    fn run(&self) -> Message {
        let mut command = Command::new("ping");

        command
            .arg("-c 1")
            // timeout after 1s
            .arg("-W 1")
            .arg("-q")
            .arg(self.host.as_str());

        let command_output = command
            .output()
            .expect("ERROR - failed to execute ping");

        // drop non ascii characters to ensure stdout can be sent by mail
        let output_ascii: Vec<u8> = command_output.stdout
            .into_iter()
            .filter(|character|  *character < 123u8)
            .collect();

        let stdout = String::from_utf8(output_ascii).unwrap();

        let mut message = Message::new();

        message.probe = Probes::Ping;
        message.date = Utc::now().to_rfc3339();
        message.service = self.service.clone();
        message.body = stdout;

        let status = &command_output.status;

        if status.success() {
            message.severity = Severity::Info;
            message.header = String::from("Ping successful");
        } else {
            message.severity = Severity::Error;
            match status.code().unwrap() {
                1 => message.header = String::from("Unreachable host"),
                2 => message.header = String::from("Invalid host"),
                _ => message.header = String::from("Unknown error"),
            }
        }

        return message;
    }
}

#[cfg(test)]
mod tests {
    use crate::probes::*;
    use crate::tools::config::read_conf_file;
    use crate::messaging::message::Severity;

    #[test]
    fn test_conf() {
        let yaml = read_conf_file(String::from("tests/ressources/tests.yaml"));
        let h = &yaml["services"]["ping_example"]["probe_spec"];
        let p = super::new(String::from("testService"), &h);

        assert_eq!(p.host, "127.0.0.1");
        assert_eq!(p.service, "testService");
    }

    #[test]
    fn test_ping_localhost() {
        let yaml = read_conf_file(String::from("tests/ressources/tests.yaml"));
        let h = &yaml["services"]["ping_example"]["probe_spec"];
        let p = super::new(String::from("testService"), &h);

        let m = p.run();

        assert_eq!(m.header, String::from("Ping successful"));
        assert!(matches!(m.severity, Severity::Info));
    }
    #[test]

    fn test_ping_unreachable_host() {
        let yaml = read_conf_file(String::from("tests/ressources/tests.yaml"));
        let h = &yaml["services"]["ping_example_unreachable_host"]["probe_spec"];
        let p = super::new(String::from("testService"), &h);

        let m = p.run();

        assert_eq!(m.header, String::from("Unreachable host"));
        assert!(matches!(m.severity, Severity::Error));
    }

    // #[test]
    // fn testMT() {
    //     let yaml = read_conf_file(String::from("tests/ressources/tests.yaml"));
    //     let h = &yaml["services"]["ping_example_unreachable_host"]["probe_spec"];
    //     let p = Box::new(super::new(String::from("testService"), &h));
    //     p.run();
    //     assert!(p.foo());
        
    //     let p2 = super::new(String::from("testService"), &h);
    //     assert!(p2.foo());
    // }
}
