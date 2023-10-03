## fvpnc
An unofficial command-line client for [Namecheap's FastVPN](https://fastvpn.com/) for Linux, written in Rust 🦀 🦀 🦀.

THIS IS ONLY INTENDED TO BE USED IN LINUX

### Usage
```
Command line interface to easily connect to Namecheap's FastVPN servers

Usage: fvpnc <COMMAND>

Commands:
  status      Status of the connection
  disconnect  Disconnect from a server
  connect     Connect to a server
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
For now, `fvpnc` is using `OpenVPN` to connect to the server. `IkeV2` and `Wireguard` could be supported in the future.

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
