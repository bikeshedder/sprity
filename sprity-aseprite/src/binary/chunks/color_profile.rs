use bitflags::bitflags;
use nom::{
    bytes::complete::take,
    combinator::{cond, flat_map, map},
};
use strum_macros::FromRepr;

use crate::binary::{
    errors::ParseResult,
    scalars::{dword, dword_size, fixed, word, Fixed, Word},
};

#[derive(Debug)]
pub struct ColorProfileChunk<'a> {
    fixed_gamma: Option<Fixed>,
    profile: ColorProfile<'a>,
}

#[derive(FromRepr)]
pub enum ColorProfileType {
    NoColorProfile,
    Srgb,
    EmbeddedICC,
    Unknown(Word),
}

bitflags! {
    pub struct ColorProfileFlags: Word {
        const FIXED_GAMMA = 0x01;
    }
}

#[derive(Debug)]
pub enum ColorProfile<'a> {
    NoColorProfile,
    Srgb,
    EmbeddedICC(&'a [u8]),
    Unknown(Word),
}

pub fn parse_color_profile(input: &[u8]) -> ParseResult<ColorProfileChunk> {
    let (input, profile_type) = word(input)?;
    let profile_type = ColorProfileType::from_repr(profile_type.into())
        .unwrap_or(ColorProfileType::Unknown(profile_type));
    let (input, flags) = word(input)?;
    let flags = ColorProfileFlags::from_bits_truncate(flags);
    let (input, fixed_gamma) = fixed(input)?;
    let (input, _) = take(8usize)(input)?;
    let (input, profile) = match profile_type {
        ColorProfileType::NoColorProfile => (input, ColorProfile::NoColorProfile),
        ColorProfileType::Srgb => (input, ColorProfile::Srgb),
        ColorProfileType::EmbeddedICC => {
            map(flat_map(dword, take), ColorProfile::EmbeddedICC)(input)?
        }
        ColorProfileType::Unknown(word) => (input, ColorProfile::Unknown(word)),
    };
    Ok((
        input,
        ColorProfileChunk {
            fixed_gamma: if flags.contains(ColorProfileFlags::FIXED_GAMMA) {
                Some(fixed_gamma)
            } else {
                None
            },
            profile,
        },
    ))
}
