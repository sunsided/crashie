//! crashie — when you need it to fail.

use clap::Parser;
use dotenvy::dotenv;
use rand::prelude::*;
use rand_distr::Normal;
use std::collections::HashSet;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[clap(
        short = 'd',
        long = "delay",
        help = "The sleep duration before exiting, in seconds",
        value_name = "SECONDS",
        allow_negative_numbers = false,
        default_value = "10.0",
        value_parser(parse_seconds)
    )]
    sleep_delay: f64,
    #[clap(
        long = "delay-stddev",
        help = "The standard deviation of the sleep duration, in seconds",
        value_name = "SECONDS",
        allow_negative_numbers = false,
        default_value = "0.0",
        value_parser(parse_seconds)
    )]
    sleep_delay_stddev: f64,
    #[clap(
        short = 'e',
        long = "exit-code",
        use_value_delimiter(true),
        allow_negative_numbers = false,
        help = "Exit with the specified code(s)"
    )]
    exit_codes: Vec<u8>,
    #[clap(
        short = 's',
        long = "signal",
        use_value_delimiter(true),
        value_parser(parse_signal),
        value_name = "NUMBER",
        allow_negative_numbers = false,
        help = "Arbitrary signal (exit code 128+SIGNAL)"
    )]
    signal: Vec<u8>,
    #[cfg_attr(
        feature = "posix",
        clap(long = "sighup", help = "Hang up controlling terminal or terminal")
    )]
    sighup: bool,
    #[cfg_attr(
        feature = "posix",
        clap(long = "sigint", help = "Interrupt from keyboard, Control-C")
    )]
    sigint: bool,
    #[cfg_attr(
        feature = "posix",
        clap(long = "sigquit", help = "Quit from keyboard, Control-\\")
    )]
    sigquit: bool,
    #[cfg_attr(feature = "posix", clap(long = "sigill", help = "Illegal instruction"))]
    sigill: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(long = "sigtrap", help = "Breakpoint for debugging")
    )]
    sigtrap: bool,
    #[cfg_attr(
        feature = "posix",
        clap(long = "sigabrt", help = "Abnormal termination")
    )]
    sigabrt: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(long = "sigiot", help = "Equivalent to SIGABRT")
    )]
    sigiot: bool,
    #[cfg_attr(feature = "non-posix", clap(long = "sigbus", help = "Bus error"))]
    sigbus: bool,
    #[cfg_attr(
        feature = "posix",
        clap(long = "sigfpe", help = "Floating-point exception")
    )]
    sigfpe: bool,
    #[cfg_attr(
        feature = "posix",
        clap(long = "sigkill", help = "Forced process termination")
    )]
    sigkill: bool,
    #[cfg_attr(
        feature = "posix",
        clap(long = "sigusr1", help = "Freely available to processes")
    )]
    sigusr1: bool,
    #[cfg_attr(
        feature = "posix",
        clap(
            long = "sigsegv",
            help = "Invalid memory reference (Segmentation Fault)"
        )
    )]
    sigsegv: bool,
    #[cfg_attr(
        feature = "posix",
        clap(long = "sigusr2", help = "Freely available to processes")
    )]
    sigusr2: bool,
    #[cfg_attr(
        feature = "posix",
        clap(long = "sigpipe", help = "Write to pipe with no readers")
    )]
    sigpipe: bool,
    #[cfg_attr(feature = "posix", clap(long = "sigalrm", help = "Real-time clock"))]
    sigalrm: bool,
    #[cfg_attr(
        feature = "posix",
        clap(long = "sigterm", help = "Process termination")
    )]
    sigterm: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(long = "sigstkflt", help = "Coprocessor stack error")
    )]
    sigstkflt: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(
            long = "sigchld",
            help = "Child process stopped, terminated or got a signal if traced"
        )
    )]
    sigchld: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(long = "sigxcpu", help = "CPU time limit exceeded")
    )]
    sigxcpu: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(long = "sigxfsz", help = "File size limit exceeded")
    )]
    sigxfsz: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(long = "sigvtalrm", help = "Virtual timer clock")
    )]
    sigvtalrm: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(long = "sigprof", help = "Profile timer clock")
    )]
    sigprof: bool,
    #[cfg_attr(feature = "non-posix", clap(long = "sigio", help = "I/O now possible"))]
    sigio: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(long = "sigpoll", help = "Equivalent to SIGIO")
    )]
    sigpoll: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(long = "sigpwr", help = "Power supply failure")
    )]
    sigpwr: bool,
    #[cfg_attr(feature = "non-posix", clap(long = "sigsys", help = "Bad system call"))]
    sigsys: bool,
    #[cfg_attr(
        feature = "non-posix",
        clap(long = "sigunused", help = "Equivalent to SIGSYS")
    )]
    sigunused: bool,
}

