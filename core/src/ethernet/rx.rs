use rips_packets::ethernet::{ether_types, EtherType, EthernetPacket, MacAddr};
use std::io;

pub trait EthernetPayloadListener<E: ::std::error::Error> {
    fn recv(&mut self, data: &[u8]) -> Result<(), E>;
}

impl EthernetPayloadListener<io::Error> for () {
    fn recv(&mut self, _data: &[u8]) -> Result<(), io::Error> {
        Ok(())
    }
}

#[macro_export]
macro_rules! ethernet_rx {
    ($struct_name:ident, $error_struct_name:ident {
        $($ether_type:expr => [
            $listener_name:ident: $listener_type:ty,
            $error_name:ident: $error_type:ty
        ])*
    }) => (
        #[derive(Debug)]
        pub enum $error_struct_name {
            TooShortPacket,
            InvalidDestination(MacAddr),
            IgnoredEtherType(EtherType),
            $($error_name($error_type),)*
        }

        pub struct $struct_name {
            mac: MacAddr,
            $($listener_name: $listener_type,)*
        }

        impl $struct_name {
            pub fn new(mac: MacAddr $(,$listener_name: $listener_type)*) -> $struct_name {
                $struct_name {
                    mac,
                    $($listener_name,)*
                }
            }

            #[inline]
            pub fn recv(&mut self, data: &[u8]) -> Result<(), $error_struct_name> {
                use rips_packets::ethernet::BROADCAST_MAC;

                let packet = EthernetPacket::new(data).ok_or($error_struct_name::TooShortPacket)?;
                let destination = packet.destination();
                if destination == self.mac || destination == BROADCAST_MAC {
                    self.route(packet)
                } else {
                    Err($error_struct_name::InvalidDestination(destination))
                }
            }

            #[inline]
            fn route<'a>(&mut self, packet: EthernetPacket<'a>) -> Result<(), $error_struct_name> {
                let ether_type = packet.ether_type();
                $(if ether_type == $ether_type {
                    return self.$listener_name.recv(packet.payload())
                        .map_err(|e| $error_struct_name::$error_name(e))
                })*
                Err($error_struct_name::IgnoredEtherType(ether_type))
            }
        }
    )
}

ethernet_rx!(EthernetRx, EthernetRxError {
    ether_types::IPV4 => [ipv4: (), Ipv4Error: io::Error]
    ether_types::ARP => [arp: (), ArpError: io::Error]
});


#[cfg(test)]
mod tests {
    use super::*;
    use rips_packets::ethernet::{EtherType, MutEthernetPacket};
    use std::io;
    use std::sync::mpsc;

    pub struct ErrorListener;

    impl EthernetPayloadListener<io::Error> for ErrorListener {
        fn recv(&mut self, _data: &[u8]) -> Result<(), io::Error> {
            Err(io::ErrorKind::AddrNotAvailable.into())
        }
    }

    pub struct TestListener(mpsc::Sender<()>);

    impl EthernetPayloadListener<io::Error> for TestListener {
        fn recv(&mut self, _data: &[u8]) -> Result<(), io::Error> {
            self.0.send(()).unwrap();
            Ok(())
        }
    }

    ethernet_rx!(EmptyEthernetRx, EmptyEthernetRxError {});
    ethernet_rx!(ErrorEthernetRx, ErrorEthernetRxError {
        ether_types::ARP => [arp: ErrorListener, ArpError: io::Error]
    });
    ethernet_rx!(HappyEthernetRx, HappyEthernetRxError {
        ether_types::IPV4 => [ipv4: TestListener, Ipv4Error: io::Error]
        ether_types::ARP => [arp: TestListener, ArpError: io::Error]
    });

    static MY_MAC: MacAddr = MacAddr([0xff, 0x01, 0x02, 0x03, 0x04, 0x05]);
    static ZERO_MAC: MacAddr = MacAddr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0]);

    #[test]
    fn too_short_packet() {
        let mut rx = EmptyEthernetRx::new(MY_MAC);
        assert_matches!(rx.recv(&[0; 13]), Err(EmptyEthernetRxError::TooShortPacket));
    }

    #[test]
    fn empty_ethernet_rx() {
        let mut rx = EmptyEthernetRx::new(MY_MAC);
        assert_matches!(
            rx.recv(&[0; 14]),
            Err(EmptyEthernetRxError::InvalidDestination(actual_mac)) if actual_mac == ZERO_MAC
        );
    }

    #[test]
    fn ignored_ether_type() {
        let mut rx = ErrorEthernetRx::new(MY_MAC, ErrorListener);
        let mut data = [0u8; 14];
        {
            let mut packet = MutEthernetPacket::new(&mut data).unwrap();
            packet.set_destination(MY_MAC);
        }

        assert_matches!(
            rx.recv(&data),
            Err(ErrorEthernetRxError::IgnoredEtherType(EtherType(0)))
        );
    }

    #[test]
    fn next_level_error() {
        let mut rx = ErrorEthernetRx::new(MY_MAC, ErrorListener);
        let mut data = [0u8; 14];
        {
            let mut packet = MutEthernetPacket::new(&mut data).unwrap();
            packet.set_destination(MY_MAC);
            packet.set_ether_type(ether_types::ARP);
        }

        assert_matches!(
            rx.recv(&data).unwrap_err(),
            ErrorEthernetRxError::ArpError(ref e) if e.kind() == io::ErrorKind::AddrNotAvailable
        );
    }

    #[test]
    fn happy_path() {
        let (ipv4_tx, ipv4_rx) = mpsc::channel();
        let (arp_tx, arp_rx) = mpsc::channel();

        let mut rx = HappyEthernetRx::new(MY_MAC, TestListener(ipv4_tx), TestListener(arp_tx));
        let mut data = [0u8; 14];
        {
            let mut packet = MutEthernetPacket::new(&mut data).unwrap();
            packet.set_destination(MY_MAC);
            packet.set_ether_type(ether_types::IPV4);
        }

        // No listener was called yet
        assert!(ipv4_rx.try_recv().is_err());
        assert!(arp_rx.try_recv().is_err());

        // Make sure IPv4 listener is called
        assert!(rx.recv(&data).is_ok());
        assert!(ipv4_rx.try_recv().is_ok());
        assert!(arp_rx.try_recv().is_err());

        {
            let mut packet = MutEthernetPacket::new(&mut data).unwrap();
            packet.set_ether_type(ether_types::ARP);
        }
        // Make sure Arp listener is called
        assert!(rx.recv(&data).is_ok());
        assert!(ipv4_rx.try_recv().is_err());
        assert!(arp_rx.try_recv().is_ok());
    }
}
