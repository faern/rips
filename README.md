# RIPS - Rust IP Stack

This is a work in progress to migrate over from the [older code base]. So far this "new" one has
very little, except the packet crate. So you probably want to check out the older one if you are
looking for working protocols.

[older code base]: https://github.com/faern/rips-old

## rips-packets

The `rips-packets` crate is a very fast and lightweight crate for reading and writing packet
headers and payloads of common network protocols.
