#[cfg(test)]
#[macro_use]
extern crate lazy_static;

pub use decode::decode_utf8;
pub use encode::encode_utf8;
pub use encode::EncodeUtf8Error;
pub use encode::EncodeUtf8ErrorKind;

mod decode;
mod encode;

#[cfg(test)]
mod tests;
