
pub mod magic {

    use std::io::Error;
    use std::net::UdpSocket;

    fn create_magic_packet(mac: &[u8; 6]) -> [u8; 102] {
        let mut buffer = [0; 102];
        for i in 0..6 {
            buffer[i] = 0xff;
        }
        for n in 0..16 {
            let start = 6 + n * 6;
            &buffer[start..start + 6].clone_from_slice(mac);
        }
        buffer
    }

    pub fn send(mac: &[u8; 6]) -> Result<(), Error> {
        let socket = try!(UdpSocket::bind("127.0.0.1:34254"));
        let buffer = create_magic_packet(mac);
        try!(socket.send_to(&buffer, "255.255.255.255:0"));
        Ok(())
    }

    #[cfg(test)]
    #[test]
    fn create_valid_magic_packet() {
        let mac = [1, 2, 3, 4, 5, 6];
        let buffer = create_magic_packet(&mac);
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
}
