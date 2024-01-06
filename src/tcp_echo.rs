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
    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // The client has closed the connection.
                return;
            }
            Ok(n) => {
                // Echo everything back.
                if let Err(e) = stream.write_all(&buffer[0..n]) {
                    eprintln!("Failed to write to socket: {e}");
                    return;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from socket: {e}");
                return;
            }
        }
    }
}
