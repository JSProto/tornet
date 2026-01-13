mod cli;
mod tor;
mod utils;

use clap::Parser;
use cli::Cli;
use colored::*;
use env_logger::Env;
use rand::Rng;
use std::thread;
use std::time::Duration;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Set up Ctrl+C handler to stop Tor services
    ctrlc::set_handler(move || {
        println!("\nReceived Ctrl+C! Stopping services...");
        if let Err(e) = tor::stop_tor() {
            eprintln!("Failed to stop Tor: {}", e);
        } else {
            println!("Tor services stopped.");
        }
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let args = Cli::parse();

    // Handle --stop
    if args.stop {
        log::info!("Stopping Tor services...");
        if let Err(e) = tor::stop_tor() {
            log::error!("Failed to stop Tor: {}", e);
        } else {
            log::info!("Tor services stopped.");
        }
        // In python it does pkill -f tornet. We can't easily kill other instances of ourself cross-platform without more deps.
        // But stopping the service is the main thing.
        return;
    }

    // Handle --auto-fix
    if args.auto_fix {
        log::info!("Running auto-fix...");
        if let Err(e) = utils::ensure_tor() {
            log::error!("Auto-fix failed: {}", e);
            std::process::exit(1);
        }
        log::info!("Auto-fix complete. Tor is installed.");
        return;
    }

    // Check pre-requisites
    if let Err(_) = which::which("tor") {
        log::error!("Tor is not installed. Run with --auto-fix to install automatically.");
        std::process::exit(1);
    }

    // Handle --ip
    if args.ip {
        match tor::get_current_ip() {
            Ok(ip) => {
                log::info!("Your IP address is: {}", ip.green());
            }
            Err(e) => {
                log::error!("Failed to get IP: {}", e);
                std::process::exit(1);
            }
        }
        return;
    }

    // Main Loop
    print_banner();

    // Ensure Tor is running
    if !tor::is_tor_running() {
        log::info!("Tor is not running. Starting service...");
        if let Err(e) = tor::start_tor() {
            log::error!("Failed to start Tor: {}", e);
            std::process::exit(1);
        }
    } else {
        log::info!("Tor is already running.");
    }

    // Initial IP
    match tor::get_current_ip() {
        Ok(ip) => log::info!("Current IP: {}", ip.green()),
        Err(e) => log::warn!("Could not get current IP: {}", e),
    }

    let count = args.count;
    let interval = args.interval;

    if count == 0 {
        loop {
            if let Err(_) = rotate_ip(&interval) {
                break;
            }
        }
    } else {
        for _ in 0..count {
            if let Err(_) = rotate_ip(&interval) {
                break;
            }
        }
    }
}

fn rotate_ip(interval_str: &str) -> anyhow::Result<()> {
    let sleep_time = parse_interval(interval_str)?;
    thread::sleep(Duration::from_secs(sleep_time));

    log::info!("Requesting new IP...");
    if let Err(e) = tor::reload_tor() {
        log::error!("Failed to reload Tor: {}", e);
        return Err(e);
    }

    match tor::get_current_ip() {
        Ok(ip) => println!("New IP address: {}", ip.green()),
        Err(e) => log::warn!("Failed to get new IP: {}", e),
    }

    Ok(())
}

fn parse_interval(interval_str: &str) -> anyhow::Result<u64> {
    if interval_str.contains('-') {
        let parts: Vec<&str> = interval_str.split('-').collect();
        if parts.len() == 2 {
            let start: u64 = parts[0].parse()?;
            let end: u64 = parts[1].parse()?;
            let mut rng = rand::thread_rng();
            return Ok(rng.gen_range(start..=end));
        }
    }
    Ok(interval_str.parse()?)
}

fn print_banner() {
    let banner = r#"
████████╗ ██████╗ ██████╗ ███╗   ██╗███████╗████████╗
╚══██╔══╝██╔═══██╗██╔══██╗████╗  ██║██╔════╝╚══██╔══╝
   ██║   ██║   ██║██████╔╝██╔██╗ ██║█████╗     ██║   
   ██║   ██║   ██║██╔══██╗██║╚██╗██║██╔══╝     ██║   
   ██║   ╚██████╔╝██║  ██║██║ ╚████║███████╗   ██║   
   ╚═╝    ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═══╝╚══════╝   ╚═╝   
"#;
    println!("{}", banner.green());
    println!("                    Version: 0.1.0");
    println!(" +---------------------(Antigravity)----------------------+");
}
