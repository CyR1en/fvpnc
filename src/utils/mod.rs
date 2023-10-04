use std::fmt::{Display, Formatter};
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

    let files = std::fs::read_dir(config_dir).unwrap();
    for file in files {
        let path = file.unwrap().path();
        if path.to_str().unwrap().contains(city) {
            return Ok(path);
        }
    }
    Err(Error::Generic(format!("No config found for {}", city)))
}

