use atoi::atoi;
use bytes::BytesMut;
use std::io;
use tokio_io::codec::{Encoder, Decoder};

#[derive(Debug)]
pub enum Completion {
    Time(u64), // milliseconds
    OutOfTime,
}

#[derive(Debug)]
pub struct Request {
    pub id: u32,
    pub difficulty: u32,
}

pub struct Response {
    pub id: u32,
    pub completion: Completion,
}

#[derive(Debug)]
pub struct TaskCodec;

impl TaskCodec {
    pub fn new() -> Self {
        TaskCodec
    }
}

impl Decoder for TaskCodec {
    type Item = Request;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Request>> {
        println!("Received: {:?}", buf);

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

        println!("\tParsed:\n\tTask {}\n\tdifficulty: {}", id, difficulty);

        Ok(Some(Request { id, difficulty }))
    }
}

impl Encoder for TaskCodec {
    type Item = Response;
    type Error = io::Error;

    fn encode(&mut self, res: Response, buf: &mut BytesMut) -> io::Result<()> {
        let msg = match res.completion {
            Completion::Time(t) => format!("Task {} completed in {} milliseconds", res.id, t),
            Completion::OutOfTime => format!("{} ran out of time", res.id),
        };

        buf.extend(msg.as_bytes());
        buf.extend(b"\r\n");
        println!("Task {}: Response: {:?}", res.id, buf);

        Ok(())
    }
}
