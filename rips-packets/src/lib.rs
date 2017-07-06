//! # Overview
//!
//! Network protocol packet representations. types that encapsulate byte
//! slices (`&[u8]`) in a zero allocation, zero copy, zero-cost way and provide
//! getter and setter methods for the fields in the corresponding protocols.
//!
//! # Usage
//!
//! See the examples in `examples/`.
//!
//! # Credit and comparison
//!
//! This crate is heavily inspired by the `packet` module of
//! [pnet](https://github.com/libpnet/libpnet).
//! Basically this is a rewrite of that part of pnet with the purpose of being
//! more light weight and versatile. The packet code generation in pnet is
//! cool and very useful. But it brings in large and outdated dependencies
//! (`syntex`), and somewhat limits what a packet can do. In comparison
//! `rips-packets` aims to have no/very few dependencies at the cost of more
//! manual work to implement each protocol representation. A benefit of the
//! more manual implementations is that it is easy to hand optimize single
//! getters or setters if needed.

#[macro_use]
mod macros;

pub mod ethernet;
pub mod arp;
pub mod ipv4;

/// Link layer MAC address primitives.
pub mod macaddr;

/// Bit field type aliases.
pub mod types;
