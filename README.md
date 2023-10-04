# fvpnc
An unofficial command-line client for [Namecheap's FastVPN](https://fastvpn.com/) for Linux, written in Rust 🦀 🦀 🦀.

THIS IS ONLY INTENDED TO BE USED IN LINUX.

_`fvpnc` can also be used in macOS. However, you are limited to only `OpenVPN`. To take advantage of `WireGuard` or `IKEV2` download the official client for macOS_

## Dependencies

All of the cargo dependencies could be found [here](https://crates.io/crates/fvpnc/dependencies)

However, `fvpnc` depends on `OpenVPN` being installed on your machine. Below are the commands to execute to install `OpenVPN`

#### Linux
```
sudo apt update && sudo apt install openvpn -y
```

#### MacOS
```
brew install openvpn
```
Make sure that it's included to your `PATH`. Add this line to the end of your shell config (`.bshrc`, `.zshrc`, etc..)
```
export PATH="/usr/local/opt/openvpn/sbin:$PATH"
```

## Usage
```
An unofficial command-line client for Namecheap's FastVPN for Linux

Usage: fvpnc <COMMAND>

Commands:
  status      Status of the connection
  cities      List all available cities
  disconnect  Disconnect from a server
  connect     Connect to a server
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
For now, `fvpnc` is using `OpenVPN` to connect to the server. `IKEV2` and `WireGuard` could be supported in the future.

#### Authentication
When you initially run `fvpnc`, it will ask for your credentials that could be found at [here](https://account.fastvpn.com/)

#### Connecting
Connecting to a server is fairly easy, you just simply need to know which city you want to connect to and set your protocol.
```
fvpnc connect --udp Dallas
```
The command above will connect us to the `Dallas` server

#### Disconnecting
Disconnecting to the server, you simply run the following command:
```
fvpnc disconnect
```
