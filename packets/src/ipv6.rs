use std::net::Ipv6Addr;
use types::*;
use ip::Protocol;

packet!(Ipv6Packet, MutIpv6Packet, 40);

getters!(Ipv6Packet
    pub fn version(&self) -> u4 {
        read_offset!(self.0, 0, u8) >> 4
    }

    pub fn payload_length(&self) -> u16 {
        read_offset!(self.0, 4, u16, from_be)
    }

    pub fn next_header(&self) -> Protocol {
        Protocol(read_offset!(self.0, 6, u8))
    }

    pub fn hop_limit(&self) -> u8 {
        read_offset!(self.0, 7, u8)
    }

    pub fn source(&self) -> Ipv6Addr {
        Ipv6Addr::from(read_offset!(self.0, 8, [u8; 16]))
    }

    pub fn destination(&self) -> Ipv6Addr {
        Ipv6Addr::from(read_offset!(self.0, 24, [u8; 16]))
    }
);

setters!(MutIpv6Packet
    pub fn set_version(&mut self, version: u4) {
        let new_byte = (version << 4) | (read_offset!(self.0, 0, u8) & 0x0f);
        write_offset!(self.0, 0, new_byte, u8);
    }

    pub fn set_payload_length(&mut self, payload_length: u16) {
        write_offset!(self.0, 4, payload_length, u16, to_be);
    }

    pub fn set_next_header(&mut self, protocol: Protocol) {
        write_offset!(self.0, 6, protocol.value(), u8);
    }

    pub fn set_hop_limit(&mut self, hop_limit: u8) {
        write_offset!(self.0, 7, hop_limit, u8);
    }

    pub fn set_source(&mut self, source: Ipv6Addr) {
        write_offset!(self.0, 8, source.octets(), [u8; 16]);
    }

    pub fn set_destination(&mut self, destination: Ipv6Addr) {
        write_offset!(self.0, 24, destination.octets(), [u8; 16]);
    }
);


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! ipv6_setget_test {
        ($name:ident, $set_name:ident, $value:expr, $offset:expr, $expected:expr) => {
            setget_test!(MutIpv6Packet, $name, $set_name, $value, $offset, $expected);
        }
    }

    ipv6_setget_test!(version, set_version, 0xf, 0, [0xf0]);
    ipv6_setget_test!(payload_length, set_payload_length, 0xabcd, 4, [0xab, 0xcd]);
    ipv6_setget_test!(next_header, set_next_header, Protocol(123), 6, [123]);
    ipv6_setget_test!(hop_limit, set_hop_limit, 0x65, 7, [0x65]);
    ipv6_setget_test!(
        source,
        set_source,
        Ipv6Addr::new(0x2001, 1, 2, 3, 4, 5, 6, 0xabcd),
        8,
        [0x20, 0x01, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0xab, 0xcd]
    );
    ipv6_setget_test!(
        destination,
        set_destination,
        Ipv6Addr::new(0x2001, 1, 2, 3, 4, 5, 6, 0x1234),
        24,
        [0x20, 0x01, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0x12, 0x34]
    );
}
