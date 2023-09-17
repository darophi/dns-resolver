use std::{
    env,
    net::{Ipv4Addr, UdpSocket},
    process::exit,
    time::{Duration, Instant},
};

mod dns_cache;
mod resolver;

use crate::{
    dns_cache::DnsCache,
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

    let bench_tries = 100000;

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
        let msg = match dns_service.send_query(dns_name.clone()) {
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
