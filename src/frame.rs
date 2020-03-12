use crate::parse;
use std::fmt;
use derive_try_from_primitive::*;
use nom::{
    bytes::complete::take, combinator::map, error::context, number::complete::be_u16,
    sequence::tuple,
};

#[derive(Debug, TryFromPrimitive)]
#[repr(u16)]
pub enum EtherType {
    IPv4 = 0x0800,
    ARP = 0x0806,
    IPv6 = 0x86DD,
}

impl EtherType {
    pub fn parse(i: parse::Input) -> parse::Result<Option<Self>> {
        context("EtherType", map(be_u16, Self::try_from))(i)
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Addr([u8; 6]);

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
        // note: this will panic if the slice is too small!
        res.0.copy_from_slice(&slice[..6]);
        res
    }
    
    pub fn parse(i: parse::Input) -> parse::Result<Self> {
        context("MAC address", map(take(6_usize), Self::new))(i)
    }
}

#[derive(Debug)]
pub struct Frame {
    pub dst: Addr,
    pub src: Addr,
    pub ether_type: Option<EtherType>,
}

impl Frame {
    pub fn parse(i: parse::Input) -> parse::Result<Self> {
        context(
            "Ethernet frame",
            map(
                tuple((Addr::parse, Addr::parse, EtherType::parse)),
                |(dst, src, ether_type)| Self {
                    dst,
                    src,
                    ether_type,
                },
            ),
        )(i)
    }
}
