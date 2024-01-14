use std::{
    net::{Ipv4Addr, UdpSocket},
};

use clap::Parser;
use resolver::RData;

use crate::{
    dns_cache::DnsCache,
    resolver::{DnsService},
};
use crate::resolver::DEFAULT_DNS;

mod dns_cache;
mod resolver;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    dns_query: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // Udp client
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 12345)).expect("Could not bind client");
    socket
        .connect(DEFAULT_DNS)
        .expect("Could not connect to google");

    let cache = DnsCache::new();
    let mut dns_service = DnsService::new(socket, cache);

    let msg = match dns_service.send_query(args.dns_query.clone()) {
        Ok(msg) => msg,
        Err(e) => {
            println!("Error: {}", e);
            panic!("Could not send query")
        }
    };

    for answer in msg.answers {
        match answer.rdata {
            RData::A(ip) => {
                println!("Found answer to query, updating cache: A={}", ip);
            }
            _ => {}
        }
    }

    Ok(())
}
