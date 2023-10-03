use std::{env, io};
use std::ffi::OsString;
use clap::{Parser, Subcommand};

pub(crate) fn parse() -> CLI {
    match CLI::try_parse() {
        Ok(c) => { c }
        Err(_) => {
            if atty::is(atty::Stream::Stdin) {
                return CLI::parse();
            }
            parse_stdin()
        }
    }
}

fn parse_stdin() -> CLI {
    let mut input = String::from("");

    io::stdin().lines().for_each(|line| {
        input.push_str(&line.unwrap());
        input.push_str("\n");
    });

    let input = input.trim();
    let mut args: Vec<OsString> = env::args_os().collect();
    args.push(input.into());
    match CLI::try_parse_from(args) {
        Ok(c) => { c }
        Err(e) => { e.exit() }
    }
}

#[derive(Parser)]
#[command(author, version, about)]
#[command(propagate_version = true)]
/// Command line interface to easily connect to Namecheap's FastVPN servers
pub(crate) struct CLI {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Status of the connection
    Status,

    /// Disconnect from a server
    Disconnect,

    /// Connect to a server
    Connect {
        #[clap(short, long, action)]
        /// Flag to set for UDP
        udp: bool,

        #[clap(short, long, action)]
        /// Flag to set for TCP
        tcp: bool,

        server_city: String,
    },
}