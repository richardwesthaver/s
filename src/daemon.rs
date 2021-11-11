//! daemon.rs --- shed daemon
pub enum DaemonState {
  Alive,
  Dead
}

pub enum DaemonFrame {
  Command,
  Query,
  Signal,
}
pub struct DaemonConfig;

pub struct Daemon;
