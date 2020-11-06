use crate::probes::*;
use crate::tools::config::read_conf_file;
use crate::messaging::*;
use crate::messaging::mailer::Mailer;
use crate::messaging::message::*;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use std::thread;
use std::time::Duration;

pub struct Daemon {
    delay: u64,
    delay_at_startup: u64,
    error_repeat_period: u64,
    warning_repeat_period: u64,
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
            error_repeat_period: yaml["common"]["error_repeat_period"]
                .as_i64().unwrap_or(6) as u64,
            warning_repeat_period: yaml["common"]["warning_repeat_period"]
                .as_i64().unwrap_or(6 * 24) as u64,
            debug: yaml["common"]["debug"].as_bool().unwrap_or(false),
            probes: probes,
            mailer: mailer::new(&yaml["notifications"]["email"]),
        }
    }

    pub fn run(&self, run: Arc<AtomicBool>) {
        thread::sleep(Duration::from_secs(self.delay_at_startup));

        let n_probes = self.probes.len();
        let mut count: u64 = 0;
        let mut period_counters: Vec<u64> = vec![0; n_probes];
        let mut message_cache: Vec<Message> = vec![Message::new(); n_probes];

        loop {
            for i in 0..n_probes {
                let message = self.probes[i].run();
                
                // guard block for debug
                if self.debug {
                    println!("{:?}\n\n", message);
                    continue;
                }

                if matches!(message.severity, Severity::Info) {
                    continue;
                }

                if message != message_cache[i] {
                    period_counters[i] = 0;
                    message_cache[i] = message.clone();
                    self.mailer.send_message(message);
                } else {
                    let should_send = match message.severity {
                        Severity::Error => period_counters[i] >= self.error_repeat_period,
                        Severity::Warning => period_counters[i] >= self.warning_repeat_period,
                        _ => panic!("Impossible"),
                    };

                    period_counters[i] += 1;

                    if should_send {
                        self.mailer.send_message(message);
                    }
                }
            }

            // these ten or so lines are just meant to support graceful SIGTERM 
            while count < self.delay {
                if !run.load(Ordering::SeqCst) { break }

                thread::sleep(Duration::from_secs(1));
                count += 1;
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