extern crate rips_packets;

use rips_packets::arp::{MutArpPacket, operations};
use rips_packets::ethernet::{BROADCAST_MAC, MacAddr, MutEthernetPacket, ether_types};

use std::net::Ipv4Addr;

fn main() {
    let src_mac = MacAddr([0x13, 0x37, 0xde, 0xad, 0xbe, 0xef]);
    let src_ip = Ipv4Addr::new(192, 168, 0, 150);
    let target_ip = Ipv4Addr::new(192, 168, 0, 1);

    let mut buffer = [0; 14 + 28];
    format_arp_request_frame(&mut buffer[..], src_mac, src_ip, target_ip)
        .expect("Unable to format frame");

    println!(
        "This is what an arp request for: IPv4 {}\nfrom: IPv4 {} and MAC {} looks like:\n{}",
        target_ip,
        src_ip,
        src_mac,
        buffer
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>()
            .join(" "),
    );
}

fn format_arp_request_frame(
    buffer: &mut [u8],
    src_mac: MacAddr,
    src_ip: Ipv4Addr,
    target_ip: Ipv4Addr,
) -> Result<(), &'static str> {
    static ERR_MSG: &str = "Too short buffer";

    let mut ethernet_packet = MutEthernetPacket::new(buffer).ok_or(ERR_MSG)?;
    format_broadcast_ethernet_arp(&mut ethernet_packet, src_mac);

    let mut arp_packet = MutArpPacket::new(ethernet_packet.payload()).ok_or(ERR_MSG)?;
    format_arp_request(&mut arp_packet, src_mac, src_ip, target_ip);
    Ok(())
}

fn format_broadcast_ethernet_arp<'a>(packet: &mut MutEthernetPacket<'a>, src_mac: MacAddr) {
    packet.set_destination(BROADCAST_MAC);
    packet.set_source(src_mac);
    packet.set_ether_type(ether_types::ARP);
}

fn format_arp_request<'a>(
    packet: &mut MutArpPacket<'a>,
    src_mac: MacAddr,
    src_ip: Ipv4Addr,
    target_ip: Ipv4Addr,
) {
    packet.set_ipv4_over_ethernet_values();
    packet.set_operation(operations::REQUEST);
    packet.set_sender_mac_addr(src_mac);
    packet.set_sender_ip_addr(src_ip);
    // packet.set_target_mac_addr(); // Is ignored in a request anyway
    packet.set_target_ip_addr(target_ip);
}
