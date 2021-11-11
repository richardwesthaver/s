//! web.rs --- shed web server with a UDP-based controller
//!
//! This is to be deployed with a set of 'capabilities' which are
//! controlled by simple messages over UDP socket. All we're doing is
//! listening for message frames and interacting with a HTTP Client or
//! Server.
use tokio_util::udp::UdpFramed;
use tokio::net::UdpSocket;
use std::net::SocketAddr;
use bytes::{BytesMut, Bytes};
use tokio_stream::StreamExt;
use futures::SinkExt;
use crate::coding::Codec;

mod client;
mod server;

/// Configuration for the Web transport
pub struct WebConfig {
  socket: SocketAddr,
}

/// The commands available via UDP controller
pub enum WebCommand {
  /// List all available services and their status
  List,
  /// HTTP Client queries
  Fetch(String),
  /// Update Config
  Config(WebConfig),
  /// change the State of a service
  Signal(Signal),
}

#[derive(Debug)]
pub enum Signal {
  /// Re-/Initialize a service
  Init(String),
  /// Start a service
  Start(String),
  /// Stop a service
  Stop(String),
  Shutdown,
}

#[derive(PartialEq)]
pub struct CommandResponse;

pub struct CtrlSocket{
  socket: UdpFramed<Codec>,
  owner: Option<SocketAddr>,
}

impl CtrlSocket {
  pub async fn new(socket: SocketAddr) -> Self {
    let sock = UdpSocket::bind(socket).await.unwrap();
    println!("listening on {}", sock.local_addr().unwrap());
    CtrlSocket {
      socket: UdpFramed::new(sock, Codec::new()),
      owner: None,
    }
  }
  pub async fn recv_next(&mut self) -> Option<BytesMut> {
    let i = self.socket.next().await;
    if let Some(Ok((frame, addr))) = i {
      println!("OK {} => {}", addr, String::from_utf8_lossy(&frame));
      Some(frame)
    } else {None}
  }
  pub async fn respond(&mut self, sender: SocketAddr, buff: Bytes) -> &Self {
    let sock = &mut self.socket;
    sock.send((buff, sender)).await.unwrap();
    self.owner = Some(sender);
    self
  }
  pub fn owner(&self) -> Option<SocketAddr> {
    self.owner
  }
  pub fn local_addr(&self) -> SocketAddr {
    self.socket.get_ref().local_addr().unwrap()
  }
}

pub type Status = (bool, SocketAddr);

pub struct WebSentinel {
  stats: Vec<Status>,
  socket: CtrlSocket,
}

impl WebSentinel {
  pub async fn new(cfg: WebConfig) -> Self {
    WebSentinel {
      stats: vec![],
      socket: CtrlSocket::new(cfg.socket).await,
    }
  }
  pub async fn send_signal(&self, tx: Signal) -> CommandResponse {
    println!("{:?}", tx);
    CommandResponse
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[tokio::test]
  async fn test_ctrl_socket() {
    let addr = "127.0.0.1:0".parse().unwrap();
    let sock = CtrlSocket::new(addr).await;
    assert!(sock.owner() == None || sock.local_addr() == addr);
  }
  #[tokio::test]
  async fn test_sentinel() {
    let cfg = WebConfig { socket: "127.0.0.1:0".parse().unwrap() };
    let st = WebSentinel::new(cfg).await;
    let res = st.send_signal(Signal::Init(String::from("test"))).await;
    assert!(res == CommandResponse);
  }
}
