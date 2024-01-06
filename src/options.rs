use clap::Parser;
use std::net::{SocketAddr, ToSocketAddrs};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    #[clap(
        short = 'd',
        long = "delay",
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
            help = "Binds an echo TCP socket on the specified addresses",
            use_value_delimiter(true),
            value_parser(parse_socket_addr),
            env = "CRASHIE_BIND_TCP_ECHO"
        )
    )]
    pub tcp_echo_socks: Vec<Vec<SocketAddr>>,
    #[clap(
        short = 'e',
        long = "exit-code",
        use_value_delimiter(true),
        allow_negative_numbers = false,
        help = "Exit with the specified code(s)",
        env = "CRASHIE_EXIT_CODES"
    )]
    pub exit_codes: Vec<u8>,
    #[clap(
        short = 's',
        long = "signals",
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
            help = "Hang up controlling terminal or terminal",
            env = "CRASHIE_SIGHUP"
        )
    )]
    pub sighup: bool,
    #[cfg_attr(
        feature = "posix",
        clap(
            long = "sigint",
            help = "Interrupt from keyboard, Control-C",
            env = "CRASHIE_SIGINT"
        )
    )]
    pub sigint: bool,
    #[cfg_attr(
        feature = "posix",
        clap(
            long = "sigquit",
            help = "Quit from keyboard, Control-\\",
            env = "CRASHIE_SIGQUIT"
        )
    )]
    pub sigquit: bool,
    #[cfg_attr(
        feature = "posix",
        clap(long = "sigill", help = "Illegal instruction", env = "CRASHIE_SIGILL")
    )]
    pub sigill: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(
            long = "sigtrap",
            help = "Breakpoint for debugging",
            env = "CRASHIE_SIGTRAP"
        )
    )]
    pub sigtrap: bool,
    #[cfg_attr(
        feature = "posix",
        clap(
            long = "sigabrt",
            help = "Abnormal termination",
            env = "CRASHIE_SIGABRT"
        )
    )]
    pub sigabrt: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(
            long = "sigiot",
            help = "Equivalent to SIGABRT",
            env = "CRASHIE_SIGIOT"
        )
    )]
    pub sigiot: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(long = "sigbus", help = "Bus error", env = "CRASHIE_SIGBUS")
    )]
    pub sigbus: bool,
    #[cfg_attr(
        feature = "posix",
        clap(
            long = "sigfpe",
            help = "Floating-point exception",
            env = "CRASHIE_SIGFPE"
        )
    )]
    pub sigfpe: bool,
    #[cfg_attr(
        feature = "posix",
        clap(
            long = "sigkill",
            help = "Forced process termination",
            env = "CRASHIE_SIGKILL"
        )
    )]
    pub sigkill: bool,
    #[cfg_attr(
        feature = "posix",
        clap(
            long = "sigusr1",
            help = "Freely available to processes",
            env = "CRASHIE_SIGUSR1"
        )
    )]
    pub sigusr1: bool,
    #[cfg_attr(
        feature = "posix",
        clap(
            long = "sigsegv",
            help = "Invalid memory reference (Segmentation Fault)",
            env = "CRASHIE_SIGSEGV"
        )
    )]
    pub sigsegv: bool,
    #[cfg_attr(
        feature = "posix",
        clap(
            long = "sigusr2",
            help = "Freely available to processes",
            env = "CRASHIE_SIGUSR2"
        )
    )]
    pub sigusr2: bool,
    #[cfg_attr(
        feature = "posix",
        clap(
            long = "sigpipe",
            help = "Write to pipe with no readers",
            env = "CRASHIE_SIGPIPE"
        )
    )]
    pub sigpipe: bool,
    #[cfg_attr(
        feature = "posix",
        clap(long = "sigalrm", help = "Real-time clock", env = "CRASHIE_SIGALRM")
    )]
    pub sigalrm: bool,
    #[cfg_attr(
        feature = "posix",
        clap(
            long = "sigterm",
            help = "Process termination",
            env = "CRASHIE_SIGTERM"
        )
    )]
    pub sigterm: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(
            long = "sigstkflt",
            help = "Coprocessor stack error",
            env = "CRASHIE_SIGSTKFLT"
        )
    )]
    pub sigstkflt: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(
            long = "sigchld",
            help = "Child process stopped, terminated or got a signal if traced",
            env = "CRASHIE_SIGCHLD"
        )
    )]
    pub sigchld: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(
            long = "sigxcpu",
            help = "CPU time limit exceeded",
            env = "CRASHIE_SIGXCPU"
        )
    )]
    pub sigxcpu: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(
            long = "sigxfsz",
            help = "File size limit exceeded",
            env = "CRASHIE_SIGXFSZ"
        )
    )]
    pub sigxfsz: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(
            long = "sigvtalrm",
            help = "Virtual timer clock",
            env = "CRASHIE_SIGVTALRM"
        )
    )]
    pub sigvtalrm: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(
            long = "sigprof",
            help = "Profile timer clock",
            env = "CRASHIE_SIGPROF"
        )
    )]
    pub sigprof: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(long = "sigio", help = "I/O now possible", env = "CRASHIE_SIGIO")
    )]
    pub sigio: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(
            long = "sigpoll",
            help = "Equivalent to SIGIO",
            env = "CRASHIE_SIGPOLL"
        )
    )]
    pub sigpoll: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(long = "sigpwr", help = "Power supply failure", env = "CRASHIE_SIGPWR")
    )]
    pub sigpwr: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(long = "sigsys", help = "Bad system call", env = "CRASHIE_SIGSYS")
    )]
    pub sigsys: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(
            long = "sigunused",
            help = "Equivalent to SIGSYS",
            env = "CRASHIE_SIGUNUSED"
        )
    )]
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

fn parse_socket_addr(input: &str) -> Result<Vec<SocketAddr>, String> {
    Ok(input
        .to_socket_addrs()
        .map_err(|e| format!("{e}"))?
        .into_iter()
        .collect())
}
