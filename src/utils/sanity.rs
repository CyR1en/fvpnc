use fs::Permissions;
use std::fs::File;
use std::{fs, io};
use std::io::{stdin, Write};
use std::string::FromUtf8Error;
use regex::{Match, Regex};
use serde::Deserialize;
use sudo::{check, RunningAs};
use crate::prelude::*;

pub const CONFIG_DIR: &str = "/etc/fvpnc";
pub const CONFIG_PATH: &str = "/etc/fvpnc/creds.txt";

pub fn pre_check() -> Result<String> {
    sudo::escalate_if_needed();
    init_configurations();
    check_creds();
    check_o_vpn()
}

fn check_o_vpn() -> Result<String> {
    // run openvpn --version
    let output = std::process::Command::new("openvpn")
        .arg("--version")
        .output()?;
    let out = String::from_utf8(output.stdout)?;
    find_o_vpn_pattern(out.as_str())
}

fn find_o_vpn_pattern(input: &str) -> Result<String> {
    let pattern = Regex::new(r"OpenVPN\s(\d+\.\d+\.\d+)").unwrap();
    let caps = pattern.captures(input).unwrap();
    let version = caps.get(1);
    match version {
        Some(str) => Ok(str.as_str().to_string()),
        None => Err(Error::Generic(format!("OpenVPN version not found")))
    }
}

fn check_creds() {
    // check if /etc/fvpnc folder exists
    if !std::path::Path::new(CONFIG_DIR).exists() {
        fs::create_dir(CONFIG_DIR).unwrap();
    }
    // check if /etc/fvpnc/creds.txt exists
    if !std::path::Path::new(CONFIG_PATH).exists() {
        println!("Login credentials could not be found, please go to https://account.fastvpn.com/ and find your credentials");
        let mut file = File::create(CONFIG_PATH).unwrap();
        let username = prompt("Username: ");
        let password = prompt("Password: ");
        file.write_all(format!("{}\n{}", username, password).as_bytes()).unwrap();
    }
}

fn prompt(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut out = String::new();
    stdin().read_line(&mut out).unwrap();
    if let Some('\n') = out.chars().next_back() {
        out.pop();
    }
    if let Some('\r') = out.chars().next_back() {
        out.pop();
    }
    out
}

#[derive(Debug, Deserialize)]
struct VpnConfig {
    country: String,
    city: String,
    protocol: String,
    filename: String,
}

fn parse_vpn_config(path: &str) -> Vec<VpnConfig> {
    let mut rdr = csv::Reader::from_path(path).unwrap();
    rdr.deserialize().map(|result| {
        let config: VpnConfig = result.unwrap();
        config
    }).collect()
}

fn init_configurations() {
    let configs = parse_vpn_config("res/vpn_config.csv");

    let mut has_missing = false;
    for config in configs {
        let path = format!("/etc/openvpn/{}/{}", config.protocol.to_lowercase(), config.filename);
        if !std::path::Path::new(&path).exists() {
            println!("Missing config: {}", path);
            has_missing = true;
        }
    }

    if !has_missing {
        return;
    }

    println!("Would you like to download the missing configuration files? [Y/n]: ");
    let mut input = String::new();

    let mut valid = false;
    while !valid {
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input.to_lowercase() == "y" {
            valid = true;
            download_configs();
            unzip();
        } else if input.to_lowercase() == "n" {
            valid = true;
            println!(r#"
                      Please download the missing configuration files from https://vpn.ncapi.io/groupedServerList.zip
                      and extract the contents to /etc/openvpn/tcp and /etc/openvpn/udp respectively."#);
            std::process::exit(1);
        }
    }
}

fn unzip() {
    // Unzip configs
    let zip = File::open("/tmp/groupedServerList.zip").unwrap();
    let mut archive = zip::ZipArchive::new(zip).unwrap();

    let paths = vec!["/etc/openvpn/tcp", "/etc/openvpn/udp"];
    for path in paths {
        if !std::path::Path::new(path).exists() {
            fs::create_dir(path).unwrap();
        }
    }

    let mut curr_dir: String = String::new();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        if (*file.name()).ends_with(".DS_Store") {
            continue;
        }

        if (*file.name()).ends_with("/") {
            curr_dir = file.name().to_string();
            continue;
        }

        let out_path = match file.enclosed_name() {
            Some(path) => {
                // add /etc/openvpn/ to path
                let parent = path.parent().unwrap();
                parent.join("/etc/openvpn/").join(curr_dir.to_owned()).join(path.file_name().unwrap())
            }
            None => continue,
        };

        println!(
            "File {} extracted to \"{}\" ({} bytes)",
            i,
            out_path.display(),
            file.size()
        );

        if std::path::Path::new(&out_path).exists() {
            continue;
        }

        let mut outfile = File::create(&out_path).unwrap();
        io::copy(&mut file, &mut outfile).unwrap();

        // Get and Set permissions
        #[cfg(unix)] {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&out_path, Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    // delete zip
    fs::remove_file("/tmp/groupedServerList.zip").unwrap();
}

fn download_configs() {
    let resp = reqwest::blocking::get("https://vpn.ncapi.io/groupedServerList.zip").expect("request failed");
    let body = resp.bytes().expect("body invalid");
    let mut zip = File::create("/tmp/groupedServerList.zip").expect("failed to create file");
    zip.write_all(&body).expect("failed to write file");
}