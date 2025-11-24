pub struct Dbheader {
    pub magic: [u8; 4],
    pub version: u8,
    pub count: u32,
    pub filesize: u64,
}

pub struct Employee {
    pub name: String,
    pub address: String,
    pub hours: u32,
}
