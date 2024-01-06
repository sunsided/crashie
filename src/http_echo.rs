use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;

pub fn http_echo(addr: &SocketAddr) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(addr)?;
    println!("Listening for HTTP connections on {addr}");

    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!(
                        "Accepting HTTP connection from {}",
                        stream.peer_addr().expect("Unable to obtain peer address")
                    );
                    thread::spawn(move || handle_client(stream));
                }
                Err(e) => {
                    eprintln!("Error accepting HTTP connection: {e}");
                }
            }
        }
    });
    Ok(())
}

fn handle_client(stream: TcpStream) {
    let mut reader = BufReader::new(stream);

    loop {
        let mut request_line = String::new();
        if reader.read_line(&mut request_line).unwrap_or(0) == 0 {
            return;
        }

        // RFC2616: should ignore any empty line(s) (CRLF only) received
        // where a Request-Line is expected
        if request_line == "\r\n" || request_line == "\n" {
            continue;
        }

        // Reading headers
        let mut header_line = String::new();
        loop {
            header_line.clear();
            if reader.read_line(&mut header_line).unwrap_or(0) == 0 {
                return;
            }
            if header_line == "\r\n" || header_line == "\n" {
                break;
            }
        }

        // To retain mutable reference to the stream after use
        let mut stream = reader
            .get_ref()
            .try_clone()
            .expect("Failed to obtain write stream");

        let response = "HTTP/1.1 204 No Content\r\n\r\n";
        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Failed to write HTTP response: {e}")
        }
    }
}
