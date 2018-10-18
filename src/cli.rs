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
use clap::{App, Arg};
use regex::Regex;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn build_cli() -> App<'static, 'static> {
    App::new("magic-packet")
        .version(VERSION)
        .about("Sends a magic packet to a MAC address")
        .arg(
            Arg::with_name("MAC_ADDR")
                .help("The MAC address of the target")
                .required(true)
                .index(1)
                .validator(is_mac),
        )
}

fn is_mac(val: String) -> Result<(), String> {
    let mac_regex = Regex::new("^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$").unwrap();
    if mac_regex.is_match(&val) {
        Ok(())
    } else {
        Err(format!(
            "\"{}\" is not a MAC address. Use format 01:23:89:AB:CD:EF or 01-45-67-AB-CD-EF.",
            val
        ))
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn valid_mac() {
        assert_eq!(Ok(()), super::is_mac("01:23:89:AB:CD:EF".to_string()));
        assert_eq!(Ok(()), super::is_mac("01-45-67-AB-CD-EF".to_string()));
    }

    #[test]
    fn invalid_mac() {
        let error_msg = "\"01:23:89:AB:CD\" is not a MAC address. \
                         Use format 01:23:89:AB:CD:EF or 01-45-67-AB-CD-EF."
            .to_owned();
        assert_eq!(Err(error_msg), super::is_mac("01:23:89:AB:CD".to_string()));
    }
}
