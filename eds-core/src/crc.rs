use crc16;

pub fn get_crc(buf: &[u8]) -> u16 {
    crc16::State::<crc16::MODBUS>::calculate(buf)
}
