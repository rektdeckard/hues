use hues::{api::V2, Bridge, Light};

#[tokio::main]
async fn main() {
    // This is running on a core thread.

    let bridge = Bridge::with_addr(std::net::Ipv4Addr::new(10, 0, 0, 190))
        .heartbeat(std::time::Duration::from_secs(30));

    let bridge = Bridge::discover().await.unwrap();

    let blocking_task = tokio::task::spawn_blocking(|| {
        // This is running on a blocking thread.
        // Blocking here is ok.
    });

    // We can wait for the blocking task like this:
    // If the blocking task panics, the unwrap below will propagate the
    // panic.
    blocking_task.await.unwrap();
}
