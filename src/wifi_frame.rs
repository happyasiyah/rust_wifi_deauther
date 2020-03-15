use crate::parse;
use crate::packet::{self, Payload};
use crate::frame::Addr;

use ux::*;
use custom_debug_derive::*;
use std::fmt;
use derive_try_from_primitive::*;
use nom::{
    bits::{
	bits,
	complete::take as take_bits
    },
    bytes::complete::take, combinator::map, error::context, number::complete::be_u16,
    sequence::tuple,
};

use crate::parse::{self, BitParsable};

#[derive(CustomDebug)]
pub struct FrameControl {
    #[debug(skip)]
    pub version: u2,

    #[debug(format = "{:b}")]
    pub ftype: u2,

    #[debug(format = "{}")]
    pub subType: u4,

    #[debug(format = "{:b}")]
    pub toDS: u1,
    
    #[debug(format = "{:b}")]   
    pub fromDS: u1,

    #[debug(format = "{:b}")]
    pub moreFragments: u1,

    #[debug(format = "{:b}")]
    pub retry: u1,

    #[debug(format = "{:b}")]
    pub powerMgmt: u1,
    
    #[debug(format = "{:b}")]    
    pub moreData: u1,

    #[debug(format = "{:b}")]
    pub WEP: u1,

    #[debug(format = "{:b}")]
    pub order: u1,
}

impl FrameControl {
    pub fn parse(i: parse::Input) -> parse::Result<Self> {
	let (i, (version, ftype, subtype)) = bits(tuple((u2::parse, u2::parse, u4::parse)))(i)?;
	let (i, (toDS,
		 fromDS,
		 moreFragments,
		 retry,
		 powerMgmt,
		 moreData,
		 WEP,
		 order)) = bits(tuple((u1::parse,
				       u1::parse,
				       u1::parse,
				       u1::parse,
				       u1::parse,
				       u1::parse,
				       u1::parse,
				       u1::parse)))(i)?;
	let res = Self {
	    version,
	    ftype,
	    subtype,
	    toDS,
	    fromDS,
	    moreFragments,
	    retry,
	    powerMgmt,
	    moreData,
	    WEP,
	    order,
	}
	Ok((i, res));
    }
}

#[derive(CustomDebug)]
pub struct WifiFrame {
    pub fc: FrameControl,

    #[debug(format = "{:b}")]
    pub duration: u2,

    pub addr1: Addr,

    pub addr2: Addr,

    pub addr3: Addr,

    pub addr4: Addr,

    #[debug(format = "{:b}")]
    pub seqControl: u2,

    // Skipping payload and CRC, since I don't know how to parse variable length bytes followed by more 2 more bytes...
}


impl WifiFrame {
    pub fn parse(i: parse::Input) -> parse::Result<Self> {
        context("802.11 frame", |i| {
            let (i, (fc, duration, add1, addr2)) = bits(tuple((FrameControl::parse, u2::parse, Addr::parse, Addr::parse)))(i)?;
	    let (i, (addr3, add4, seqControl)) = bits(tuple((Addr::parse, Addr::parse, u2::parse)))(i)?;
	    let res = Self {
		fc,
		duration
		addr1,
		addr2,
		addr3,
		addr4,
		seqControl,
	    }
	    Ok((i, res));
        })(i);
    }
}

