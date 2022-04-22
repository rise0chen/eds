//! # EDS
//!
//! EDS(Easy Data Separator)，简易数据分割器，专门用于解决数据在传输时不连续、粘包问题的数据链路层协议。

#![no_std]

pub mod crc;
pub mod error;
pub mod frame;
