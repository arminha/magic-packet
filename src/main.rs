
extern crate clap;
extern crate regex;

use clap::{Arg, App};
use regex::Regex;

mod magic;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("magic-packet")
                        .version(VERSION)
                        .arg(Arg::with_name("MAC_ADDR")
                                .help("The MAC address of the target")
                                .required(true)
                                .index(1)
                                .validator(is_mac))
                        .get_matches();
    let mac = matches.value_of("MAC_ADDR").unwrap();
    let mac = read_mac(mac);
    let address = "255.255.255.255:9";
    magic::send_magic_packet(&mac, address).unwrap();
}

fn is_mac(val: String) -> Result<(), String> {
    let mac_regex = Regex::new("^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$").unwrap();
    if mac_regex.is_match(&val) {
        Ok(())
    } else {
        Err("\"".to_string() + &val + "\" is not a MAC address.")
    }
}

fn read_mac(val: &str) -> [u8; 6] {
    let mac_regex = Regex::new("^([0-9A-Fa-f]{2})[:-]([0-9A-Fa-f]{2})[:-]([0-9A-Fa-f]{2})[:-]\
                        ([0-9A-Fa-f]{2})[:-]([0-9A-Fa-f]{2})[:-]([0-9A-Fa-f]{2})$").unwrap();
    let caps = mac_regex.captures(val).unwrap();
    let items = (1..7).map(|x| caps.at(x).unwrap())
                      .map(|x| u8::from_str_radix(x, 16).unwrap())
                      .collect::<Vec<_>>();
    let mut mac = [0u8; 6];
    for i in 0..6 {
        mac[i] = items[i];
    }
    mac
}
