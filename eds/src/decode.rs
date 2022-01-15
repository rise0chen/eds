use crate::error::Error;
use crate::frame::*;
use crc16;

pub struct Decoder {
    lead_len: u8,
}
impl Decoder {
    pub fn new(min_lead_len: u8) -> Decoder {
        Decoder {
            lead_len: min_lead_len,
        }
    }

    pub fn find_start(&self, buf: &[u8]) -> Option<usize> {
        let lead_len = self.lead_len as usize;
        let mut i_start = None;
        for i in lead_len..buf.len() {
            if buf[i] == CHAR_START && buf[i - lead_len..i].iter().all(|x| *x == CHAR_LEAD) {
                i_start = Some(i);
            }
        }
        i_start
    }

    pub fn get_load<'a>(&self, buf: &'a [u8]) -> Result<&'a [u8], Error> {
        let i_start = if let Some(i) = self.find_start(buf) {
            i
        } else {
            return Err(Error::NotFindStart);
        };
        let load_len = (buf[i_start + Field::Length as usize] as usize) << 8
            | buf[i_start + Field::Length as usize + 1] as usize;
        let load = &buf[i_start + Field::Load as usize..i_start + Field::Load as usize + load_len];
        let crc = (buf[i_start + Field::Load as usize + load_len] as u16) << 8
            | buf[i_start + Field::Load as usize + load_len + 1] as u16;
        // CRC检验
        let check = crc16::State::<crc16::MODBUS>::calculate(load);
        if check != crc {
            return Err(Error::CrcCheckError);
        }
        if buf[i_start + Field::Load as usize + load_len + 2] != CHAR_END {
            return Err(Error::NotFindEnd);
        }
        Ok(load)
    }
}
