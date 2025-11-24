use crate::file::file;

use crate::constant::VERSION;
use crate::db::schema::*;
use std::convert::TryInto;
use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
impl file::FileDb {
    pub fn create_header() -> io::Result<Dbheader> {
        Ok(Dbheader {
            magic: crate::constant::MAGIC,
            count: 0,
            version: VERSION,
            filesize: std::mem::size_of::<Dbheader>() as u64,
        })
    }

    pub fn write_header(file: &mut File, header: &Dbheader) -> io::Result<()> {
        file.seek(SeekFrom::Start(0))?;
        // create a buffer which will be pushed to file
        let mut buf = [0u8; 17];
        buf[0..4].copy_from_slice(&header.magic);
        buf[4..5].copy_from_slice(&header.count.to_le_bytes());
        buf[5..9].copy_from_slice(&header.version.to_le_bytes());
        buf[9..17].copy_from_slice(&header.filesize.to_le_bytes());
        file.write_all(&buf)?;
        file.flush()?;
        Ok(())
    }

    pub fn read_header(file: &mut File) -> io::Result<Dbheader> {
        let mut buf = [0u8; 17];
        file.seek(SeekFrom::Start(0))?;
        file.read_exact(&mut buf)?;
        let magic = buf[0..4]
            .try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid magic"))?;
        let version = buf[4];
        let count = u32::from_le_bytes(
            buf[5..9]
                .try_into()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid count"))?,
        );
        let filesize = u64::from_le_bytes(
            buf[9..17]
                .try_into()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid filesize"))?,
        );
        Ok(Dbheader {
            magic,
            version,
            count,
            filesize,
        })
    }
    pub fn validate_header(header: &Dbheader, file: &mut File) -> io::Result<()> {
        if header.magic != crate::constant::MAGIC {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid magic"));
        }
        if header.version != crate::constant::VERSION {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid version",
            ));
        }
        // validate file size with header
        let current_pos = file.seek(SeekFrom::Current(0))?;
        let real_size = file.metadata()?.len();
        file.seek(SeekFrom::Start(current_pos))?;
        if real_size != header.filesize {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid size"));
        }
        Ok(())
    }
}
