mod frame;
mod parse;

use pcap::{Capture, Device, Linktype};

const DLT_IEEE802_11: i32 = 105;

fn main() {
    let interface: Device = Device::list()
	.unwrap()
	.into_iter()
	.find(|iface| iface.name == "en0")
	.unwrap();
    let mut capture = Capture::from_device(interface).unwrap()
	.rfmon(true)
	.open()
	.unwrap();

    // 105 =  data link type
    capture
	.set_datalink(Linktype(DLT_IEEE802_11))
	.expect("Invalid data link type set");
    
    while let Ok(packet) = capture.next() {
        process_packet(packet.data);
    }
}

fn process_packet(packet: &[u8]) {
    match frame::Frame::parse(packet) {
        Ok((remaining, frame)) => {
            if let Some(frame::EtherType::IPv4) = frame.ether_type {
                let protocol = remaining[9];
                println!("ipv4, protocol 0x{:02x}", protocol);
            } else {
                println!("non-ipv4!");
            }
        }
        Err(nom::Err::Error(e)) => {
            println!("{:?}", e);
        }
        _ => unreachable!(),
    }
}
