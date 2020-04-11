mod cli;
mod dot11;
mod frame;
mod packet;
mod parse;

use cli::CLI;
use frame::Frame;
use pnet::datalink::{self, channel, Channel::Ethernet, DataLinkSender};
use std::{
    boxed::Box,
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

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
    let file = create_file().unwrap();

    loop {
        match rx.next() {
            Ok(packet) => {
                let frame: Frame = process_packet(packet);
                let content = format!("{:#?}\n\n", frame);
                write_file(&file, content).unwrap();
                send_death_frame(&tx, &frame);
            }
            Err(e) => {
                panic!("An error occurred while reading packet: {}", e);
            }
        }
    }
}

fn create_file() -> Result<File, std::io::Error> {
    let file_path = Path::new("out/packets.txt");
    let parent = file_path.parent().unwrap();

    create_dir_all(parent)?;

    return File::create(&file_path);
}

fn write_file(mut file: &File, content: String) -> Result<(), std::io::Error> {
    file.write_all(content.as_bytes())?;
    Ok(())
}

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
