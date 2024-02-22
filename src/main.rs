use std::borrow::BorrowMut;
use std::net::{Ipv4Addr, UdpSocket};
use std::sync::{Arc, Mutex};

use axum::extract::State;
use axum::routing::get;
use axum::Router;
use clap::{command, Parser};
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

struct AppState {
    db: i32,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Udp client
    let state = Arc::new(Mutex::new(AppState { db: 0 }));

    let app = Router::new().route("/", get(root)).with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

fn dns_cmd() {
    let args = Args::parse();
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
}

async fn root(State(state): State<Arc<Mutex<AppState>>>) -> &'static str {
    let mut x = state.lock().unwrap();

    x.db += 1;
    println!("DB value: {:?}", x.db);

    "Hello World"
}
