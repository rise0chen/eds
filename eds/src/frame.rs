//! 数据帧格式

/// 前导
pub const CHAR_LEAD: u8 = 0x55;
/// 起始符
pub const CHAR_START: u8 = 0x54;
/// 结束符
pub const CHAR_END: u8 = 0x0A;

/// 数据帧最小长度(负载长度为0)
pub const MIN_LEN: usize = (Field::Load as isize - Field::Check as isize) as usize;

/// 数据帧字段相对位置
pub enum Field {
    Start = 0,
    Length = 1,
    Load = 3,
    Check = -3,
    End = -1,
}
