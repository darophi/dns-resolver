use std::{
    env,
    net::{Ipv4Addr, UdpSocket},
    process::exit,
    time::{Duration, Instant},
};

use clap::Parser;

use crate::{
    dns_cache::DnsCache,
    resolver::{DnsService, DEFAULT_DNS},
};

mod dns_cache;
mod resolver;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    dns_query: String,
}
fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let bench_tries = 1;

    // Udp client
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 12345)).expect("Could not bind client");
    socket
        .connect(DEFAULT_DNS)
        .expect("Could not connect to google");

    let cache = DnsCache::new();
    let mut dns_service = DnsService::new(socket, cache);

    let mut runs: Vec<Duration> = Vec::new();

    for i in 0..bench_tries {
        let start = Instant::now();
        let msg = match dns_service.send_query(args.dns_query.clone()) {
            Ok(msg) => msg,
            Err(e) => {
                println!("Error: {}", e);
                panic!("Could not send query")
            }
        };

        let end = Instant::elapsed(&start);

        println!("Time elapsed: {:?}", end);

        runs.push(end);
    }

    let mut sum = Duration::new(0, 0);

    for run in runs {
        sum += run;
    }

    let avg = sum / (bench_tries as u32);

    println!("Average time: {:?}", avg);

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
