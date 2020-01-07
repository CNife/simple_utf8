#[cfg(test)]
#[macro_use]
extern crate lazy_static;

pub use decode::decode_utf8;
pub use encode::encode_utf8;
pub use encode::EncodeUtf8Error;
pub use encode::EncodeUtf8ErrorKind;

mod encode;
mod decode;

#[cfg(test)]
mod tests;
