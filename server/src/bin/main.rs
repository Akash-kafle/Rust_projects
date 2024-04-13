use init::*;
use std::net::TcpListener;

#[tokio::main]
async fn main() {
    print!("\x1B[2J\x1B[1;1H");
    let tcplistner = TcpListener::bind("127.0.0.1:8087").unwrap();
    let pool: ThreadPool = ThreadPool::new(MAX_THREAD as usize);
    for stream in tcplistner.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            async_std::task::spawn(async {
                handle_connection(stream).await;
            });
        })
        .await;
    }
}
