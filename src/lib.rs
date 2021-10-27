//! lib.rs --- shed lib
//!
//! A `shed` is a collections of development resources such as code,
//! configs, docs, secrets, and data. It is intended to be an
//! `org-specific` structure which is maintained internally.
//!
//! This program is a development tool I use to manage such
//! structures.
//      _              _
//     | |            | |
//  ___| |__   ___  __| |
// / __| '_ \ / _ \/ _` |
// \__ \ | | |  __/ (_| |
// |___/_| |_|\___|\__,_|

#![feature(drain_filter)]

pub mod app;
pub mod cli;
pub mod config;
