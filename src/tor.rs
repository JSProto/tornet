use crate::utils::run_cmd;
use anyhow::{anyhow, Context, Result};
use log::info;
use reqwest::Proxy;
use std::process::Command;
use std::thread;
use std::time::Duration;

const TOR_PROXY: &str = "socks5://127.0.0.1:9050";
const CHECK_IP_URL: &str = "https://api.ipify.org";

pub fn detect_service_manager() -> Option<&'static str> {
    if which::which("systemctl").is_ok() && std::path::Path::new("/run/systemd/system").exists() {
        Some("systemctl")
    } else if which::which("service").is_ok() {
        Some("service")
    } else if which::which("brew").is_ok() {
        Some("brew")
    } else {
        None
    }
}

pub fn service_action(action: &str) -> Result<()> {
    let mgr = detect_service_manager().ok_or_else(|| {
        anyhow!("No supported service manager found (systemctl, service, or brew)")
    })?;

    match mgr {
        "systemctl" => run_cmd("systemctl", &[action, "tor"], true),
        "service" => run_cmd("service", &["tor", action], true),
        "brew" => {
            // map "reload" to "restart" for brew services as reload might not be supported or behaves differently
            let brew_action = if action == "reload" {
                "restart"
            } else {
                action
            };
            run_cmd("brew", &["services", brew_action, "tor"], false)
        }
        _ => unreachable!(),
    }
}

pub fn is_tor_running() -> bool {
    // Check if process exists using pgrep
    if let Ok(_) = Command::new("pgrep").arg("-x").arg("tor").output() {
        return true;
    }
    false
}

pub fn start_tor() -> Result<()> {
    service_action("start")?;
    info!("Tor service started. Waiting for connection...");
    thread::sleep(Duration::from_secs(5));
    Ok(())
}

pub fn stop_tor() -> Result<()> {
    let _ = service_action("stop");
    Ok(())
}

pub fn reload_tor() -> Result<()> {
    service_action("reload")?;
    thread::sleep(Duration::from_secs(2));
    Ok(())
}

pub fn get_ip_direct() -> Result<String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let ip = client
        .get(CHECK_IP_URL)
        .send()
        .context("Failed to get IP directly")?
        .text()?;

    Ok(ip.trim().to_string())
}

pub fn get_ip_via_tor() -> Result<String> {
    let proxy = Proxy::all(TOR_PROXY)?;
    let client = reqwest::blocking::Client::builder()
        .proxy(proxy)
        .timeout(Duration::from_secs(20))
        .build()?;

    let ip = client
        .get(CHECK_IP_URL)
        .send()
        .context("Failed to get IP via Tor")?
        .text()?;

    Ok(ip.trim().to_string())
}

pub fn get_current_ip() -> Result<String> {
    if is_tor_running() {
        get_ip_via_tor()
    } else {
        get_ip_direct()
    }
}
