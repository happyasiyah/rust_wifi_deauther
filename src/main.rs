mod frame;
mod parse;
mod packet;

use pnet::datalink;
use pnet::datalink::Channel::Ethernet;

fn main() {
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter()
                              .filter(|iface| iface.name == "en0")
                              .next()
                              .unwrap();

    let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e)
    };

    loop {
        match rx.next() {
            Ok(packet) => {
		process_packet(packet);
	    }
	    Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
	}
    }
	
}

fn process_packet(packet: &[u8]) {
    match frame::Frame::parse(packet) {
        Ok((_remaining, frame)) => {
            if let Some(frame::EtherType::IPv4) = frame.ether_type {
                println!("{:#?}", frame);
            }
        }
        Err(nom::Err::Error(e)) => {
            println!("{:?}", e);
        }
        _ => unreachable!(),
    }
}
