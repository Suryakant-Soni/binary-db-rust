pub const MAGIC: [u8; 4] = *b"DB01";
pub const FILE_NAME: &str = "employees.db";
pub const VERSION: u8 = 1;
pub const HEADER_LEN: u64 = 4 + 1 + 4 + 8;
