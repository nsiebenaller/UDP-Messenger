use std::net::UdpSocket;
use std::io::stdin;
use std::str;
use std::thread;

fn main() {

    let socket = UdpSocket::bind("127.0.0.1:55555").unwrap();
    register_reciever(socket.try_clone().unwrap());

    loop {
        println!("Say Something: ");

        let mut input = String::new();       

        stdin().read_line(&mut input).unwrap();
        let bytes = input.trim().as_bytes();

        socket.send_to(bytes, "127.0.0.1:11111").unwrap();
        println!("-----");
    }

}

fn register_reciever(socket: UdpSocket) {
    thread::spawn(move || {
        let mut buf = [0; 50];
        loop {
            let (amt, _src) = socket.recv_from(&mut buf).unwrap();
            let rec = str::from_utf8(&buf[0 .. amt]).unwrap();
            println!("Recieved: {:?}", rec);
        } 
    });    
}
