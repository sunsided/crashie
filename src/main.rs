//! crashie — when you need it to fail.

#[cfg(feature = "http-echo")]
mod http_echo;
mod options;
#[cfg(feature = "tcp-echo")]
mod tcp_echo;
#[cfg(feature = "udp-echo")]
mod udp_echo;

use clap::Parser;
use dotenvy::dotenv;
use options::Opts;
use rand::prelude::*;
use rand_distr::Normal;
use std::collections::HashSet;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    dotenv().ok();
    let mut rng = thread_rng();
    let opts: Opts = Opts::parse();

    // Bind TCP echo sockets.
    #[cfg(feature = "tcp-echo")]
    for addr in opts.tcp_echo_socks.iter().flatten() {
        if let Err(e) = tcp_echo::tcp_echo(addr) {
            eprintln!("Failed to bind to TCP socket: {e}");
            exit(1);
        }
    }

    // Bind TDP echo sockets.
    #[cfg(feature = "udp-echo")]
    for addr in opts.udp_echo_socks.iter().flatten() {
        if let Err(e) = udp_echo::udp_echo(addr) {
            eprintln!("Failed to bind to UDP socket: {e}");
            exit(1);
        }
    }

    // Bind HTTP sockets.
    #[cfg(feature = "http-echo")]
    for addr in opts.http_echo_socks.iter().flatten() {
        if let Err(e) = http_echo::http_echo(addr, opts.http_echo_liveness_probe_path.clone()) {
            eprintln!("Failed to bind to HTTP socket: {e}");
            exit(1);
        }
    }

    let sleep_delay_grace = opts.sleep_delay_grace;
    let sleep_delay_mean = opts.sleep_delay;
    let sleep_delay_stddev = opts.sleep_delay_stddev;
    let mut codes = collect_exit_codes(opts);
    if codes.is_empty() {
        codes.push(rng.gen_range(1_u8..=255))
    }

    // Select a random exit code.
    let exit_code = codes.choose(&mut rng).copied().expect("set was empty");

    // Sleep for a random duration.
    let sleep_time = sleep_delay_grace
        + sample_random_sleep_duration(&mut rng, sleep_delay_mean, sleep_delay_stddev);
    if sleep_time >= 1e-6 {
        println!("Sleeping for {sleep_time:.2} seconds, then exiting with code {exit_code}");
        sleep(Duration::from_secs_f64(sleep_time));
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
    codes.into_iter().collect()
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

const fn signal_to_exit(signal: u8) -> u8 {
    128 + signal
}
