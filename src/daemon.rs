//! daemon.rs --- shed daemon

pub enum DaemonState {
  Alive,
  Dead
}

pub struct DaemonCodec;

pub enum DaemonMessage {
  Start,
  Stop,
  Cmd,
}

pub struct DaemonResponse {
  obj: dyn Objective,
}

pub struct DaemonConfig;

pub struct Daemon;
