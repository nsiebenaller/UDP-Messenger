use std::net::UdpSocket;
use std::str;
use std::{thread, time};

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:11111").unwrap();
    let mut buf = [0; 10];

    loop {
        let (amt, src) = socket.recv_from(&mut buf).unwrap();
        let rec = str::from_utf8(&buf[0 .. amt]).unwrap();
        println!("Recieved: {:?} from {}", rec, src);

        let wait_time = time::Duration::from_millis(10000);
        thread::sleep(wait_time);

        let return_str = format!("{}{}", rec, "-recieved");
        let bytes = return_str.as_bytes();
        socket.send_to(bytes, src).unwrap();
    }
}
