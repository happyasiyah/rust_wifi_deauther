mod frame;
mod packet;
mod parse;

use frame::{Frame};
use pnet::datalink::{self, Channel::Ethernet, MacAddr};
use pnet::packet::ethernet::{MutableEthernetPacket};

fn main() {
    let interfaces = datalink::interfaces();
    let interface = interfaces
	.into_iter()
	.filter(|iface| iface.name == "en0")
	.next()
	.unwrap();

    let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
	Ok(Ethernet(tx, rx)) => (tx, rx),
	Ok(_) => panic!("Unhandled channel type"),
	Err(e) => panic!("Datalink channel error: {}", e),
    };

    loop {
	match rx.next() {
	    Ok(packet) => {
		let frame: Frame = process_packet(packet);
		tx.build_and_send(1, packet.len(), &mut |new_packet| {
		    let mut new_packet: MutableEthernetPacket = MutableEthernetPacket::new(new_packet).unwrap();
		    let source = frame.src.0;
		    let dest = frame.dst.0;
		    new_packet.set_ethertype(pnet::packet::ethernet::EtherType::new(3));
                    new_packet.set_source(MacAddr::new(
			source[0],
			source[1],
			source[2],
			source[3],
			source[4],
			source[5]));
                    new_packet.set_destination(MacAddr::new(
			dest[0],
			dest[1],
			dest[2],
			dest[3],
			dest[4],
			dest[5]));
		});
		panic!();
	    }
	    Err(e) => {
		panic!("An error occurred while reading packet: {}", e);
	    }
	}
    }
}

fn process_packet(packet: &[u8]) -> frame::Frame {
    match frame::Frame::parse(packet) {
	Ok((_remaining, frame)) => {
	    // if let Some(frame::EtherType::IPv4) = frame.ether_type {
	    // 	println!("{:#?}", frame);
	    // }
	    return frame;
	}
	Err(nom::Err::Error(e)) => {
	    println!("{:?}", e);
	    panic!();
	}
	_ => unreachable!(),
    }
}
