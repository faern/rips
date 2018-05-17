/// Represents the eight bit header field in IPv4/IPv6 that defines what protocol the payload has.
/// See [this list] for the full definition.
///
/// [this list]: https://en.wikipedia.org/wiki/List_of_IP_protocol_numbers
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Protocol(pub u8);

impl Protocol {
    pub const ICMP: Protocol = Protocol(1);
    pub const TCP: Protocol = Protocol(6);
    pub const UDP: Protocol = Protocol(17);
    pub const RESERVED: Protocol = Protocol(255);

    /// Returns the numeric representation of this protocol.
    #[inline]
    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn is_unassigned(&self) -> bool {
        self.0 >= 143 && self.0 <= 252
    }

    pub fn is_experimental(&self) -> bool {
        self.0 >= 253 && self.0 <= 254
    }
}
