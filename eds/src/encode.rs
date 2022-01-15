use crate::error::Error;
use crate::frame::*;
use crc16;

/// 编码器
pub struct Encoder<'a> {
    /// 前导长度
    lead_len: u8,
    /// 负载数据
    load: &'a [u8],
}

impl<'a> Encoder<'a> {
    pub fn new(lead_len: u8, load: &'a [u8]) -> Encoder<'a> {
        Encoder {
            lead_len: lead_len,
            load: load,
        }
    }
    pub fn set_load(&mut self, load: &'a [u8]) {
        self.load = load;
    }
    /// 数据编码为字节流
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, Error> {
        let lead_len = self.lead_len as usize;
        let load_len = self.load.len();
        let buf_len = lead_len + Field::Load as usize + load_len + 3;
        if buf.len() < buf_len {
            return Err(Error::OutOfMemory);
        }

        // 设置校验
        let check = crc16::State::<crc16::MODBUS>::calculate(self.load);
        for i in 0..lead_len {
            buf[i] = CHAR_LEAD;
        }
        buf[lead_len + Field::Start as usize] = CHAR_START;
        buf[lead_len + Field::Length as usize] = (load_len >> 8) as u8;
        buf[lead_len + Field::Length as usize + 1] = load_len as u8;
        buf[lead_len + Field::Load as usize..lead_len + Field::Load as usize + load_len]
            .copy_from_slice(self.load);
        buf[(buf_len as isize + Field::Check as isize) as usize] = (check >> 8) as u8;
        buf[(buf_len as isize + Field::Check as isize + 1) as usize] = check as u8;
        buf[(buf_len as isize + Field::End as isize) as usize] = CHAR_END;
        return Ok(buf_len);
    }
}
