use std::net::Ipv4Addr;
use types::*;

/// Bitmasks for the three bit flags field in IPv4
pub mod flags {
    use types::*;

    /// A bitmask with a one in the "Reserved" position.
    pub static RESERVED: u3 = 0b100;
    /// A bitmask with a one in the "Don't fragment" position.
    pub static DF: u3 = 0b010;
    /// A bitmask with a one in the "More fragments" position.
    pub static MF: u3 = 0b001;
}

packet!(Ipv4Packet, MutIpv4Packet, 20);

getters!(Ipv4Packet
    pub fn version(&self) -> u4 {
        read_offset!(self.0, 0, u8) >> 4
    }

    pub fn header_length(&self) -> u4 {
        read_offset!(self.0, 0, u8) & 0x0f
    }

    pub fn dscp(&self) -> u6 {
        read_offset!(self.0, 1, u8) >> 2
    }

    pub fn ecn(&self) -> u2 {
        read_offset!(self.0, 1, u8) & 0x03
    }

    pub fn total_length(&self) -> u16 {
        read_offset!(self.0, 2, u16, from_be)
    }

    pub fn identification(&self) -> u16 {
        read_offset!(self.0, 4, u16, from_be)
    }

    pub fn flags(&self) -> u3 {
        read_offset!(self.0, 6, u8) >> 5
    }

    pub fn dont_fragment(&self) -> bool {
        (self.flags() & flags::DF) != 0
    }

    pub fn more_fragments(&self) -> bool {
        (self.flags() & flags::MF) != 0
    }

    pub fn fragment_offset(&self) -> u13 {
        read_offset!(self.0, 6, u16, from_be) & 0x1fff
    }

    pub fn ttl(&self) -> u8 {
        read_offset!(self.0, 8, u8)
    }

    pub fn protocol(&self) -> Protocol {
        Protocol(read_offset!(self.0, 9, u8))
    }

    pub fn header_checksum(&self) -> u16 {
        read_offset!(self.0, 10, u16, from_be)
    }

    pub fn source(&self) -> Ipv4Addr {
        Ipv4Addr::from(read_offset!(self.0, 12, [u8; 4]))
    }

    pub fn destination(&self) -> Ipv4Addr {
        Ipv4Addr::from(read_offset!(self.0, 16, [u8; 4]))
    }
);

setters!(MutIpv4Packet
    pub fn set_version(&mut self, version: u4) {
        let new_byte = (version << 4) | (read_offset!(self.0, 0, u8) & 0x0f);
        write_offset!(self.0, 0, new_byte, u8);
    }

    pub fn set_header_length(&mut self, header_length: u4) {
        let new_byte = (read_offset!(self.0, 0, u8) & 0xf0) | (header_length & 0x0f);
        write_offset!(self.0, 0, new_byte, u8);
    }

    pub fn set_dscp(&mut self, dscp: u6) {
        let new_byte = (dscp << 2) | (read_offset!(self.0, 1, u8) & 0x03);
        write_offset!(self.0, 1, new_byte, u8);
    }

    pub fn set_ecn(&mut self, ecn: u2) {
        let new_byte = (read_offset!(self.0, 1, u8) & 0xfc) | (ecn & 0x03);
        write_offset!(self.0, 1, new_byte, u8);
    }

    pub fn set_total_length(&mut self, total_length: u16) {
        write_offset!(self.0, 2, total_length, u16, to_be);
    }

    pub fn set_identification(&mut self, identification: u16) {
        write_offset!(self.0, 4, identification, u16, to_be);
    }

    pub fn set_flags(&mut self, flags: u3) {
        let new_byte = (flags << 5) | (read_offset!(self.0, 6, u8) & 0x1f);
        write_offset!(self.0, 6, new_byte, u8);
    }

    pub fn set_fragment_offset(&mut self, fragment_offset: u13) {
        let new_byte = (read_offset!(self.0, 6, u16, from_be) & 0xe000) |
            (fragment_offset & 0x1fff);
        write_offset!(self.0, 6, new_byte, u16, to_be);
    }

    pub fn set_ttl(&mut self, ttl: u8) {
        write_offset!(self.0, 8, ttl, u8);
    }

    pub fn set_protocol(&mut self, protocol: Protocol) {
        write_offset!(self.0, 9, protocol.value(), u8);
    }

    pub fn set_header_checksum(&mut self, checksum: u16) {
        write_offset!(self.0, 10, checksum, u16, to_be);
    }

    pub fn set_source(&mut self, source: Ipv4Addr) {
        write_offset!(self.0, 12, source.octets(), [u8; 4]);
    }

    pub fn set_destination(&mut self, destination: Ipv4Addr) {
        write_offset!(self.0, 16, destination.octets(), [u8; 4]);
    }
);


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Protocol(pub u8);

impl Protocol {
    #[inline]
    pub fn value(&self) -> u8 {
        self.0
    }
}

pub mod protocols {
    use super::Protocol;

    pub const ICMP: Protocol = Protocol(1);
    pub const TCP: Protocol = Protocol(6);
    pub const UDP: Protocol = Protocol(17);
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! ipv4_setget_test {
        ($name:ident, $set_name:ident, $value:expr, $offset:expr, $expected:expr) => {
            setget_test!(MutIpv4Packet, $name, $set_name, $value, $offset, $expected);
        }
    }

    ipv4_setget_test!(version, set_version, 0xf, 0, [0xf0]);
    ipv4_setget_test!(header_length, set_header_length, 0xf, 0, [0x0f]);
    ipv4_setget_test!(dscp, set_dscp, 0x3f, 1, [0xfc]);
    ipv4_setget_test!(ecn, set_ecn, 0x3, 1, [0x3]);
    ipv4_setget_test!(total_length, set_total_length, 0xffbf, 2, [0xff, 0xbf]);
    ipv4_setget_test!(identification, set_identification, 0xffaf, 4, [0xff, 0xaf]);
    ipv4_setget_test!(flags, set_flags, 0b111, 6, [0xe0]);
    ipv4_setget_test!(
        fragment_offset,
        set_fragment_offset,
        0x1faf,
        6,
        [0x1f, 0xaf]
    );
    ipv4_setget_test!(ttl, set_ttl, 0xff, 8, [0xff]);
    ipv4_setget_test!(protocol, set_protocol, Protocol(0xff), 9, [0xff]);
    ipv4_setget_test!(
        header_checksum,
        set_header_checksum,
        0xfeff,
        10,
        [0xfe, 0xff]
    );
    ipv4_setget_test!(
        source,
        set_source,
        Ipv4Addr::new(192, 168, 15, 1),
        12,
        [192, 168, 15, 1]
    );
    ipv4_setget_test!(
        destination,
        set_destination,
        Ipv4Addr::new(168, 254, 99, 88),
        16,
        [168, 254, 99, 88]
    );

    #[test]
    fn getters_alternating_bits() {
        let backing_data = [0b1010_1010; 20];
        let testee = Ipv4Packet::new(&backing_data).unwrap();
        assert_eq!(0b1010, testee.version());
        assert_eq!(0b1010, testee.header_length());
        assert_eq!(0b101010, testee.dscp());
        assert_eq!(0b10, testee.ecn());
        assert_eq!(0b1010_1010_1010_1010, testee.total_length());
        assert_eq!(0b1010_1010_1010_1010, testee.identification());
        assert_eq!(0b101, testee.flags());
        assert!(!testee.dont_fragment());
        assert!(testee.more_fragments());
        assert_eq!(0b0_1010_1010_1010, testee.fragment_offset());
    }
}
