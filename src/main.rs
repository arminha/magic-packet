/*
Copyright (C) 2016  Armin Häberling

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <http://www.gnu.org/licenses/>
*/
#![forbid(unsafe_code)]

extern crate clap;
extern crate regex;

use regex::Regex;

mod cli;
mod magic;

fn main() {
    let matches = cli::build_cli().get_matches();
    let mac = matches.value_of("MAC_ADDR").unwrap();
    send(mac);
}

fn send(mac: &str) {
    eprintln!("Sending magic packet to {}", mac);
    let mac = read_mac(mac);
    let address = "255.255.255.255:9";
    magic::send_magic_packet(&mac, address).unwrap();
}

fn read_mac(val: &str) -> [u8; 6] {
    let mac_regex = Regex::new(
        "^([0-9A-Fa-f]{2})[:-]([0-9A-Fa-f]{2})[:-]([0-9A-Fa-f]{2})[:-]\
         ([0-9A-Fa-f]{2})[:-]([0-9A-Fa-f]{2})[:-]([0-9A-Fa-f]{2})$",
    )
    .unwrap();
    let caps = mac_regex.captures(val).unwrap();
    let items = (1..7)
        .map(|x| caps.get(x).unwrap())
        .map(|x| u8::from_str_radix(x.as_str(), 16).unwrap())
        .collect::<Vec<_>>();
    let mut mac = [0u8; 6];
    mac[..6].copy_from_slice(&items[..6]);
    mac
}

#[cfg(test)]
mod test {

    #[test]
    fn read_valid_mac() {
        assert_eq!(
            [1, 35, 137, 171, 205, 239],
            super::read_mac("01:23:89:AB:CD:EF")
        );
        assert_eq!(
            [1, 69, 103, 171, 205, 239],
            super::read_mac("01-45-67-AB-CD-EF")
        );
    }
}
