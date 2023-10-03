use std::fmt::{Display, format, Formatter, write};
use std::net::IpAddr;
use std::path::PathBuf;
use crate::prelude::*;

pub mod sanity;

pub enum Proto {
    UDP,
    TCP,
}

impl Display for Proto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Proto::UDP => "udp",
            Proto::TCP => "tcp",
        })
    }
}

pub fn find_config(city: &str, proto: Proto) -> Result<PathBuf> {
    let config_dir = format!("/etc/openvpn/{}", proto);
    // list all files in config_dir
    let files = std::fs::read_dir(config_dir).unwrap();
    // for each file, the ip address is in the 4th line with format "remote <ip> <port>"
    for file in files {
        let path = file.unwrap().path();
        // if path buff contains city, return it
        if path.to_str().unwrap().contains(city) {
            // return path
            return Ok(path);
        }
    }
    Err(Error::Generic(format!("No config found for {}", city)))
}

