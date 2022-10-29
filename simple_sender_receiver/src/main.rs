use std::{net::UdpSocket, time::Duration};
use std::sync::atomic::AtomicBool;
use std::io::ErrorKind;
use std::thread;
use std::str;


use std::io::Read;

static IS_ALIVE: AtomicBool = AtomicBool::new(false);

fn write(){
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.set_nonblocking(true).unwrap();
    while IS_ALIVE.load(std::sync::atomic::Ordering::SeqCst){
        match socket.send_to("hello world!".as_bytes(), "127.0.0.1:45000"){         
            Ok(send) => println!("Send {} bytes", send),
            Err(e) => println!("send function failed: {e:?}"),
        }
        thread::sleep(Duration::from_millis(100));
    }
    println!("Exiting write");
}

fn read(socket:&UdpSocket){
    while IS_ALIVE.load(std::sync::atomic::Ordering::SeqCst){
        let mut buf=[0; 65535];
         
        let received_size = match socket.recv(&mut buf) {
            Ok(received) => received,
            Err(err) if err.kind() != ErrorKind::WouldBlock => panic!("Caught error: {}", err),
            _ => {thread::sleep(Duration::from_millis(1000));continue}
        };
        println!("Received {} bytes", received_size);
        let msg = match str::from_utf8(&buf[..received_size]){
            Ok(msg)=> msg, 
            Err(e)=> panic!("Failed to parse message: {e:?}"),
        };
        
        println!("Message: {}", &msg);
        
        

    }
    println!("Exiting read");
}

fn main() {
    let name = "David";
    println!("Hello, {}!", name);
    let socket = UdpSocket::bind("127.0.0.1:45000").expect("couldn't bind to address");
    socket.set_nonblocking(true).unwrap();
    IS_ALIVE.store(true, std::sync::atomic::Ordering::SeqCst);
    let write_thread = thread::spawn(|| write());
    let read_thread = thread::spawn(move || read(&socket));

    println!("Press Enter to stop!");
    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
    IS_ALIVE.store(false, std::sync::atomic::Ordering::SeqCst);
    write_thread.join().expect("Couldn't join write on the associated thread");
    read_thread.join().expect("Couldn't join read on the associated thread");
    println!("Exit!");
}