fn main() {
    dotenv().ok();
    let mut rng = thread_rng();

    let opts: Opts = Opts::parse();

    let sleep_delay_mean = opts.sleep_delay;
    let sleep_delay_stddev = opts.sleep_delay_stddev;
    let codes = collect_exit_codes(opts);

    // Select a random exit code.
    let exit_code = codes.choose(&mut rng).copied().expect("set was empty");

    // Sleep for a random duration.
    let sleep_time = sample_random_sleep_duration(&mut rng, sleep_delay_mean, sleep_delay_stddev);
    if sleep_time >= 1e-6 {
        println!("Sleeping for {sleep_time:.2} seconds, then exiting with code {exit_code}",);
        let duration = Duration::from_secs_f64(sleep_time);
        sleep(duration);
    }

    println!("Exiting with code {exit_code}");
    exit(exit_code as i32)
}

fn sample_random_sleep_duration(
    mut rng: &mut ThreadRng,
    sleep_delay_mean: f64,
    sleep_delay_stddev: f64,
) -> f64 {
    let normal = match Normal::new(sleep_delay_mean, sleep_delay_stddev) {
        Ok(dist) => dist,
        Err(e) => {
            eprintln!("Failed to initialize normal distribution: {e}");
            exit(1);
        }
    };

    normal.sample(&mut rng).abs()
}

fn collect_exit_codes(opts: Opts) -> Vec<u8> {
    let mut codes: HashSet<u8> = HashSet::from_iter(opts.exit_codes.iter().copied());
    add_signals(opts, &mut codes);

    if codes.is_empty() {
        codes.insert(0);
    }

    let codes: Vec<u8> = codes.into_iter().collect();
    codes
}

fn add_signals(opts: Opts, codes: &mut HashSet<u8>) {
    for signal in opts.signal {
        codes.insert(signal_to_exit(signal));
    }
    if opts.sighup {
        codes.insert(signal_to_exit(1));
    }
    if opts.sigint {
        codes.insert(signal_to_exit(2));
    }
    if opts.sigquit {
        codes.insert(signal_to_exit(3));
    }
    if opts.sigill {
        codes.insert(signal_to_exit(4));
    }
    if opts.sigtrap {
        codes.insert(signal_to_exit(5));
    }
    if opts.sigabrt || opts.sigiot {
        codes.insert(signal_to_exit(6));
    }
    if opts.sigbus {
        codes.insert(signal_to_exit(7));
    }
    if opts.sigfpe {
        codes.insert(signal_to_exit(8));
    }
    if opts.sigkill {
        codes.insert(signal_to_exit(9));
    }
    if opts.sigusr1 {
        codes.insert(signal_to_exit(10));
    }
    if opts.sigsegv {
        codes.insert(signal_to_exit(11));
    }
    if opts.sigusr2 {
        codes.insert(signal_to_exit(12));
    }
    if opts.sigpipe {
        codes.insert(signal_to_exit(13));
    }
    if opts.sigalrm {
        codes.insert(signal_to_exit(14));
    }
    if opts.sigterm {
        codes.insert(signal_to_exit(15));
    }
    if opts.sigstkflt {
        codes.insert(signal_to_exit(16));
    }
    if opts.sigchld {
        codes.insert(signal_to_exit(17));
    }
    if opts.sigxcpu {
        codes.insert(signal_to_exit(24));
    }
    if opts.sigxfsz {
        codes.insert(signal_to_exit(25));
    }
    if opts.sigvtalrm {
        codes.insert(signal_to_exit(26));
    }
    if opts.sigprof {
        codes.insert(signal_to_exit(27));
    }
    if opts.sigio || opts.sigpoll {
        codes.insert(signal_to_exit(29));
    }
    if opts.sigpwr {
        codes.insert(signal_to_exit(30));
    }
    if opts.sigsys || opts.sigunused {
        codes.insert(signal_to_exit(31));
    }
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

const fn signal_to_exit(signal: u8) -> u8 {
    128 + signal
}
