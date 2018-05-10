use clap::{Arg, App};
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
