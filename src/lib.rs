#[cfg(test)]
#[macro_use]
extern crate lazy_static;

pub use decode::decode_utf8;
pub use encode::encode_utf8;

mod encode;
mod decode;

#[cfg(test)]
mod tests;
