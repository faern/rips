extern crate rips_packets;

use rips_packets::ethernet::{EthernetPacket, MacAddr, MutEthernetPacket, EtherType};

fn main() {
    // Allocate a byte buffer that represents the bytes in the Ethernet frame.
    let mut buffer = [0; 14];

    {
        // Lend the buffer mutably to `MutEthernetPacket` so it can manipulate the
        // header fields.
        let mut ethernet_packet = MutEthernetPacket::new(&mut buffer[..])
            .expect("Too short buffer");

        // Use the setter methods to change the data in `buffer`
        ethernet_packet.set_destination(MacAddr::BROADCAST);
        ethernet_packet.set_source(MacAddr([0x01, 0x02, 0x03, 0x04, 0x05, 0x06]));
        ethernet_packet.set_ether_type(EtherType::IPV4);

        // When `ethernet_packet` goes out of scope, the mutable borrow of `buffer` ends
        // and we can access the buffer again.
    }

    // Create an immutable representation of the ethernet frame based on the same
    // buffer. Where a mutable `MutFooPacket` has setters `FooPacket` has the
    // corresponding getters.
    let pkg = EthernetPacket::new(&buffer[..]).expect("Too short buffer");

    println!("Destination MAC: {}", pkg.destination());
    println!("Source MAC: {}", pkg.source());
    println!("EtherType: {:?}", pkg.ether_type());
    println!("Packet data, including header: {:?}", pkg.data())
}
