use std::net::{SocketAddr, UdpSocket};
use std::thread;

/// Listens for UDP datagrams on the given address and echoes each received datagram back to the sender.
///
/// # Arguments
///
/// * `addr` - The address to bind the UDP listener to.
///
/// # Examples
///
/// ```no_run
/// use std::net::{IpAddr, Ipv4Addr, SocketAddr};
///
/// let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
///
/// if let Err(e) = udp_echo(&addr) {
///     eprintln!("Error occurred while running UDP echo server: {:?}", e);
/// }
/// ```
///
/// # Errors
///
/// This function returns an `std::io::Error` if there is an error binding the UDP socket to the
/// given address.
pub fn udp_echo(addr: &SocketAddr) -> Result<(), std::io::Error> {
    let socket = UdpSocket::bind(addr)?;
    println!("Listening for UDP datagrams on {addr}");

    thread::spawn(move || {
        let mut buffer = [0; 512];

        loop {
            match socket.recv_from(&mut buffer) {
                Ok((size, src)) => {
                    if let Err(e) = socket.send_to(&buffer[0..size], src) {
                        eprintln!("Failed to echo UDP datagram: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to receive UDP datagram: {}", e);
                }
            }
        }
    });
    Ok(())
}
