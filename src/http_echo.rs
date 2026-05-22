use chrono::prelude::*;
use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;

pub fn http_echo(
    addr: &SocketAddr,
    liveness_probe_path: String,
    default_status: u16,
) -> Result<(), std::io::Error> {
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
                    let liveness_probe_path = liveness_probe_path.clone();
                    thread::spawn(move || {
                        handle_client(stream, liveness_probe_path, default_status)
                    });
                }
                Err(e) => {
                    eprintln!("Error accepting HTTP connection: {e}");
                }
            }
        }
    });
    Ok(())
}

fn handle_client(stream: TcpStream, liveness_probe_path: String, default_status: u16) {
    let mut reader = BufReader::new(stream);

    loop {
        let mut request_line = String::new();
        if reader.read_line(&mut request_line).unwrap_or(0) == 0 {
            return;
        }

        // RFC2616: should ignore any empty line(s) (CRLF only) received
        // where a Request-Line is expected.
        if request_line == "\r\n" || request_line == "\n" {
            continue;
        }

        // Extract the path from the request line.
        let path = request_line.split_whitespace().nth(1).unwrap_or("/");

        // Read and ignore headers.
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

        // To retain mutable reference to the stream after use.
        let mut stream = reader
            .get_ref()
            .try_clone()
            .expect("Failed to obtain write stream");

        let date = Utc::now().format("%a, %d %b %Y %T GMT").to_string();
        let response = build_response(path, &liveness_probe_path, default_status, &date);

        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Failed to write HTTP response: {e}")
        }
    }
}

fn build_response(
    path: &str,
    liveness_probe_path: &str,
    default_status: u16,
    date: &str,
) -> String {
    let version = env!("CARGO_PKG_VERSION");
    let status = if path == liveness_probe_path {
        200
    } else {
        default_status
    };
    let reason = reason_phrase(status);
    format!(
        "HTTP/1.1 {status} {reason}\r\nServer: crashie/{version}\r\nDate: {date}\r\nContent-Length: 0\r\nCache-Control: no-cache, no-store\r\n\r\n"
    )
}

fn reason_phrase(status: u16) -> &'static str {
    match status {
        100 => "Continue",
        101 => "Switching Protocols",
        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        203 => "Non-Authoritative Information",
        204 => "No Content",
        205 => "Reset Content",
        206 => "Partial Content",
        300 => "Multiple Choices",
        301 => "Moved Permanently",
        302 => "Found",
        303 => "See Other",
        304 => "Not Modified",
        307 => "Temporary Redirect",
        308 => "Permanent Redirect",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        408 => "Request Timeout",
        409 => "Conflict",
        410 => "Gone",
        418 => "I'm a teapot",
        422 => "Unprocessable Entity",
        425 => "Too Early",
        428 => "Precondition Required",
        429 => "Too Many Requests",
        431 => "Request Header Fields Too Large",
        451 => "Unavailable For Legal Reasons",
        500 => "Internal Server Error",
        501 => "Not Implemented",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        504 => "Gateway Timeout",
        505 => "HTTP Version Not Supported",
        507 => "Insufficient Storage",
        511 => "Network Authentication Required",
        _ => "Status",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATE: &str = "Sat, 06 Jan 2024 14:44:53 GMT";
    const LIVENESS: &str = "/health/live";

    #[test]
    fn liveness_path_always_returns_200() {
        let response = build_response(LIVENESS, LIVENESS, 503, DATE);
        assert!(response.starts_with("HTTP/1.1 200 OK\r\n"));
    }

    #[test]
    fn non_liveness_path_uses_default_status() {
        let response = build_response("/anything", LIVENESS, 204, DATE);
        assert!(response.starts_with("HTTP/1.1 204 No Content\r\n"));
    }

    #[test]
    fn custom_status_is_used_for_non_liveness_path() {
        let response = build_response("/api", LIVENESS, 503, DATE);
        assert!(response.starts_with("HTTP/1.1 503 Service Unavailable\r\n"));
    }

    #[test]
    fn unknown_status_falls_back_to_generic_reason() {
        let response = build_response("/api", LIVENESS, 599, DATE);
        assert!(response.starts_with("HTTP/1.1 599 Status\r\n"));
    }

    #[test]
    fn response_includes_required_headers() {
        let response = build_response("/api", LIVENESS, 200, DATE);
        assert!(response.contains(&format!("Date: {DATE}\r\n")));
        assert!(response.contains("Content-Length: 0\r\n"));
        assert!(response.contains("Cache-Control: no-cache, no-store\r\n"));
        assert!(response.ends_with("\r\n\r\n"));
    }
}
