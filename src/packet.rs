use crate::parse;

use std::fmt;
use custom_debug_derive::*;
use derive_try_from_primitive::*;
use nom::{
    bytes::complete::take,
    combinator::map,
    error::context,
    number::complete::{be_u16, be_u8},
    sequence::tuple,
};

#[derive(CustomDebug)]
pub struct Packet {
    pub src: Addr,
    pub dst: Addr,
    #[debug(skip)]
    pub checksum: u16,
    #[debug(skip)]
    pub protocol: Option<Protocol>,
    payload: Payload,
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum Protocol {
    ICMP = 0x01,
    TCP = 0x06,
    UDP = 0x11,
}

#[derive(Debug)]
pub enum Payload {
    Unknown,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Addr(pub [u8; 4]);

impl fmt::Display for Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let [a, b, c, d] = self.0;
        write!(f, "{}.{}.{}.{}", a, b, c, d)
    }
}

impl fmt::Debug for Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Protocol {
    pub fn parse(i: parse::Input) -> parse::Result<Option<Self>> {
        // same as EtherType, this time with an u8.
        // note that `be_u8` is there for completeness, there's
        // no such thing as a little-endian or big-endian u8.
        context("IPv4 Protocol", map(be_u8, Self::try_from))(i)
    }
}

impl Addr {
    pub fn parse(i: parse::Input) -> parse::Result<Self> {
        let (i, slice) = context("IPv4 address", take(4_usize))(i)?;
        let mut res = Self([0, 0, 0, 0]);
        res.0.copy_from_slice(slice);
        Ok((i, res))
    }
}

impl Packet {
    pub fn parse(i: parse::Input) -> parse::Result<Self> {
        // skip over those first 9 bytes for now
        let (i, _) = take(9_usize)(i)?;
        let (i, protocol) = Protocol::parse(i)?;
        let (i, checksum) = be_u16(i)?;
        let (i, (src, dst)) = tuple((Addr::parse, Addr::parse))(i)?;
        let res = Self {
            protocol,
            checksum,
            src,
            dst,
            payload: Payload::Unknown,
        };
        Ok((i, res))
    }
}
