use sysinfo::{Pid, Process, ProcessExt, System, SystemExt};

use crate::cli::Commands;
use crate::prelude::*;
use crate::utils::Proto;
use crate::utils::sanity::{CONFIG_PATH, pre_check};

mod error;
mod prelude;
mod utils;
mod cli;

fn find_openvpn(sys: &System) -> Option<(&Pid, &Process)> {
    sys.processes().iter().find(|entry| entry.1.name().contains("openvpn"))
}

fn is_running(sys: &System) -> bool {
    find_openvpn(sys).is_some()
}

fn status(sys: &System) {
    let running = is_running(&sys);
    println!("Connected: {}", running);

    let new_ip = std::process::Command::new("curl")
        .arg("https://ipinfo.io/ip")
        .output().unwrap();
    println!("IP Addr: {}", String::from_utf8(new_ip.stdout).unwrap());

    if running {
        let (pid, process) = find_openvpn(&sys).unwrap();
        println!("PID: {}", pid);
        println!("Runtime: {}s", process.run_time());
    }
}

fn disconnect(sys: &System) {
    if !is_running(&sys) {
        println!("You are not connected to FastVPN Server.");
        return;
    }

    std::process::Command::new("killall")
        .arg("openvpn")
        .output().unwrap();
    println!("Disconnected from FastVPN Server.")
}

fn connect(sys: &System, city: &str, proto: Proto) {
    if is_running(&sys) {
        println!("Already connected to FastVPN Server. Disconnect first.");
        return;
    }

    match utils::find_config(city, proto) {
        Ok(path) => {
            std::process::Command::new("openvpn")
                .arg("--config")
                .arg(path.into_os_string().into_string().unwrap())
                .arg("--daemon")
                .arg("--auth-user-pass")
                .arg(CONFIG_PATH)
                .output().unwrap();
            println!("Connected to FastVPN Server.");
        }
        Err(_) => {
            println!("Could not find config for {} server.", city);
        }
    }
}

#[cfg(unix)]
fn main() -> Result<()> {
    match pre_check() {
        Ok(_) => {}
        Err(_) => {
            std::process::exit(1);
        }
    }

    let sys = System::new_all();
    let args = cli::parse();
    match &args.command {
        Commands::Status => status(&sys),
        Commands::Disconnect => disconnect(&sys),
        Commands::Connect { udp, tcp, server_city } => {
            let proto = if *udp && *tcp { Proto::TCP } else if *udp { Proto::UDP } else { Proto::TCP };
            connect(&sys, server_city, proto)
        }
    }
    Ok(())
}

#[cfg(windows)]
fn main() -> Result<()> {
    panic!("This program is only supported on Unix systems. Download FastVPN client for Windows.");
}
