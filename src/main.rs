use std::{
    env,
    net::{Ipv4Addr, UdpSocket},
    process::exit,
    time::Instant,
};

mod dns_cache;
mod resolver;

use crate::{
    dns_cache::IpCache,
    resolver::{DnsService, DEFAULT_DNS},
};

fn main() -> std::io::Result<()> {
    let args = env::args().collect::<Vec<String>>();

    let dns_name = match args.len() {
        2 => args[1].clone(),
        _ => {
            println!("Usage: dns-resolver <dns-name>");
            exit(1)
        }
    };

    // Udp client
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 12345)).expect("Could not bind client");
    socket
        .connect(DEFAULT_DNS)
        .expect("Could not connect to google");

    let cache = IpCache::new();
    let parser = DnsService::new(socket, cache);

    let start = Instant::now();

    let msg = match parser.send_query(dns_name) {
        Ok(msg) => msg,
        Err(e) => {
            println!("Error: {}", e);
            panic!("Could not send query")
        }
    };

    println!("Message: {:?}", msg.header);

    for msg in msg.answers.iter() {
        println!("Message: {:?}", msg);
    }

    let end = Instant::elapsed(&start);

    println!("Time elapsed: {:?}", end);

    // for answer in msg.answers {
    //     match answer.rdata {
    //         RData::A(ip) => {
    //             println!("Found answer to query, updating cache: A={}", ip);
    //             cache.set(dns_name.clone(), &ip.to_string());
    //         }
    //         _ => {}
    //     }
    // }

    Ok(())
}
