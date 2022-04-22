//! 数据接收器

use bytes::{Bytes, BytesMut};
use eds_core::crc::get_crc;
use eds_core::frame::*;

const MAX_LEN: usize = 4096;

/// 接收器状态
enum Status {
    Lead(u8),
    Start,
    Length1,
    Length2,
    Load,
    Crc1,
    Crc2,
    End,
    Finish,
}

/// 接收器
pub struct Reader {
    lead_len: u8,
    /// 已处理的数据
    load: BytesMut,
    /// 负载总长度
    load_len: u16,
    /// 负载crc
    load_crc: u16,
    /// 接收状态
    status: Status,
}
impl Reader {
    /// 创建一个空的接收器
    pub fn new(lead_len: u8) -> Self {
        Reader {
            lead_len,
            load: BytesMut::new(),
            load_len: 0,
            load_crc: 0,
            status: Status::Lead(0),
        }
    }
    /// 接收器是否为空
    pub fn is_empty(&self) -> bool {
        self.load.is_empty()
    }
    /// 清空接收器中处理完的数据
    fn clean(&mut self) {
        self.load.clear();
        self.status = Status::Lead(0);
    }
    /// 处理一个字节
    fn recv_one(&mut self, res: u8) -> bool {
        match self.status {
            Status::Lead(mut count) => {
                if res == CHAR_LEAD {
                    count += 1;
                    if count >= self.lead_len {
                        self.status = Status::Start;
                    } else {
                        self.status = Status::Lead(count);
                    }
                } else {
                    self.clean();
                }
            }
            Status::Start => {
                if res == CHAR_LEAD {
                } else if res == CHAR_START {
                    self.status = Status::Length1;
                } else {
                    self.clean();
                }
            }
            Status::Length1 => {
                self.load_len = (res as u16) << 8;
                self.status = Status::Length2;
            }
            Status::Length2 => {
                self.load_len |= res as u16;
                if self.load_len as usize > MAX_LEN {
                    self.clean();
                    self.recv_one(res);
                } else {
                    self.status = Status::Load;
                }
            }
            Status::Load => {
                let _ = self.load.extend([res]);
                if self.load.len() >= self.load_len as usize {
                    self.load_crc = get_crc(&self.load);
                    self.status = Status::Crc1;
                }
            }
            Status::Crc1 => {
                if res == (self.load_crc >> 8) as u8 {
                    self.status = Status::Crc2;
                } else {
                    self.clean();
                    self.recv_one(res);
                }
            }
            Status::Crc2 => {
                if res == self.load_crc as u8 {
                    self.status = Status::End;
                } else {
                    self.clean();
                    self.recv_one(res);
                }
            }
            Status::End => {
                if res == CHAR_END {
                    self.status = Status::Finish;
                    return true;
                } else {
                    self.clean();
                    self.recv_one(res);
                }
            }
            Status::Finish => {
                self.clean();
                self.recv_one(res);
            }
        }
        return false;
    }
    /// 接收数据并找出一个数据帧，返回已处理数据的字节数
    pub fn recv(&mut self, buf: &[u8]) -> usize {
        for i in 0..buf.len() {
            let res = buf[i];
            let finish = self.recv_one(res);
            if finish {
                return i + 1;
            }
        }
        return buf.len();
    }
    /// 接收到了完整数据
    pub fn is_ready(&self) -> bool {
        if let Status::Finish = self.status {
            true
        } else {
            false
        }
    }
    /// 获取接收到的数据
    pub fn get_load(&mut self) -> Option<Bytes> {
        if let Status::Finish = self.status {
            let load = core::mem::replace(&mut self.load, BytesMut::new());
            self.status = Status::Lead(0);
            Some(load.freeze())
        } else {
            None
        }
    }
}
