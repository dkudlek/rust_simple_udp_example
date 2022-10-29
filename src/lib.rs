pub mod comm{
    use std::net::UdpSocket;
    use std::sync::Arc;
    use std::time::Duration;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;
    use std::thread;
    use std::thread::JoinHandle;
    use std::sync;
    use std::io::ErrorKind;
    use std::str;
    
    pub struct SocketSender{
        is_alive: sync::Arc<AtomicBool>,
        sender_thread: Option<JoinHandle<()>>,
        reader_thread: Option<JoinHandle<()>>,
    }

    impl SocketSender{
        pub fn new(local_addr: &str, target_addr: &str, msg: &str) -> SocketSender{
            let socket: Arc<UdpSocket> = match UdpSocket::bind(local_addr){
                Ok(sock) => Arc::new(sock), 
                Err(e) => panic!("Error! {}", e),
            }; 
            match socket.set_nonblocking(true){
                    Err(e) => panic!("Error! {}", e),
                    _ => {}
            }
            // Shared variable to kill both threads
            let is_alive = Arc::new(AtomicBool::new(true));


            let writer_message = msg.clone().to_string();
            let writer_is_alive = is_alive.clone();
            let writer_target: String = target_addr.clone().to_string();
            let writer_socket = socket.clone();
            let writer_thread = Some(thread::spawn(move || {
                let target: &str = writer_target.as_str();
                let msg= writer_message.as_str();
                while writer_is_alive.load(std::sync::atomic::Ordering::SeqCst){
                    match writer_socket.send_to(msg.as_bytes(), target){         
                        Ok(send) => println!("Send {} bytes", send),
                        Err(e) => println!("send function failed: {e:?}"),
                    };
                    thread::sleep(Duration::from_millis(1000));
                }
            }));

            let reader_is_alive = is_alive.clone();
            let reader_socket = socket.clone();
            let reader_thread = Some(thread::spawn(move || {
                let mut buf=[0; 65535];
                while reader_is_alive.load(std::sync::atomic::Ordering::SeqCst){        
                    let received_size = match reader_socket.recv(&mut buf) {
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
            }));



            SocketSender{
                is_alive: is_alive,
                sender_thread: writer_thread,
                reader_thread: reader_thread,
            }
            


        }
    
    }
    impl Drop for SocketSender{
        fn drop(&mut self) {
            self.is_alive.store(false, Ordering::SeqCst);
            self.reader_thread.take().unwrap().join().expect("");
            self.sender_thread.take().unwrap().join().expect("");
            println!("Panic?");

        }
    }
}