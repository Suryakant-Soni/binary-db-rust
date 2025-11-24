pub(crate) const MAGIC: [u8; 4] = *b"DB01";
pub(crate) const FILE_NAME: &str = "employees.db";

pub(crate) const VERSION: u8 = 1;
pub(crate) const HEADER_LEN: usize = 4 + 1 + 4 + 4;
