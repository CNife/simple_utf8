pub fn encode_utf8(src: &[char]) -> Result<Vec<u8>, EncodeUtf8Error> {
    let mut result = Vec::new();
    for (index, ch) in src.iter().enumerate() {
        if let Some(error_kind) = encode_char(*ch, &mut result) {
            return Err(EncodeUtf8Error {
                src,
                index,
                kind: error_kind,
            });
        }
    }
    Ok(result)
}

#[derive(Debug, Clone)]
pub struct EncodeUtf8Error<'a> {
    pub src: &'a [char],
    pub index: usize,
    pub kind: EncodeUtf8ErrorKind,
}

#[derive(Debug, Clone)]
pub enum EncodeUtf8ErrorKind {
    InvalidCodePoint,
}

const ONE_MAX: u32 = (1 << 7) - 1;
const TWO_MAX: u32 = (1 << 11) - 1;
const THREE_MAX: u32 = (1 << 16) - 1;
const FOUR_MAX: u32 = (1 << 21) - 1;

const LOW_6_MASK: u32 = 0b111111;
const NON_START_MASK: u8 = 0b10000000;
const TWO_START_MASK: u8 = 0b11000000;
const THREE_START_MASK: u8 = 0b11100000;
const FOUR_START_MASK: u8 = 0b11110000;

fn encode_char(ch: char, result: &mut Vec<u8>) -> Option<EncodeUtf8ErrorKind> {
    let raw_char = ch as u32;
    if raw_char >= FOUR_MAX {
        Some(EncodeUtf8ErrorKind::InvalidCodePoint)
    } else {
        if raw_char < ONE_MAX {
            encode_one(raw_char, result);
        } else if raw_char < TWO_MAX {
            encode_two(raw_char, result);
        } else if raw_char < THREE_MAX {
            encode_three(raw_char, result);
        } else if raw_char < FOUR_MAX {
            encode_four(raw_char, result);
        } else {
            unreachable!()
        }
        None
    }
}

fn encode_one(raw_char: u32, result: &mut Vec<u8>) {
    result.push(raw_char as u8);
}

fn encode_two(raw_char: u32, result: &mut Vec<u8>) {
    let (raw_char, second) = encode_low_6_bits(raw_char);
    let first = (raw_char as u8) | TWO_START_MASK;
    result.push(first);
    result.push(second);
}

fn encode_three(raw_char: u32, result: &mut Vec<u8>) {
    let (raw_char, third) = encode_low_6_bits(raw_char);
    let (raw_char, second) = encode_low_6_bits(raw_char);
    let first = (raw_char as u8) | THREE_START_MASK;
    result.push(first);
    result.push(second);
    result.push(third);
}

fn encode_four(raw_char: u32, result: &mut Vec<u8>) {
    let (raw_char, forth) = encode_low_6_bits(raw_char);
    let (raw_char, third) = encode_low_6_bits(raw_char);
    let (raw_char, second) = encode_low_6_bits(raw_char);
    let first = (raw_char as u8) | FOUR_START_MASK;
    result.push(first);
    result.push(second);
    result.push(third);
    result.push(forth);
}

fn encode_low_6_bits(raw_char: u32) -> (u32, u8) {
    let low_6_bits = (raw_char & LOW_6_MASK) as u8;
    (raw_char >> 6, low_6_bits | NON_START_MASK)
}