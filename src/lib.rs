//! lib.rs --- shed lib
/*!
A `shed` is a collections of development resources such as code,
configs, docs, secrets, and data. It is intended to be an
`org-specific` structure which is maintained internally.

This library is included as a convenience and is implemented by the
individual programs in the `bin` folder.
*/
//      _              _
//     | |            | |
//  ___| |__   ___  __| |
// / __| '_ \ / _ \/ _` |
// \__ \ | | |  __/ (_| |
// |___/_| |_|\___|\__,_|
mod cli;
mod app;
mod config;
pub use self::{cli::build_cli, app::App, config::Config};
