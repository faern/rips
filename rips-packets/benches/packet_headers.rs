#![feature(test)]

extern crate rips_packets;
extern crate test;

use rips_packets::ethernet::{MutEthernetPacket, ether_types};
use rips_packets::ipv4::{self, MutIpv4Packet};
use rips_packets::macaddr::MacAddr;
use std::net::Ipv4Addr;
use test::{Bencher, black_box};

#[bench]
fn set_all_ethernet_fields(b: &mut Bencher) {
    let mut buffer = [0; 14];
    let source = MacAddr([0xff, 0x00, 0xff, 0x00, 0xff, 0x00]);
    let destination = MacAddr([0x00, 0xff, 0x00, 0xff, 0x00, 0xff]);
    b.iter(|| {
        let mut packet = MutEthernetPacket::new(black_box(&mut buffer[..])).unwrap();
        packet.set_destination(black_box(destination));
        packet.set_source(black_box(source));
        packet.set_ether_type(black_box(ether_types::ARP));
    });
}

#[bench]
fn set_all_ipv4_fields(b: &mut Bencher) {
    let mut buffer = [0; 20];
    let source = Ipv4Addr::new(192, 168, 0, 1);
    let destination = Ipv4Addr::new(192, 168, 0, 2);
    b.iter(|| {
        let mut packet = MutIpv4Packet::new(black_box(&mut buffer[..])).unwrap();
        packet.set_version(black_box(4));
        packet.set_header_length(black_box(5));
        packet.set_dscp(black_box(0));
        packet.set_ecn(black_box(0));
        packet.set_total_length(black_box(20));
        packet.set_identification(black_box(0x1337));
        packet.set_flags(black_box(0b011));
        packet.set_fragment_offset(black_box(13));
        packet.set_ttl(black_box(40));
        packet.set_protocol(black_box(ipv4::protocols::UDP));
        packet.set_header_checksum(black_box(0x1337));
        packet.set_source(black_box(source));
        packet.set_destination(black_box(destination));
    });
}
