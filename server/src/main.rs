extern crate counter_server;

extern crate tokio_proto;
extern crate tokio_core;
extern crate tokio_timer;
extern crate tokio_io;
extern crate tokio_service;

extern crate futures;
extern crate futures_cpupool;
extern crate bytes;

extern crate atoi;

use tokio_proto::TcpServer;
use futures_cpupool::CpuPool;

mod service;

use std::env;
use std::process;
use counter_server::Config;

use std::net::SocketAddr;
use std::net::{IpAddr, Ipv4Addr};


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|e| {
        println!("Problem parsing arguments:\n\t{}", e);
        process::exit(0);
    });

    let thread_pool = CpuPool::new(config.n_kernels);
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), config.port);

    if let IpAddr::V4(ip) = addr.ip() {
        println!("Running on {:?}:{:?}", ip, addr.port());
    }

    let server = TcpServer::new(service::TaskProto, addr);

    server.serve(move || {
        Ok(service::ComputingService {
            thread_pool: thread_pool.clone(),
            timeout: config.timeout,
        })
    });
}
