use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;

pub fn tcp_echo(addr: &SocketAddr) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(&addr)?;
    println!("Listening for TCP connections on: {addr}");

    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!(
                        "Accepting TCP connection from {}",
                        stream.peer_addr().expect("Unable to obtain peer address")
                    );
                    thread::spawn(move || handle_client(stream));
                }
                Err(e) => {
                    eprintln!("Error accepting TCP connection: {e}");
                }
            }
        }
    });
    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = String::new();

    match stream.read_to_string(&mut buffer) {
        Ok(_) => {
            stream.write_all(buffer.as_bytes()).unwrap();
        }
        Err(e) => {
            eprintln!("Failed to read from socket: {}", e);
        }
    }
}
