use std::net::TcpStream;
use std::sync::mpsc;
use std::thread::{self, sleep};
use std::time::Instant;

use crate::error::*;
use std::time::Duration;

// TODO: use actual ping rather than Tcp
// NOTE: doesnt seem posible hmmm
pub fn run(adrr: String, deadline: u64) -> Result<Duration> {
    let (tx, rx) = mpsc::channel();

    // ping task
    thread::spawn(move || {
        let start = Instant::now();

        if let Err(err) = TcpStream::connect(format!("{adrr}:80")).map_err(Error::Disconnect) {
            tx.send(Err(err)).unwrap();
        } else {
            let _ = tx.send(Ok(start.elapsed()));
        }
    });

    rx.recv_timeout(Duration::from_millis(deadline))
        .map_err(|_| Error::Timeout)?
}
