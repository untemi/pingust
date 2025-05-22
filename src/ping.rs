use crate::error::*;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::{Instant, timeout};

pub async fn run(adrr: &str, deadline: u64) -> Result<Duration> {
    let future = TcpStream::connect(format!("{adrr}:80"));

    let start = Instant::now();
    timeout(Duration::from_millis(deadline), future).await??;
    Ok(start.elapsed())
}
