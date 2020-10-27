use crate::probes::*;
use crate::tools::config::read_conf_file;
use crate::messaging::mailer;
use crate::messaging::mailer::Mailer;
use crate::messaging::message::Severity;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use std::thread;
use std::time::Duration;
// use std::convert::TryInto;

pub struct Daemon {
    delay: u64,
    delay_at_startup: u64,
    debug: bool,
    mailer: Mailer,
    probes: Vec<Box<dyn Probe>>,
}

impl Daemon {
    pub fn new(conf_filename: String) -> Self {
        let yaml = read_conf_file(conf_filename);

        let mut probes: Vec<Box<dyn Probe>> = Vec::new();

        for service in yaml["services"].as_hash().unwrap().iter() {
            let name = String::from(service.0.as_str().unwrap());
            let service_conf = service.1;
            // println!("{:?}"*, service_conf["probe"]);
            let probe_type = service_conf["probe"].as_str().unwrap();
            
            probes.push(match probe_type {
                "ping" => Box::new(ping::new(name, &service_conf["probe_spec"])),
                "https" => Box::new(https::new(name, &service_conf["probe_spec"])),
                _ => panic!("Unknown probe type"),
            })
        }

        Daemon {
            delay: yaml["common"]["delay"].as_i64().unwrap_or(600) as u64,
            delay_at_startup: yaml["common"]["delay_at_startup"]
                .as_i64().unwrap_or(30) as u64,
            debug: yaml["common"]["debug"].as_bool().unwrap_or(false),
            probes: probes,
            mailer: mailer::new(&yaml["notifications"]["email"]),
        }
    }

    pub fn run(&self, run: Arc<AtomicBool>) {
        thread::sleep(Duration::from_secs(self.delay_at_startup));

        let mut count: u64 = 0;

        loop {
            for probe in self.probes.iter() {
                let message = probe.run();
                
                if self.debug {
                    println!("{:?}\n\n", message);
                } else {
                    match message.severity {
                        Severity::Error => self.mailer.send_message(message),
                        Severity::Warning => self.mailer.send_message(message),
                        _ => continue,
                    }
                }
            }

            while count < self.delay {
                if !run.load(Ordering::SeqCst) { break }

                thread::sleep(Duration::from_secs(1))
            }

            if !run.load(Ordering::SeqCst) { break }
            count = 0;
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::panic;

    #[test]
    fn test_conf() {
        Daemon::new(String::from("tests/ressources/tests.yaml"));
    }

    #[test]
    fn test_conf_nofile() {
        let failing = panic::catch_unwind(|| {
            Daemon::new(String::from("tests/ressources/none.yaml"));
        });

        assert!(failing.is_err());
    }
}