use tokio::net::TcpStream;
use tokio::time::{Instant, timeout};

use crate::error::*;
use std::time::Duration;

// TODO: use actual ping rather than Tcp
pub async fn run(adrr: &str, deadline: u64) -> Result<Duration> {
    let future = TcpStream::connect(format!("{adrr}:80"));

    let start = Instant::now();
    timeout(Duration::from_millis(deadline), future).await??;
    Ok(start.elapsed())
}
