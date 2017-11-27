#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate futures_cpupool;

extern crate counter_client;
extern crate rand;

use counter_client::Config;
use futures::{Future, future};
use futures_cpupool::CpuPool;
use rand::{thread_rng, Rng};

use std::env;
use std::net::SocketAddr;
use std::process;

use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use tokio_io::io;

fn request(address: SocketAddr, rq: &str) -> String {
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
    String::from_utf8_lossy(&data).into_owned()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|e| {
        println!("Problem parsing arguments:\n\t{}", e);
        process::exit(0);
    });

    let mut tasks = vec![];
    // set up a thread pool
    let pool = CpuPool::new_num_cpus();

    for i in 0..config.n_tasks {
        let address = config.address;
        let (min, max) = (config.min, config.max);

        let task = pool.spawn_fn(move || {
            let mut rng = thread_rng();
            let d = rng.gen_range(min, max);
            let rq = format!("{} {}\n", i + 1, d);

            let rs = request(address, rq.as_str());
            Ok::<String, ()>(rs)
        }).map(|rs| println!("{}", rs));
        tasks.push(task);
    }

    let _ = future::join_all(tasks).wait();
}
