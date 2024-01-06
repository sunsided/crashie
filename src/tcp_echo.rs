use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;

/// Listens for TCP connections on the given address and spawns a new thread for each
/// accepted connection.
///
/// # Arguments
///
/// * `addr` - The address to bind the TCP listener to.
///
/// # Examples
///
/// ```no_run
/// use std::net::{IpAddr, Ipv4Addr, SocketAddr};
///
/// let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
///
/// if let Err(e) = tcp_echo(&addr) {
///     eprintln!("Error occurred while running TCP echo server: {:?}", e);
/// }
/// ```
///
/// # Errors
///
/// This function returns an `std::io::Error` if there is an error binding the TCP listener to the
/// given address, or if there is an error accepting a connection.
pub fn tcp_echo(addr: &SocketAddr) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(addr)?;
    println!("Listening for TCP connections on {addr}");

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

/// Handles the client connection and echoes the received data back.
///
/// This function reads data from the provided `TcpStream` and writes it back to the stream.
/// It operates in a loop until the client closes the connection or an error occurs.
///
/// # Arguments
///
/// * `stream` - A `TcpStream` representing the client connection.
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
