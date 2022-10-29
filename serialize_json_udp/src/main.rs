use serde_json::json;
use std::{net::UdpSocket, time::Duration};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread;
use std::io::ErrorKind;
use std::str;
use std::io::Read;

fn main() {
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
    // lock 
    let is_alive = Arc::new(AtomicBool::new(true));
    // json sender 1 
    let socket1: Arc<UdpSocket> = match UdpSocket::bind("127.0.0.1:45000"){
        Ok(sock) => Arc::new(sock), 
        Err(e) => panic!("Error! {}", e),
    }; 
   match socket1.set_nonblocking(true){
        Err(e) => panic!("Error! {}", e),
        _ => {}
   }
    // json sender 2
    let socket2: Arc<UdpSocket> = match UdpSocket::bind("127.0.0.1:45001"){
        Ok(sock) => Arc::new(sock), 
        Err(e) => panic!("Error! {}", e),
    }; 
    match socket2.set_nonblocking(true){
         Err(e) => panic!("Error! {}", e),
         _ => {}
    }

    // socket 1 sender
    let socket1_sender_is_alive = is_alive.clone();
    let socket1_sender = socket1.clone();
    let socket1_sender_thread = thread::spawn(move || {
        while socket1_sender_is_alive.load(std::sync::atomic::Ordering::SeqCst){
            match socket1_sender.send_to(message1.to_string().as_bytes(), "127.0.0.1:45001"){         
                Ok(send) => println!("Send {} bytes", send),
                Err(e) => println!("send function failed: {e:?}"),
            };
            thread::sleep(Duration::from_millis(1000));
        }
    });

    // socket 1 receiver 
    let socket1_receiver_is_alive = is_alive.clone();
    let socket1_receiver = socket1.clone();
    let socket1_receiver_thread = thread::spawn(move || {
        let mut buf=[0; 65535];
        while socket1_receiver_is_alive.load(std::sync::atomic::Ordering::SeqCst){        
            let received_size = match socket1_receiver.recv(&mut buf) {
                Ok(received) => received,
                Err(err) if err.kind() != ErrorKind::WouldBlock => panic!("Caught error: {}", err),
                _ => {thread::sleep(Duration::from_millis(1000));continue}
            };
            println!("Received {} bytes", received_size);
            let msg = match str::from_utf8(&buf[..received_size]){
                Ok(msg)=> msg, 
                Err(e)=> panic!("Failed to parse message: {e:?}"),
            };
            println!("{msg}");
        }
    });

        // socket 2 sender
        let socket2_sender_is_alive = is_alive.clone();
        let socket2_sender = socket2.clone();
        let socket2_sender_thread = thread::spawn(move || {
            while socket2_sender_is_alive.load(std::sync::atomic::Ordering::SeqCst){
                match socket2_sender.send_to(message2.to_string().as_bytes(), "127.0.0.1:45000"){         
                    Ok(send) => println!("Send {} bytes", send),
                    Err(e) => println!("send function failed: {e:?}"),
                };
                thread::sleep(Duration::from_millis(1000));
            }
        });
    
        // socket 2 receiver 
        let socket2_receiver_is_alive = is_alive.clone();
        let socket2_receiver = socket2.clone();
        let socket2_receiver_thread = thread::spawn(move || {
            let mut buf=[0; 65535];
            while socket2_receiver_is_alive.load(std::sync::atomic::Ordering::SeqCst){        
                let received_size = match socket2_receiver.recv(&mut buf) {
                    Ok(received) => received,
                    Err(err) if err.kind() != ErrorKind::WouldBlock => panic!("Caught error: {}", err),
                    _ => {thread::sleep(Duration::from_millis(1000));continue}
                };
                println!("Received {} bytes", received_size);
                let msg = match str::from_utf8(&buf[..received_size]){
                    Ok(msg)=> msg, 
                    Err(e)=> panic!("Failed to parse message: {e:?}"),
                };
                println!("{msg}");
            }
        });



    println!("Press Enter to stop!");
    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
    is_alive.store(false, std::sync::atomic::Ordering::SeqCst);
    socket1_sender_thread.join().expect("Couldn't join socket1_sender_thread on the associated thread");
    socket1_receiver_thread.join().expect("Couldn't join socket1_receiver_thread on the associated thread");
    socket2_sender_thread.join().expect("Couldn't join socket2_sender_thread on the associated thread");
    socket2_receiver_thread.join().expect("Couldn't join socket2_receiver_thread on the associated thread");
    println!("Exit!");
}
