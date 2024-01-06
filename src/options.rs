use clap::Parser;
use std::net::SocketAddr;

const HELP_SECTION_CRASH_AFTER: &str = "Delay (crash after)";
const HELP_SECTION_ECHO_SERVER: &str = "Echo Server";
const HELP_SECTION_ECHO_SERVER_HTTP: &str = "Echo Server (HTTP)";
const HELP_SECTION_EXIT_CODES: &str = "Exit Codes";
const HELP_SECTION_EXIT_CODES_POSIX: &str = "Exit Codes (POSIX)";
const HELP_SECTION_EXIT_CODES_NON_POSIX: &str = "Exit Codes (non-POSIX)";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    #[clap(
        short = 'd',
        long = "delay",
        help_heading = HELP_SECTION_CRASH_AFTER,
        help = "The sleep duration before exiting, in seconds",
        value_name = "SECONDS",
        allow_negative_numbers = false,
        default_value = "10.0",
        value_parser(parse_seconds),
        env = "CRASHIE_SLEEP_DELAY"
    )]
    pub sleep_delay: f64,
    #[clap(
        long = "delay-stddev",
        help_heading = HELP_SECTION_CRASH_AFTER,
        help = "The standard deviation of the sleep duration, in seconds",
        value_name = "SECONDS",
        allow_negative_numbers = false,
        default_value = "2.0",
        value_parser(parse_seconds),
        env = "CRASHIE_SLEEP_DELAY_STDDEV"
    )]
    pub sleep_delay_stddev: f64,

    #[cfg_attr(
        feature = "tcp-echo",
        clap(
            long = "bind-tcp-echo",
            help_heading = HELP_SECTION_ECHO_SERVER,
            help = "Provide TCP echo on the specified addresses",
            value_name = "SOCK_ADDR",
            use_value_delimiter(true),
            value_parser(parse_socket_addr),
            env = "CRASHIE_BIND_TCP_ECHO"
        )
    )]
    #[cfg_attr(not(feature = "tcp-echo"), clap(skip))]
    pub tcp_echo_socks: Vec<Vec<SocketAddr>>,
    #[cfg_attr(
        feature = "udp-echo",
        clap(
            long = "bind-udp-echo",
            help_heading = HELP_SECTION_ECHO_SERVER,
            help = "Provide UDP echo on the specified addresses",
            value_name = "SOCK_ADDR",
            use_value_delimiter(true),
            value_parser(parse_socket_addr),
            env = "CRASHIE_BIND_UDP_ECHO"
        )
    )]
    #[cfg_attr(not(feature = "udp-echo"), clap(skip))]
    pub udp_echo_socks: Vec<Vec<SocketAddr>>,
    #[cfg_attr(
        feature = "http-echo",
        clap(
            long = "bind-http-echo",
            help_heading = HELP_SECTION_ECHO_SERVER_HTTP,
            help = "Provide HTTP echo on the specified addresses",
            value_name = "SOCK_ADDR",
            use_value_delimiter(true),
            value_parser(parse_socket_addr),
            env = "CRASHIE_BIND_HTTP_ECHO"
        )
    )]
    #[cfg_attr(not(feature = "http-echo"), clap(skip))]
    pub http_echo_socks: Vec<Vec<SocketAddr>>,
    #[cfg_attr(
        feature = "http-echo",
        clap(
            long = "http-liveness-probe-path",
            help_heading = HELP_SECTION_ECHO_SERVER_HTTP,
            help = "The request path on which to serve liveness probe results",
            value_name = "HTTP_PATH",
            use_value_delimiter(true),
            default_value = "/health/live",
            env = "CRASHIE_HTTP_LIVENESS_PROBE_PATH"
        )
    )]
    #[cfg_attr(not(feature = "http-echo"), clap(skip))]
    pub http_echo_liveness_probe_path: String,

    #[clap(
        short = 'e',
        long = "exit-code",
        help_heading = HELP_SECTION_EXIT_CODES,
        use_value_delimiter(true),
        allow_negative_numbers = false,
        help = "Exit with the specified code(s)",
        env = "CRASHIE_EXIT_CODES"
    )]
    pub exit_codes: Vec<u8>,
    #[clap(
        short = 's',
        long = "signals",
        help_heading = HELP_SECTION_EXIT_CODES,
        use_value_delimiter(true),
        value_parser(parse_signal),
        value_name = "NUMBER",
        allow_negative_numbers = false,
        help = "Arbitrary signal (exit code 128+SIGNAL)",
        env = "CRASHIE_SIGNALS"
    )]
    pub signal: Vec<u8>,

    #[cfg_attr(
        feature = "posix",
        clap(
            long = "sighup",
            help_heading = HELP_SECTION_EXIT_CODES_POSIX,
            help = "Hang up controlling terminal or terminal",
            env = "CRASHIE_SIGHUP"
        )
    )]
    #[cfg_attr(not(feature = "posix"), clap(skip))]
    pub sighup: bool,
    #[cfg_attr(
        feature = "posix",
        clap(
            long = "sigint",
            help_heading = HELP_SECTION_EXIT_CODES_POSIX,
            help = "Interrupt from keyboard, Control-C",
            env = "CRASHIE_SIGINT"
        )
    )]
    #[cfg_attr(not(feature = "posix"), clap(skip))]
    pub sigint: bool,
    #[cfg_attr(
        feature = "posix",
        clap(
            long = "sigquit",
            help_heading = HELP_SECTION_EXIT_CODES_POSIX,
            help = "Quit from keyboard, Control-\\",
            env = "CRASHIE_SIGQUIT"
        )
    )]
    #[cfg_attr(not(feature = "posix"), clap(skip))]
    pub sigquit: bool,
    #[cfg_attr(
        feature = "posix",
        clap(
            long = "sigill",
            help_heading = HELP_SECTION_EXIT_CODES_POSIX,
            help = "Illegal instruction",
            env = "CRASHIE_SIGILL"
        )
    )]
    #[cfg_attr(not(feature = "posix"), clap(skip))]
    pub sigill: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(
            long = "sigtrap",
            help_heading = HELP_SECTION_EXIT_CODES_NON_POSIX,
            help = "Breakpoint for debugging",
            env = "CRASHIE_SIGTRAP"
        )
    )]
    #[cfg_attr(not(feature = "non-posix"), clap(skip))]
    pub sigtrap: bool,
    #[cfg_attr(
        feature = "posix",
        clap(
            long = "sigabrt",
            help_heading = HELP_SECTION_EXIT_CODES_POSIX,
            help = "Abnormal termination",
            env = "CRASHIE_SIGABRT"
        )
    )]
    #[cfg_attr(not(feature = "posix"), clap(skip))]
    pub sigabrt: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(
            long = "sigiot",
            help_heading = HELP_SECTION_EXIT_CODES_NON_POSIX,
            help = "Equivalent to SIGABRT",
            env = "CRASHIE_SIGIOT"
        )
    )]
    #[cfg_attr(not(feature = "non-posix"), clap(skip))]
    pub sigiot: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(
            long = "sigbus",
            help_heading = HELP_SECTION_EXIT_CODES_NON_POSIX,
            help = "Bus error",
            env = "CRASHIE_SIGBUS"
        )
    )]
    #[cfg_attr(not(feature = "non-posix"), clap(skip))]
    pub sigbus: bool,
    #[cfg_attr(
        feature = "posix",
        clap(
            long = "sigfpe",
            help_heading = HELP_SECTION_EXIT_CODES_POSIX,
            help = "Floating-point exception",
            env = "CRASHIE_SIGFPE"
        )
    )]
    #[cfg_attr(not(feature = "posix"), clap(skip))]
    pub sigfpe: bool,
    #[cfg_attr(
        feature = "posix",
        clap(
            long = "sigkill",
            help_heading = HELP_SECTION_EXIT_CODES_POSIX,
            help = "Forced process termination",
            env = "CRASHIE_SIGKILL"
        )
    )]
    #[cfg_attr(not(feature = "posix"), clap(skip))]
    pub sigkill: bool,
    #[cfg_attr(
        feature = "posix",
        clap(
            long = "sigusr1",
            help_heading = HELP_SECTION_EXIT_CODES_POSIX,
            help = "Freely available to processes",
            env = "CRASHIE_SIGUSR1"
        )
    )]
    #[cfg_attr(not(feature = "posix"), clap(skip))]
    pub sigusr1: bool,
    #[cfg_attr(
        feature = "posix",
        clap(
            long = "sigsegv",
            help_heading = HELP_SECTION_EXIT_CODES_POSIX,
            help = "Invalid memory reference (Segmentation Fault)",
            env = "CRASHIE_SIGSEGV"
        )
    )]
    #[cfg_attr(not(feature = "posix"), clap(skip))]
    pub sigsegv: bool,
    #[cfg_attr(
        feature = "posix",
        clap(
            long = "sigusr2",
            help_heading = HELP_SECTION_EXIT_CODES_POSIX,
            help = "Freely available to processes",
            env = "CRASHIE_SIGUSR2"
        )
    )]
    #[cfg_attr(not(feature = "posix"), clap(skip))]
    pub sigusr2: bool,
    #[cfg_attr(
        feature = "posix",
        clap(
            long = "sigpipe",
            help_heading = HELP_SECTION_EXIT_CODES_POSIX,
            help = "Write to pipe with no readers",
            env = "CRASHIE_SIGPIPE"
        )
    )]
    #[cfg_attr(not(feature = "posix"), clap(skip))]
    pub sigpipe: bool,
    #[cfg_attr(
        feature = "posix",
        clap(
            long = "sigalrm",
            help_heading = HELP_SECTION_EXIT_CODES_POSIX,
            help = "Real-time clock",
            env = "CRASHIE_SIGALRM"
        )
    )]
    #[cfg_attr(not(feature = "posix"), clap(skip))]
    pub sigalrm: bool,
    #[cfg_attr(
        feature = "posix",
        clap(
            long = "sigterm",
            help_heading = HELP_SECTION_EXIT_CODES_POSIX,
            help = "Process termination",
            env = "CRASHIE_SIGTERM"
        )
    )]
    #[cfg_attr(not(feature = "posix"), clap(skip))]
    pub sigterm: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(
            long = "sigstkflt",
            help_heading = HELP_SECTION_EXIT_CODES_NON_POSIX,
            help = "Coprocessor stack error",
            env = "CRASHIE_SIGSTKFLT"
        )
    )]
    #[cfg_attr(not(feature = "non-posix"), clap(skip))]
    pub sigstkflt: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(
            long = "sigchld",
            help_heading = HELP_SECTION_EXIT_CODES_NON_POSIX,
            help = "Child process stopped, terminated or got a signal if traced",
            env = "CRASHIE_SIGCHLD"
        )
    )]
    #[cfg_attr(not(feature = "non-posix"), clap(skip))]
    pub sigchld: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(
            long = "sigxcpu",
            help_heading = HELP_SECTION_EXIT_CODES_NON_POSIX,
            help = "CPU time limit exceeded",
            env = "CRASHIE_SIGXCPU"
        )
    )]
    #[cfg_attr(not(feature = "non-posix"), clap(skip))]
    pub sigxcpu: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(
            long = "sigxfsz",
            help_heading = HELP_SECTION_EXIT_CODES_NON_POSIX,
            help = "File size limit exceeded",
            env = "CRASHIE_SIGXFSZ"
        )
    )]
    #[cfg_attr(not(feature = "non-posix"), clap(skip))]
    pub sigxfsz: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(
            long = "sigvtalrm",
            help_heading = HELP_SECTION_EXIT_CODES_NON_POSIX,
            help = "Virtual timer clock",
            env = "CRASHIE_SIGVTALRM"
        )
    )]
    #[cfg_attr(not(feature = "non-posix"), clap(skip))]
    pub sigvtalrm: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(
            long = "sigprof",
            help_heading = HELP_SECTION_EXIT_CODES_NON_POSIX,
            help = "Profile timer clock",
            env = "CRASHIE_SIGPROF"
        )
    )]
    #[cfg_attr(not(feature = "non-posix"), clap(skip))]
    pub sigprof: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(
            long = "sigio",
            help_heading = HELP_SECTION_EXIT_CODES_NON_POSIX,
            help = "I/O now possible",
            env = "CRASHIE_SIGIO"
        )
    )]
    #[cfg_attr(not(feature = "non-posix"), clap(skip))]
    pub sigio: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(
            long = "sigpoll",
            help_heading = HELP_SECTION_EXIT_CODES_NON_POSIX,
            help = "Equivalent to SIGIO",
            env = "CRASHIE_SIGPOLL"
        )
    )]
    #[cfg_attr(not(feature = "non-posix"), clap(skip))]
    pub sigpoll: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(
            long = "sigpwr",
            help_heading = HELP_SECTION_EXIT_CODES_NON_POSIX,
            help = "Power supply failure",
            env = "CRASHIE_SIGPWR"
        )
    )]
    #[cfg_attr(not(feature = "non-posix"), clap(skip))]
    pub sigpwr: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(
            long = "sigsys",
            help_heading = HELP_SECTION_EXIT_CODES_NON_POSIX,
            help = "Bad system call",
            env = "CRASHIE_SIGSYS"
        )
    )]
    #[cfg_attr(not(feature = "non-posix"), clap(skip))]
    pub sigsys: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(
            long = "sigunused",
            help_heading = HELP_SECTION_EXIT_CODES_NON_POSIX,
            help = "Equivalent to SIGSYS",
            env = "CRASHIE_SIGUNUSED"
        )
    )]
    #[cfg_attr(not(feature = "non-posix"), clap(skip))]
    pub sigunused: bool,
}

fn parse_signal(input: &str) -> Result<u8, String> {
    let signal: u8 = input.parse().map_err(|e| format!("{e}"))?;
    if !(1..=31).contains(&signal) {
        Err(String::from("Signals must be in range 1 to 31 (inclusive)"))
    } else {
        Ok(signal)
    }
}

fn parse_seconds(input: &str) -> Result<f64, String> {
    let value: f64 = input.parse().map_err(|e| format!("{e}"))?;
    if value < 0.0 {
        Err(String::from("Value must be a non-negative number"))
    } else {
        Ok(value)
    }
}

#[cfg(any(feature = "tcp-echo", feature = "udp-echo"))]
fn parse_socket_addr(input: &str) -> Result<Vec<SocketAddr>, String> {
    use std::net::ToSocketAddrs;
    Ok(input
        .to_socket_addrs()
        .map_err(|e| format!("{e}"))?
        .collect())
}
