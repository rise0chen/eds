//! # EDS
//!
//! EDS(Easy Data Separator)，简易数据分割器，专门用于解决数据在传输时不连续、粘包问题的数据链路层协议。

#![no_std]

mod decode;
mod encode;

pub use eds_core::crc;
pub use eds_core::error;
pub use eds_core::frame;

pub use decode::Decoder;
pub use encode::Encoder;

#[cfg(feature = "alloc")]
pub use eds_reader::Reader;
#[cfg(feature = "alloc")]
pub use eds_writer::Writer;
