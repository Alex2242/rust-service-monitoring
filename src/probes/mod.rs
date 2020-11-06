pub mod https;
pub mod ping;

extern crate yaml_rust;

use super::messaging::message::Message;

#[derive(Debug, Clone)]
pub enum Probes {
    Ping,
    Https,
}

impl Default for Probes {
    fn default() -> Self { Probes::Ping }
}

impl Probes {
    pub fn to_str(&self) -> String {
        match self {
            Probes::Ping => String::from("ping"),
            Probes::Https => String::from("https"),
        }
    }
}

pub trait Probe {
    // fn spawn(service: String, conf: &Yaml) -> Self;
    fn run(&self) -> Message;
}

// pub trait ProbeCommon {
//     fn foo(&self) -> bool;

// }

// impl<T> ProbeCommon for T where T: Probe + ?Sized {
//     fn foo(&self) -> bool {
//         self.bla
//     }
// }
