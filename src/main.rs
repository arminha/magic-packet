
extern crate magic_packet;

fn main() {
    let mac = [1, 2, 3, 4, 5, 6];
    let address = "192.168.1.255:9";
    magic_packet::send_magic_packet(&mac, address).unwrap();
}
