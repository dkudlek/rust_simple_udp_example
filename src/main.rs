use rust_simple_udp_example::comm;
use std::io::Read;

fn main() {
    let _socket1 = comm::SocketSender::new("127.0.0.1:45000", "127.0.0.1:45001", "Hello World");
    let _socket2 = comm::SocketSender::new("127.0.0.1:45001", "127.0.0.1:45000", "Henlo Earth!");
    println!("Press Enter to stop!");
    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
}
