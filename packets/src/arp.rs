use ethernet::{EtherType, MacAddr};
use std::net::Ipv4Addr;

packet!(ArpPacket, MutArpPacket, 28);

getters!(ArpPacket
    pub fn hardware_type(&self) -> HardwareType {
        HardwareType(read_offset!(self.0.as_ref(), 0, u16, from_be))
    }

    pub fn protocol_type(&self) -> EtherType {
        EtherType(read_offset!(self.0.as_ref(), 2, u16, from_be))
    }

    pub fn hardware_length(&self) -> u8 {
        read_offset!(self.0.as_ref(), 4, u8)
    }

    pub fn protocol_length(&self) -> u8 {
        read_offset!(self.0.as_ref(), 5, u8)
    }

    pub fn operation(&self) -> Operation {
        Operation(read_offset!(self.0.as_ref(), 6, u16, from_be))
    }

    pub fn sender_mac_addr(&self) -> MacAddr {
        MacAddr::from_slice(&self.0.as_ref()[8..14])
    }

    pub fn sender_ip_addr(&self) -> Ipv4Addr {
        Ipv4Addr::from(read_offset!(self.0.as_ref(), 14, [u8; 4]))
    }

    pub fn target_mac_addr(&self) -> MacAddr {
        MacAddr::from_slice(&self.0.as_ref()[18..24])
    }

    pub fn target_ip_addr(&self) -> Ipv4Addr {
        Ipv4Addr::from(read_offset!(self.0.as_ref(), 24, [u8; 4]))
    }
);

impl<'a> MutArpPacket<'a> {
    /// Sets the hardware_type, hardware_length, protocol_type and
    /// protocol_length fields to correct values for an IPv4 over Ethernet
    /// packet.
    pub fn set_ipv4_over_ethernet_values(&mut self) {
        self.set_hardware_type(HardwareType::ETHERNET);
        self.set_protocol_type(EtherType::IPV4);
        self.set_hardware_length(6);
        self.set_protocol_length(4);
    }
}

setters!(MutArpPacket
    pub fn set_hardware_type(&mut self, hardware_type: HardwareType) {
        write_offset!(self.0, 0, hardware_type.value(), u16, to_be)
    }

    pub fn set_protocol_type(&mut self, protocol_type: EtherType) {
        write_offset!(self.0, 2, protocol_type.value(), u16, to_be)
    }

    pub fn set_hardware_length(&mut self, hardware_length: u8) {
        write_offset!(self.0, 4, hardware_length, u8, to_be);
    }

    pub fn set_protocol_length(&mut self, protocol_length: u8) {
        write_offset!(self.0, 5, protocol_length, u8, to_be);
    }

    pub fn set_operation(&mut self, operation: Operation) {
        write_offset!(self.0, 6, operation.value(), u16, to_be)
    }

    pub fn set_sender_mac_addr(&mut self, sender_mac: MacAddr) {
        self.0[8..14].copy_from_slice(sender_mac.as_ref());
    }

    pub fn set_sender_ip_addr(&mut self, sender_ip: Ipv4Addr) {
        self.0[14..18].copy_from_slice(&sender_ip.octets());
    }

    pub fn set_target_mac_addr(&mut self, target_mac: MacAddr) {
        self.0[18..24].copy_from_slice(target_mac.as_ref());
    }

    pub fn set_target_ip_addr(&mut self, target_ip: Ipv4Addr) {
        self.0[24..28].copy_from_slice(&target_ip.octets());
    }
);


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct HardwareType(pub u16);

impl HardwareType {
    pub const ETHERNET: HardwareType = HardwareType(1);

    pub fn value(&self) -> u16 {
        self.0
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Operation(pub u16);

impl Operation {
    pub const REQUEST: Operation = Operation(1);
    pub const REPLY: Operation = Operation(2);

    pub fn value(&self) -> u16 {
        self.0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    static MAC: [u8; 6] = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    static IP: [u8; 4] = [0xff, 0xff, 0xff, 0xff];

    macro_rules! arp_setget_test {
        ($name:ident, $set_name:ident, $value:expr, $offset:expr, $expected:expr) => {
            setget_test!(MutArpPacket, $name, $set_name, $value, $offset, $expected);
        }
    }

    arp_setget_test!(
        hardware_type,
        set_hardware_type,
        HardwareType(0xffff),
        0,
        [0xff, 0xff]
    );
    arp_setget_test!(
        protocol_type,
        set_protocol_type,
        EtherType(0xffff),
        2,
        [0xff, 0xff]
    );
    arp_setget_test!(hardware_length, set_hardware_length, 0xff, 4, [0xff]);
    arp_setget_test!(protocol_length, set_protocol_length, 0xff, 5, [0xff]);
    arp_setget_test!(operation, set_operation, Operation(0xffff), 6, [0xff, 0xff]);
    arp_setget_test!(sender_mac_addr, set_sender_mac_addr, MacAddr(MAC), 8, MAC);
    arp_setget_test!(
        sender_ip_addr,
        set_sender_ip_addr,
        Ipv4Addr::new(0xff, 0xff, 0xff, 0xff),
        14,
        IP
    );
    arp_setget_test!(target_mac_addr, set_target_mac_addr, MacAddr(MAC), 18, MAC);
    arp_setget_test!(
        target_ip_addr,
        set_target_ip_addr,
        Ipv4Addr::new(0xff, 0xff, 0xff, 0xff),
        24,
        IP
    );

    #[test]
    fn setters_incremental() {
        let mut backing_data = [0; 28];
        {
            let mut testee = MutArpPacket::new(&mut backing_data).unwrap();
            testee.set_hardware_type(HardwareType(1 << 8 | 2));
            testee.set_protocol_type(EtherType(3 << 8 | 4));
            testee.set_hardware_length(5);
            testee.set_protocol_length(6);
            testee.set_operation(Operation(7 << 8 | 8));
            testee.set_sender_mac_addr(MacAddr([9, 10, 11, 12, 13, 14]));
            testee.set_sender_ip_addr(Ipv4Addr::new(15, 16, 17, 18));
            testee.set_target_mac_addr(MacAddr([19, 20, 21, 22, 23, 24]));
            testee.set_target_ip_addr(Ipv4Addr::new(25, 26, 27, 28));
        }
        for (i, (expected, actual)) in (1u8..29).zip(backing_data.iter()).enumerate() {
            assert_eq!(expected, *actual, "Invalid byte at index {}", i);
        }
    }

    #[test]
    fn default_setter() {
        let mut backing_data = [0; 28];
        let mut testee = MutArpPacket::new(&mut backing_data).unwrap();
        testee.set_ipv4_over_ethernet_values();

        assert_eq!(
            HardwareType::ETHERNET,
            testee.as_immutable().hardware_type()
        );
        assert_eq!(EtherType::IPV4, testee.as_immutable().protocol_type());
        assert_eq!(6, testee.as_immutable().hardware_length());
        assert_eq!(4, testee.as_immutable().protocol_length());
    }
}
