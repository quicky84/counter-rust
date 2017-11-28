extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate bytes;
extern crate atoi;
extern crate futures_cpupool;
extern crate num_cpus;
extern crate counter_server;

mod codec;

use codec::{TaskCodec, Response, Completion};
use counter_server::Config;
use futures::{Future, Stream, Sink};
use futures_cpupool::CpuPool;
use std::{cmp, env, process};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::time::Instant;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use tokio_io::AsyncRead;


fn compute(difficulty: u64, timeout: u64) -> Completion {
    let now = Instant::now();

    for _ in 0..difficulty {
        let elapsed = now.elapsed();
        if elapsed.as_secs() > timeout {
            return Completion::OutOfTime;
        }
    }

    let elapsed = now.elapsed();

    let millisec = (elapsed.as_secs() * 1_000) + u64::from(elapsed.subsec_nanos() / 1_000_000);

    Completion::Time(millisec)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|e| {
        println!("Problem parsing arguments:\n\t{}", e);
        process::exit(0);
    });

    let cpus = cmp::min(num_cpus::get(), config.n_kernels);
    let pool = CpuPool::new(cpus);
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), config.port);

    if let IpAddr::V4(ip) = addr.ip() {
        println!("Running on {:?}:{:?}", ip, addr.port());
    }

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let listener = TcpListener::bind(&addr, &handle).unwrap();

    let connections = listener.incoming();

    let server = connections.for_each(|(client, _peer_address)| {
        //     sink  stream
        let (writer, reader) = client.framed(TaskCodec::new()).split();

        let pool_clone = pool.clone();
        let timeout = config.timeout;

        let responses = reader.and_then(move |rq| {
            let (id, difficulty) = (rq.id, u64::from(rq.difficulty));
            let copy_id = id;

            let computation = pool_clone.spawn_fn(move || {
                println!("--------> Starting Task {}", copy_id);

                let completion = compute(difficulty, timeout);

                println!("<-------- Finishing Task {}", copy_id);

                Ok(completion)
            });

            Box::new(computation.map(move |completion| {
                println!("Task {} completed", id);
                Response { id, completion }
            }))
        });

        let handler = writer.send_all(responses).then(|_| Ok(()));
        handle.spawn(handler);

        Ok(())
    });

    core.run(server).unwrap();
}
