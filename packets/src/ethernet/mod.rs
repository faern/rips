mod macaddr;
pub use self::macaddr::*;

packet!(EthernetPacket, MutEthernetPacket, 14);

getters!(EthernetPacket
    pub fn destination(&self) -> MacAddr {
        MacAddr::from_slice(&self.0[0..6])
    }

    pub fn source(&self) -> MacAddr {
        MacAddr::from_slice(&self.0[6..12])
    }

    pub fn ether_type(&self) -> EtherType {
        EtherType(read_offset!(self.0, 12, u16, from_be))
    }
);

setters!(MutEthernetPacket
    pub fn set_destination(&mut self, destination: MacAddr) {
        self.0[0..6].copy_from_slice(destination.as_ref());
    }

    pub fn set_source(&mut self, source: MacAddr) {
        self.0[6..12].copy_from_slice(source.as_ref());
    }

    pub fn set_ether_type(&mut self, ether_type: EtherType) {
        write_offset!(self.0, 12, ether_type.value(), u16, to_be)
    }
);


/// A representation of the 16 bit EtherType header field of an Ethernet packet.
///
/// A few select, commonly used, values are attached as associated constants. Their values are
/// defined on [IANA's website].
///
/// [IANA's website]: https://www.iana.org/assignments/ieee-802-numbers/ieee-802-numbers.xhtml
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct EtherType(pub u16);

impl EtherType {
    pub const IPV4: EtherType = EtherType(0x0800);
    pub const ARP: EtherType = EtherType(0x0806);
    pub const IPV6: EtherType = EtherType(0x86DD);

    #[inline]
    pub fn value(&self) -> u16 {
        self.0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    static MAC: [u8; 6] = [0xff; 6];

    macro_rules! eth_setget_test {
        ($name:ident, $set_name:ident, $value:expr, $offset:expr, $expected:expr) => {
            setget_test!(MutEthernetPacket, $name, $set_name, $value, $offset, $expected);
        }
    }

    eth_setget_test!(destination, set_destination, MacAddr(MAC), 0, MAC);
    eth_setget_test!(source, set_source, MacAddr(MAC), 6, MAC);
    eth_setget_test!(ether_type, set_ether_type, EtherType(0xffff), 12, [0xff; 2]);

    #[test]
    fn set_payload() {
        let mut backing_data = [0; 15];
        {
            let mut testee = MutEthernetPacket::new(&mut backing_data).unwrap();
            testee.payload()[0] = 99;
        }
        assert_eq!(99, backing_data[14]);
    }
}
