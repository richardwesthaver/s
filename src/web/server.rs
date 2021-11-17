//! web/server.rs --- Web HTTP Server
/*!
Generic HTTP server bound to a TCP socket. The server listens for
connections on a single port and responds to clients based on the
recv_ methods.
*/
use axum::Router;
use std::net::SocketAddr;
use tokio::sync::mpsc::Sender;
use tokio::sync::watch::{channel, Receiver, Sender as WatchSender};

pub struct WebServerConfig {
  socket: SocketAddr,
}

pub struct WebServer {
  socket: SocketAddr,
  router: Router,
  channel: Option<(Sender<u8>, Receiver<u8>)>,
}

impl WebServer {
  /// build a new 'WebServer' from 'WebServerConfig'.
  pub fn new(cfg: WebServerConfig) -> Self {
    WebServer {
      socket: cfg.socket,
      router: Router::new(),
      channel: None,
    }
  }
  /// init the channel by passing it the sender half of a watch
  /// and returning another receiver
  pub async fn init(&mut self, tx: Sender<u8>) -> WatchSender<u8> {
    let (tx1, rx) = channel(32);
    self.channel = Some((tx, rx));
    tx1
  }
}
