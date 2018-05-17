//! # Overview
//!
//! Network protocol packet representations. types that encapsulate byte
//! slices (`&[u8]`) in a zero allocation, zero copy, zero-cost way and provide
//! getter and setter methods for the fields in the corresponding protocols.
//!
//! All packet types in this crate are represented by two structs, one for immutable data (used to
//! read header fields) and one for mutable data (used to set header fields). These structs are
//! defined like this:
//!
//! ```rust
//! pub struct FooPacket<'a>(&'a [u8]);
//! pub struct MutFooPacket<'a>(&'a mut [u8]);
//! ```
//!
//! Creating an immutable packet is done with `FooPacket::new(a_slice_of_u8)` and a mutable one with
//! `MutFooPacket::new(a_mut_slice_of_u8)`. This returns a new packet instance after making sure the
//! given slice is at least as long as the header of a "Foo packet". The packet types has getters
//! and setters for each header field. A getter/setter only bitshifts, masks out and optionally do
//! endianess conversion of the bytes in the backing buffer, making the operations very cheap.
//!
//! # Usage
//!
//! See the examples in `examples/` for more examples.
//!
//! ```rust
//! extern crate rips_packets;
//!
//! use rips_packets::ethernet::{EthernetPacket, MacAddr, MutEthernetPacket, EtherType};
//!
//! fn main() {
//!     // Allocate a byte buffer that hold the bytes in the Ethernet frame.
//!     let mut buffer = [0; 14];
//!
//!     {
//!         // Lend the buffer mutably to `MutEthernetPacket` so it can manipulate the
//!         // header fields.
//!         let mut ethernet_packet = MutEthernetPacket::new(&mut buffer[..])
//!             .expect("Too short buffer");
//!
//!         // Use the setter methods to change the data in `buffer`
//!         ethernet_packet.set_destination(MacAddr::BROADCAST);
//!         ethernet_packet.set_source(MacAddr([0x01, 0x02, 0x03, 0x04, 0x05, 0x06]));
//!         ethernet_packet.set_ether_type(EtherType::IPV4);
//!     }
//!
//!     // Create an immutable representation of the ethernet frame based on the same
//!     // buffer. Where a mutable `MutEthernetPacket` has setters `EthernetPacket` has the
//!     // corresponding getters.
//!     let packet = EthernetPacket::new(&buffer[..]).expect("Too short buffer");
//!
//!     println!("Destination MAC: {}", packet.destination());
//!     println!("Source MAC: {}", packet.source());
//!     println!("EtherType: {:?}", packet.ether_type());
//!     println!("Packet data, including header: {:?}", packet.data())
//! }
//! ```
//!
//! # Prior art and comparison
//!
//! This crate is heavily inspired by `pnet_packet` from
//! [pnet](https://github.com/libpnet/libpnet).
//! Basically this is a rewrite of that part of pnet with the purpose of being
//! more light weight and versatile. The packet code generation in pnet is
//! cool and very useful. But it brings in large and outdated dependencies
//! (`syntex`), and somewhat limits what a packet can do. In comparison
//! `rips-packets` aims to have no/very few dependencies at the cost of more
//! manual work to implement each protocol representation. A benefit of the
//! more manual implementations is that it is easy to hand optimize single
//! getters or setters if needed.
//!
//! Compiling `rips-packets` takes under a second on a modern computer, whereas `pnet_packet` take
//! well over a minute on the same hardware.

#[macro_use]
extern crate bitflags;

#[macro_use]
mod macros;

/// Link layer primitives.
pub mod ethernet;

pub mod arp;
pub mod ip;
pub mod ipv4;
pub mod ipv6;


/// Bit field type aliases.
pub mod types;
