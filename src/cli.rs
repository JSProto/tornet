use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Fidal (Ported by Antigravity)", version, about = "Automate IP address changes using Tor", long_about = None)]
pub struct Cli {
    /// Time in seconds between IP changes (or range like "30-120")
    #[arg(long, default_value = "60")]
    pub interval: String,

    /// Number of times to change IP. If 0, change IP indefinitely
    #[arg(long, default_value_t = 10)]
    pub count: u32,

    /// Display current IP address and exit
    #[arg(long)]
    pub ip: bool,

    /// Automatically install missing dependencies
    #[arg(long)]
    pub auto_fix: bool,

    /// Stop all Tor services and tornet processes
    #[arg(long)]
    pub stop: bool,
}
