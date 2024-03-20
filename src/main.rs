use std::net::{Ipv4Addr, UdpSocket};
use std::os;

use clap::Parser;
use resolver::message::Message;
use resolver::RData;

use crate::resolver::DEFAULT_DNS;
use crate::{dns_cache::DnsCache, resolver::DnsService};

mod dns_cache;
mod resolver;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    dns_query: String,
}

fn main() -> std::io::Result<()> {
    // Udp client
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 12345)).expect("Could not bind client");
    socket
        .connect(DEFAULT_DNS)
        .expect("Could not connect to google");

    let listener = UdpSocket::bind(("127.0.0.1", 12345)).expect("Could not bind listener");

    loop {
        let mut buf = [0; 512];
        match listener.recv_from(&mut buf) {
            Ok((size, _)) => {
                let message: Message = buf.to_vec().into();
                println!("Message: {:?}", message)
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }

    let args = Args::parse();
    let cache = DnsCache::new();
    let mut dns_service = DnsService::new(socket, cache);

    let msg = match dns_service.lookup(args.dns_query.clone()) {
        Ok(msg) => msg,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1)
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
