# TorNet (Rust Edition)

TorNet is a Rust-based tool that automates IP address changes using Tor. It is designed to enhance your online privacy by frequently changing your IP address, making it difficult for trackers to pinpoint your location.

https://api.ipify.org
https://dnsleaktest.com

## Features

- **Automated IP Rotation**: Change your IP address at specified intervals.
- **Cross-Platform Support**: Works on Linux (apt, dnf, yum, pacman, apk, zypper) and macOS (Homebrew).
- **Dependency Management**: Automatically installs Tor if missing (requires sudo).
- **Verification**: Built-in IP checker to verify your anonymity.
- **Fast & Efficient**: Written in Rust for better performance and safety compared to the original Python version.

## Installation

### From Source
1. Ensure you have Rust installed (via [rustup](https://rustup.rs/)).
2. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/tornet.git
   cd tornet
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```
4. The binary will be available at `target/release/tornet`.

### Prerequisites
- **Tor**: The tool attempts to install Tor automatically, or you can install it manually:
  - Linux: `sudo apt install tor` (or your distro's equivalent)
  - macOS: `brew install tor`

## Usage

Run the tool using `cargo run` (development) or the built binary.

```bash
# Basic usage: Change IP every 60 seconds, 10 times
./tornet

# Customize interval and count
./tornet --interval 30 --count 5

# Change IP indefinitely with a random interval between 10 and 20 seconds
./tornet --interval 30-120 --count 0

# Check current IP address
./tornet --ip

# Auto-install dependencies (Tor)
./tornet --auto-fix

# Stop Tor services
./tornet --stop

# Show help
./tornet --help
```

## Options

- `--interval <SECONDS>`: Time in seconds between IP changes. Supports ranges like `30-120` for random intervals. Default: 60.
- `--count <NUMBER>`: Number of times to change the IP. Set to `0` for infinite. Default: 10.
- `--ip`: Display the current IP address (both direct and via Tor) and exit.
- `--auto-fix`: Automatically attempt to install missing dependencies (Tor).
- `--stop`: Stop the Tor service and exit.

## Browser Configuration

To use the rotating IP in your browser:

1. Open your browser's proxy settings.
2. Select **Manual proxy configuration**.
3. Set **SOCKS Host** to `127.0.0.1` and **Port** to `9050`.
4. Ensure SOCKS v5 is selected.
5. (Optional) Enable "Proxy DNS when using SOCKS v5".

## Troubleshooting

- **Access Denied**: Installing packages or restarting services usually requires root privileges. The tool uses `sudo` internally; ensure you have sudo access.
- **Tor Connection Failed**: Check if Tor is already running or if a firewall is blocking the connection.
- **Brew issues on macOS**: Ensure Homebrew is correctly set up.

## License

MIT License
