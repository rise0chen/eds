#[derive(Debug)]
pub enum Error {
    OutOfMemory,
    NotFindStart,
    CrcCheckError,
    NotFindEnd,
}
