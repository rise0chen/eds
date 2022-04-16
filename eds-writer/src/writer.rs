//! 数据发送器

use bytes::BytesMut;
use eds::crc::get_crc;
use eds::frame::*;

/// 接收器
pub struct Writer {
    lead_len: u8,
    fixed: BytesMut,
}
impl Writer {
    /// 新建发送器
    pub fn new(lead_len: u8) -> Self {
        let mut buf = BytesMut::new();
        for _ in 0..lead_len {
            buf.extend([CHAR_LEAD]);
        }
        buf.extend([CHAR_START]);
        buf.extend([0, 0]);
        Self {
            lead_len,
            fixed: buf,
        }
    }
    /// 待赋值的负载
    pub fn get_load(&mut self) -> BytesMut {
        let mut load = self
            .fixed
            .split_off(self.lead_len as usize + Field::Load as usize);
        load.clear();
        load
    }
    /// 获取编码后的数据
    pub fn get_data(&mut self, load: BytesMut) -> &[u8] {
        let len = load.len();
        let crc = get_crc(&load);
        self.fixed[self.lead_len as usize + Field::Length as usize] = (len >> 8) as u8;
        self.fixed[self.lead_len as usize + Field::Length as usize + 1] = (len) as u8;
        self.fixed.unsplit(load);
        self.fixed.extend([(crc >> 8) as u8, crc as u8, CHAR_END]);
        &self.fixed
    }
}
