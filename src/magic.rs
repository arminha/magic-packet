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
use std::io::Error;
use std::net::{ToSocketAddrs, UdpSocket};

fn create_magic_packet(mac: &[u8; 6]) -> [u8; 102] {
    let mut buffer = [0; 102];
    for i in 0..6 {
        buffer[i] = 0xff;
    }
    for n in 0..16 {
        let start = 6 + n * 6;
        buffer[start..start + 6].clone_from_slice(mac);
    }
    buffer
}

pub fn send_magic_packet<A: ToSocketAddrs>(mac: &[u8; 6], addr: A) -> Result<(), Error> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;
    let buffer = create_magic_packet(mac);
    socket.send_to(&buffer, addr)?;
    Ok(())
}

#[cfg(test)]
mod test {

    #[test]
    fn create_valid_magic_packet() {
        let mac = [1, 2, 3, 4, 5, 6];
        let buffer = super::create_magic_packet(&mac);
        println!("{:?}", &buffer[..]);
        assert_eq!(102, buffer.len());
        assert_eq!(255, buffer[3]);
        assert_eq!(1, buffer[6]);
        assert_eq!(3, buffer[8]);
        assert_eq!(6, buffer[11]);
        assert_eq!(1, buffer[96]);
        assert_eq!(3, buffer[98]);
        assert_eq!(6, buffer[101]);
    }

    #[test]
    fn broadcast_magic_packet() {
        let mac = [1, 2, 3, 4, 5, 6];
        assert_eq!(
            (),
            super::send_magic_packet(&mac, "255.255.255.255:9").unwrap()
        );
    }
}
