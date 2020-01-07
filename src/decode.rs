pub fn decode_utf8(src: &[u8]) -> Result<Vec<char>, DecodeUtf8Error> {
    let mut result = Vec::new();
    let mut index = 0;
    while index < src.len() {
        match decode_char(&src[index..]) {
            Ok((ch, len)) => {
                result.push(ch);
                index += len;
            }
            Err(kind) => {
                return Err(DecodeUtf8Error { src, index, kind });
            }
        }
    }
    Ok(result)
}

#[derive(Debug, Clone)]
pub struct DecodeUtf8Error<'a> {
    pub src: &'a [u8],
    pub index: usize,
    pub kind: DecodeUtf8ErrorKind,
}

#[derive(Debug, Clone)]
pub enum DecodeUtf8ErrorKind {
    MissingStartByte,
    InvalidStartByte,
    NotEnoughBytes,
    InvalidTrailingByte,
}

const MAX_ONE_BYTE: u8 = 0b1000_0000;
const MAX_MID_BYTE: u8 = 0b1100_0000;
const MAX_TWO_START_BYTE: u8 = 0b1110_0000;
const MAX_THREE_START_BYTE: u8 = 0b1111_0000;
const MAX_FOUR_START_BYTE: u8 = 0b1111_1000;

const NON_START_BYTE_MASK: u8 = 0b0011_1111;
const TWO_MASK: u8 = 0b0001_1111;
const THREE_MASK: u8 = 0b0000_1111;
const FOUR_MASK: u8 = 0b0000_0111;

fn decode_char(src: &[u8]) -> Result<(char, usize), DecodeUtf8ErrorKind> {
    let start = src[0];
    if start < MAX_ONE_BYTE {
        Ok((start as char, 1))
    } else if start < MAX_MID_BYTE {
        Err(DecodeUtf8ErrorKind::MissingStartByte)
    } else if start < MAX_TWO_START_BYTE {
        decode_multi_bytes(src, 2).map(|ch| (ch, 2))
    } else if start < MAX_THREE_START_BYTE {
        decode_multi_bytes(src, 3).map(|ch| (ch, 3))
    } else if start < MAX_FOUR_START_BYTE {
        decode_multi_bytes(src, 4).map(|ch| (ch, 4))
    } else {
        Err(DecodeUtf8ErrorKind::InvalidStartByte)
    }
}

fn decode_multi_bytes(src: &[u8], len: usize) -> Result<char, DecodeUtf8ErrorKind> {
    if src.len() < len {
        return Err(DecodeUtf8ErrorKind::NotEnoughBytes);
    }

    let mask = match len {
        2 => TWO_MASK,
        3 => THREE_MASK,
        4 => FOUR_MASK,
        _ => unreachable!(),
    };
    let mut raw_char = (src[0] & mask) as u32;

    for &byte in &src[1..len] {
        if byte < MAX_ONE_BYTE || byte >= MAX_MID_BYTE {
            return Err(DecodeUtf8ErrorKind::InvalidTrailingByte);
        }
        raw_char = (raw_char << 6) | (byte & NON_START_BYTE_MASK) as u32;
    }
    Ok(std::char::from_u32(raw_char).unwrap())
}
