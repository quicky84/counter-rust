#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

extern crate counter_client;
extern crate rand;

use counter_client::Config;
use futures::Future;
use rand::{thread_rng, Rng};

use std::env;
use std::net::SocketAddr;
use std::process;

use std::thread;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use tokio_io::io;

fn request(address: SocketAddr, rq: &str) {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let socket = TcpStream::connect(&address, &handle);

    let task = socket
        .and_then(|socket| io::write_all(socket, rq.as_bytes()))
        .and_then(|(socket, _request)| {
            // Shoutdown the socket, so server knows that we end up with the input
            // https://stackoverflow.com/questions/39049365/
            // rust-echo-server-and-client-using-futures-blocks-itself-forever
            socket.shutdown(std::net::Shutdown::Write).expect(
                "Couldn't shut \
                 down",
            );
            io::read_to_end(socket, Vec::new())
        });

    let (_socket, data) = core.run(task).unwrap();
    println!("{}", String::from_utf8_lossy(&data));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|e| {
        println!("Problem parsing arguments:\n\t{}", e);
        process::exit(0);
    });

    let mut rng = thread_rng();

    let mut tasks = vec![];
    for i in 0..config.n_tasks {
        let d = rng.gen_range(config.min, config.max);
        let rq = format!("{} {}\n", i + 1, d);
        let address = config.address;

        tasks.push(thread::spawn(move || { request(address, rq.as_str()); }));
    }

    for task in tasks {
        // Wait for the thread to finish. Returns a result.
        let _ = task.join();
    }
}
