use std::time::Duration;

use clap::Parser;
use handler::Handler;
use hickory_server::ServerFuture;
use options::Options;
use tokio::net::{TcpListener, UdpSocket};
use tracing::info;

mod handler;
mod options;

const TCP_TIMEOUT: Duration = Duration::from_secs(10);

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let options = Options::parse();
    let handler = Handler::from_options(&options);
    let mut server = ServerFuture::new(handler);
    for udp in &options.udp {
        server.register_socket(UdpSocket::bind(udp).await.unwrap());
        info!("Listening for UDP on {}", udp);
    }

    // register TCP listeners
    for tcp in &options.tcp {
        server.register_listener(TcpListener::bind(&tcp).await.unwrap(), TCP_TIMEOUT);
        info!("Listening for TCP on {}", tcp);
    }
    server.block_until_done().await.unwrap();
}
