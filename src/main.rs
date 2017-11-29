
extern crate clap;
extern crate regex;

use regex::Regex;

mod magic;
mod cli;

fn main() {
    let matches = cli::build_cli().get_matches();
    let mac = matches.value_of("MAC_ADDR").unwrap();
    send(mac);
}

fn send(mac: &str) {
    let mac = read_mac(mac);
    let address = "255.255.255.255:9";
    magic::send_magic_packet(&mac, address).unwrap();
}

fn read_mac(val: &str) -> [u8; 6] {
    let mac_regex = Regex::new(
        "^([0-9A-Fa-f]{2})[:-]([0-9A-Fa-f]{2})[:-]([0-9A-Fa-f]{2})[:-]\
                        ([0-9A-Fa-f]{2})[:-]([0-9A-Fa-f]{2})[:-]([0-9A-Fa-f]{2})$",
    ).unwrap();
    let caps = mac_regex.captures(val).unwrap();
    let items = (1..7)
        .map(|x| caps.get(x).unwrap())
        .map(|x| u8::from_str_radix(x.as_str(), 16).unwrap())
        .collect::<Vec<_>>();
    let mut mac = [0u8; 6];
    for i in 0..6 {
        mac[i] = items[i];
    }
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
