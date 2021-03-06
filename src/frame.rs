use crate::packet;
use crate::parse;

use custom_debug_derive::*;
use derive_try_from_primitive::*;
use nom::{
    bytes::complete::take, combinator::map, error::context, number::complete::be_u16,
    sequence::tuple,
};
use std::fmt;

#[derive(Debug, TryFromPrimitive)]
#[repr(u16)]
pub enum EtherType {
    IPv4 = 0x0800,
    WIFI = 0x890d,
}

impl EtherType {
    pub fn parse(i: parse::Input) -> parse::Result<Option<Self>> {
        context("EtherType", map(be_u16, Self::try_from))(i)
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Addr(pub [u8; 6]);

impl fmt::Display for Addr {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        let [a, b, c, d, e, f] = self.0;
        write!(
            w,
            "{:02X}-{:02X}-{:02X}-{:02X}-{:02X}-{:02X}",
            a, b, c, d, e, f
        )
    }
}

impl fmt::Debug for Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Addr {
    pub fn new(slice: &[u8]) -> Self {
        let mut res = Self([0u8; 6]);
        res.0.copy_from_slice(&slice[..6]);
        res
    }

    pub fn parse(i: parse::Input) -> parse::Result<Self> {
        context("MAC address", map(take(6_usize), Self::new))(i)
    }
}

#[derive(Debug)]
pub enum Payload {
    IPv4(packet::Packet),
    WIFI(packet::Packet),
    Unknown,
}

#[derive(CustomDebug)]
pub struct Frame {
    pub dst: Addr,
    pub src: Addr,
    pub payload: Payload,
    pub ether_type: Option<EtherType>,
}

impl Frame {
    pub fn parse(i: parse::Input) -> parse::Result<Self> {
        context("Ethernet frame", |i| {
            let (i, (dst, src)) = tuple((Addr::parse, Addr::parse))(i)?;
            let (i, ether_type) = EtherType::parse(i)?;
            let (i, payload) = match ether_type {
                Some(EtherType::IPv4) => map(packet::Packet::parse, Payload::IPv4)(i)?,
                Some(EtherType::WIFI) => map(packet::Packet::parse, Payload::WIFI)(i)?,
                None => (i, Payload::Unknown),
            };

            let res = Self {
                dst,
                src,
                ether_type,
                payload,
            };
            Ok((i, res))
        })(i)
    }
}
