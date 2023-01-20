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
use clap::{Arg, Command};
use regex::Regex;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn build_cli() -> Command {
    Command::new("magic-packet")
        .version(VERSION)
        .about("Sends a magic packet to a MAC address")
        .arg(
            Arg::new("MAC_ADDR")
                .help("The MAC address of the target")
                .required(true)
                .index(1)
                .value_parser(parse_mac),
        )
}

fn parse_mac(val: &str) -> Result<String, String> {
    let mac_regex = Regex::new("^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$").unwrap();
    if mac_regex.is_match(val) {
        Ok(val.to_owned())
    } else {
        Err(format!(
            "\"{val}\" is not a MAC address. Use format 01:23:89:AB:CD:EF or 01-45-67-AB-CD-EF."
        ))
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn valid_mac() {
        assert_eq!(
            Ok("01:23:89:AB:CD:EF".to_owned()),
            super::parse_mac("01:23:89:AB:CD:EF")
        );
        assert_eq!(
            Ok("01-45-67-AB-CD-EF".to_owned()),
            super::parse_mac("01-45-67-AB-CD-EF")
        );
    }

    #[test]
    fn invalid_mac() {
        let error_msg = "\"01:23:89:AB:CD\" is not a MAC address. \
                         Use format 01:23:89:AB:CD:EF or 01-45-67-AB-CD-EF."
            .to_owned();
        assert_eq!(Err(error_msg), super::parse_mac("01:23:89:AB:CD"));
    }
}
