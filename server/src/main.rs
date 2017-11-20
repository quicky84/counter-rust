// #![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

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

use std::env;
use std::process;
use counter_server::Config;

use std::net::SocketAddr;
use std::net::{IpAddr, Ipv4Addr};

use futures::Future;
use tokio_service::Service;

use std::io;
use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder, Framed};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_proto::pipeline::ServerProto;
use atoi::atoi;
use std::time::{Duration, Instant};
use tokio_timer::Timer;


pub enum Completion {
    Time(u64), // milliseconds
    OutOfTime,
}

pub struct Request {
    id: u32,
    difficulty: u32,
}

pub struct Response {
    id: u32,
    completion: Completion,
}

fn decode(buf: &mut BytesMut) -> io::Result<Option<Request>> {
    // println!("Received: {:?}", buf);

    // Expected input is `u32 u32\n`
    let i = match buf.iter().position(|&b| b == b'\n') {
        Some(i) => i,
        _ => return Ok(None),
    };

    // read up the first `\n`
    let sub_buf = buf.split_to(i);

    // after the read-out, there is still `\n` belogning to "our" input which has to be removed
    buf.split_to(1);

    let i = match sub_buf.iter().position(|&b| b == b' ') {
        Some(i) => i,
        _ => return Ok(None),
    };

    let (id, difficulty) = (&sub_buf[..i], &sub_buf[i + 1..]);

    let (id, difficulty) = match (atoi::<u32>(id), atoi::<u32>(difficulty)) {
        (Some(id), Some(difficulty)) => (id, difficulty),
        _ => return Ok(None),
    };

    println!("Parsed:\nid: {}\ndifficulty: {}", id, difficulty);

    Ok(Some(Request { id, difficulty }))
}

fn encode(res: &Response, buf: &mut BytesMut) {
    let msg = match res.completion {
        Completion::Time(t) => format!("{} completed in {} milliseconds", res.id, t),
        Completion::OutOfTime => format!("{} ran out of time", res.id),
    };

    buf.extend(msg.to_string().as_bytes());
    buf.extend(b"\n");
}

pub struct TaskCodec;

impl Decoder for TaskCodec {
    type Item = Request;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Request>> {
        decode(buf)
    }
}

impl Encoder for TaskCodec {
    type Item = Response;
    type Error = io::Error;

    fn encode(&mut self, res: Response, buf: &mut BytesMut) -> io::Result<()> {
        encode(&res, buf);
        Ok(())
    }
}

pub struct TaskProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for TaskProto {
    type Request = Request;
    type Response = Response;
    type Transport = Framed<T, TaskCodec>;
    type BindTransport = io::Result<Framed<T, TaskCodec>>;

    fn bind_transport(&self, io: T) -> io::Result<Framed<T, TaskCodec>> {
        Ok(io.framed(TaskCodec))
    }
}

pub struct ComputingService {
    pub thread_pool: CpuPool,
    pub timeout: u64,
}

impl Service for ComputingService {
    type Request = Request;
    type Response = Response;

    type Error = io::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    // Produce a future for computing a response from a request.
    fn call(&self, req: Self::Request) -> Self::Future {
        let (id, difficulty) = (req.id, u64::from(req.difficulty));

        let computation = self.thread_pool.spawn_fn(move || {
            let now = Instant::now();

            for _ in 0..difficulty {}

            let elapsed = now.elapsed();

            let millisec = (elapsed.as_secs() * 1_000) + u64::from(elapsed.subsec_nanos() / 1_000_000);
            let computation_time: Result<u64, ()> = Ok(millisec);
            computation_time
        });

        let timer = Timer::default();
        let timed_computation = timer
            .timeout(computation, Duration::from_secs(self.timeout))
            .then(|result| match result {
                Ok(t) => Ok(Completion::Time(t)),
                _ => Ok(Completion::OutOfTime),
            });

        Box::new(timed_computation.map(move |completion| {
            Response { id, completion }
        }))
    }
}

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

    let server = TcpServer::new(TaskProto, addr);

    server.serve(move || {
        Ok(ComputingService {
            thread_pool: thread_pool.clone(),
            timeout: config.timeout,
        })
    });
}
