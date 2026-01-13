use anyhow::{anyhow, Context, Result};
use log::info;
use std::process::{Command, Stdio};

pub fn is_root() -> bool {
    whoami::username() == "root"
}

pub fn has_sudo() -> bool {
    which::which("sudo").is_ok()
}

pub fn run_cmd(cmd: &str, args: &[&str], use_sudo: bool) -> Result<()> {
    let mut command = if use_sudo && !is_root() {
        if !has_sudo() {
            return Err(anyhow!("Root privileges required but sudo not available."));
        }
        let mut c = Command::new("sudo");
        c.arg(cmd);
        c
    } else {
        Command::new(cmd)
    };

    command.args(args);
    command.stdout(Stdio::piped()).stderr(Stdio::piped());

    let output = command
        .execute()
        .with_context(|| format!("Failed to execute command: {} {:?}", cmd, args))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!(
            "Command failed: {} {:?}\nError: {}",
            cmd,
            args,
            stderr.trim()
        ));
    }

    Ok(())
}

trait CommandExecutor {
    fn execute(&mut self) -> std::io::Result<std::process::Output>;
}

impl CommandExecutor for Command {
    fn execute(&mut self) -> std::io::Result<std::process::Output> {
        self.output()
    }
}

pub fn detect_package_manager() -> Option<&'static str> {
    let managers = [
        ("apt-get", "apt"),
        ("dnf", "dnf"),
        ("yum", "yum"),
        ("pacman", "pacman"),
        ("apk", "apk"),
        ("zypper", "zypper"),
        ("brew", "brew"),
    ];

    for (binary, name) in managers {
        if which::which(binary).is_ok() {
            return Some(name);
        }
    }
    None
}

pub fn install_package(package_name: &str) -> Result<()> {
    let pm =
        detect_package_manager().ok_or_else(|| anyhow!("No supported package manager found."))?;

    info!("Installing {} using {}...", package_name, pm);

    match pm {
        "apt" => {
            run_cmd("apt-get", &["update"], true)?;
            run_cmd("apt-get", &["install", "-y", package_name], true)?;
        }
        "dnf" => {
            run_cmd("dnf", &["install", "-y", package_name], true)?;
        }
        "yum" => {
            run_cmd("yum", &["install", "-y", package_name], true)?;
        }
        "pacman" => {
            run_cmd("pacman", &["-Sy", "--noconfirm", package_name], true)?;
        }
        "apk" => {
            run_cmd("apk", &["add", package_name], true)?;
        }
        "zypper" => {
            run_cmd(
                "zypper",
                &["--non-interactive", "install", package_name],
                true,
            )?;
        }
        "brew" => {
            run_cmd("brew", &["install", package_name], false)?;
        }
        _ => return Err(anyhow!("Unsupported package manager: {}", pm)),
    }

    Ok(())
}

pub fn ensure_tor() -> Result<()> {
    if which::which("tor").is_ok() {
        return Ok(());
    }

    info!("Tor not found, installing...");
    install_package("tor")
}
