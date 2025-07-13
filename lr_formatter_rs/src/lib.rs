//! This crate provides safe, compatible reading and writing of Line Rider track file formats.
//!
//! Supported formats: LRB (read/write), SOL (read/write), JSON (read/write), TRK (read)
//!
//! # Quickstart
//!
//! ### Reading
//! ```rust
#![doc = include_str!("../examples/read-track.rs")]
//! ```
//!
//! ### Writing
//! ```no_run
#![doc = include_str!("../examples/write-track.rs")]
//! ```

pub mod formats;
pub mod track;
pub(crate) mod util;
