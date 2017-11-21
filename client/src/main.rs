#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

extern crate counter_client;

use counter_client::Config;
use futures::Future;

use std::env;
use std::process;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use tokio_io::io;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|e| {
        println!("Problem parsing arguments:\n\t{}", e);
        process::exit(0);
    });

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let socket = TcpStream::connect(&config.address, &handle);

    let request = socket.and_then(|socket| {
        println!("Making requests");
        io::write_all(socket, b"15 100000\n")
    });

    let response = request.and_then(|(socket, _request)| {
        println!("{:?}", _request);
        io::read_to_end(socket, Vec::new())
    });

    let (_socket, data) = core.run(response).unwrap();
    println!("{}", String::from_utf8_lossy(&data));
}
