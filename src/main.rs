mod frame;
mod parse;

use pcap::{Capture, Device, Linktype};

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

    // 105 = DLT_IEEE802_11 data link type
    capture
	.set_datalink(Linktype(105))
	.expect("Invalid data link type set.");
    
    while let Ok(packet) = capture.next() {
	process_packet(packet.data);
    }
}

fn process_packet(packet: &[u8]) {
    match frame::Frame::parse(packet) {
        Ok((_remaining, frame)) => {
            println!("{:?}", frame);
        }
        Err(nom::Err::Error(e)) => {
            println!("{:?}", e);
        }
        // this will crash *loudly* if our assumptions were wrong
        _ => unreachable!(),
    }
}
