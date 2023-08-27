use serde_json::json;
use std::io::ErrorKind;
use std::io::Read;
use std::str;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;
use std::{net::UdpSocket, time::Duration};

fn send(sender: &UdpSocket, addr: &str, msg: &serde_json::Value) {
    match sender.send_to(msg.to_string().as_bytes(), addr) {
        Ok(send) => println!("Send {} bytes", send),
        Err(e) => println!("send function failed: {e:?}"),
    };
}

fn receive(receiver: &UdpSocket) {
    let mut buf = [0; 65535];
    let received_size = match receiver.recv(&mut buf) {
        Ok(received) => received,
        Err(err) => match err.kind() {
            ErrorKind::WouldBlock => 0,
            _ => panic!("Caught error: {}", err),
        },
    };
    println!("Received {} bytes", received_size);
    let msg = match str::from_utf8(&buf[..received_size]) {
        Ok(msg) => msg,
        Err(e) => panic!("Failed to parse message: {e:?}"),
    };
    println!("{msg}");
}

fn stop(keep_alive: Arc<AtomicBool>) {
    println!("Press Enter to stop!");
    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
    keep_alive.store(false, std::sync::atomic::Ordering::SeqCst);
}

fn main() {
    let keep_alive = Arc::new(AtomicBool::new(true));
    // json message 1
    let message1 = json!({
        "id": 0,
        "msg: ": "Hello Earth!"
    });
    // json message 2
    let message2 = json!({
        "id": 1,
        "msg: ": "Hello Space!"
    });
    // json sender 1
    let socket1: UdpSocket = match UdpSocket::bind("127.0.0.1:45000") {
        Ok(sock) => sock,
        Err(e) => panic!("Error! {}", e),
    };
    match socket1.set_nonblocking(true) {
        Err(e) => panic!("Error! {}", e),
        _ => {}
    }
    // json sender 2
    let socket2: UdpSocket = match UdpSocket::bind("127.0.0.1:45001") {
        Ok(sock) => sock,
        Err(e) => panic!("Error! {}", e),
    };
    match socket2.set_nonblocking(true) {
        Err(e) => panic!("Error! {}", e),
        _ => {}
    }

    let signal_atomic = keep_alive.clone();
    let signal_thread = thread::spawn(move || stop(signal_atomic));
    while keep_alive.load(std::sync::atomic::Ordering::SeqCst) {
        send(&socket1, "127.0.0.1:45001", &message1);
        send(&socket2, "127.0.0.1:45000", &message2);
        receive(&socket1);
        receive(&socket2);
        thread::sleep(Duration::from_millis(100));
        //break;
    }
    signal_thread
        .join()
        .expect("Couldn't join signal_thread on the associated thread");
    println!("Exit!");
}
