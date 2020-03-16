mod frame;
mod packet;
mod parse;
mod cli;
mod dot11;

use dot11::Dot11Frame;
use cli::CLI;
use frame::Frame;
use pnet::datalink::{self, channel, Channel::Ethernet, DataLinkSender};
use std::boxed::Box;


fn main() {
    let args = CLI::new();
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(|iface| iface.name == args.interface)
        .next()
        .unwrap();

    let (tx, mut rx) = match channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Datalink channel error: {}", e),
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                let frame: Frame = process_packet(packet);
                if args.verbose {
                    println!("{:#?}", frame);
                }
                send_death_frame(&tx, &frame);
            }
            Err(e) => {
                panic!("An error occurred while reading packet: {}", e);
            }
        }
    }
}

// The name of this function was a typo... But it's kind of an appropriate name...
fn send_death_frame(sender: &Box<dyn DataLinkSender>, frame: &Frame) {
    let src_mac_addr = frame.src;
    let dst_mac_addr = frame.dst;
}

fn process_packet(packet: &[u8]) -> frame::Frame {
    match frame::Frame::parse(packet) {
        Ok((_remaining, frame)) => {
            return frame;
        }
        Err(nom::Err::Error(e)) => {
            println!("{:?}", e);
            panic!();
        }
        _ => unreachable!(),
    }
}
