mod messaging;
mod probes;
mod tools;
mod daemon;

extern crate clap;
extern crate ctrlc;

use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::path::Path;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Rust Sevice Monitoring")
        .version("0.1.0")
        // .author("")
        // .about("LONG DESCRIPTION")
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .about("Sets a custom config file")
            .takes_value(true))
        .get_matches();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    running.load(Ordering::SeqCst);

    let config_location: String = {
        if matches.is_present("config") {
            matches.value_of_t("config").unwrap()
        } else if Path::new("/etc/rsm.yaml").exists() {
            String::from("/etc/rsm.yaml")
        } else if Path::new("rsm.yaml").exists() {
            String::from("rsm.yaml")
        } else {
            println!("No configuration file provided, aborting !");
            exit(1);
        }
    };

    let daemon = daemon::Daemon::new(config_location);

    daemon.run(running);
}
