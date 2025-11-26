use crate::constant::HEADER_LEN;
use crate::db::schema::Employee;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

pub struct FileDb {
    file: File,
}

impl FileDb {
    pub fn orchestrate_file_db<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        // create file if not exist or opens the file
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path.as_ref())?;
        // check if the file is new or existing one
        let md = file.metadata()?;
        if md.len() == 0 {
            // create header
            let header = Self::create_header()?;
            //write header
            Self::write_header(&mut file, &header)?;
        } else {
            // file existing - fetch header
            let header = Self::read_header(&mut file)?;
            // validate header
            Self::validate_header(&header, &mut file)?;
        }
        Ok(Self { file })
    }

    pub fn write_one_employee(&mut self, employee: Employee) -> io::Result<(u64)> {
        self.file.seek(SeekFrom::End(0))?;
        let mut bytes_written = 0;
        // get name bytes, write it
        let name_bytes = employee.name.as_bytes();
        let address_bytes = employee.address.as_bytes();
        let name_len = name_bytes.len() as u32;
        let address_len = address_bytes.len() as u32;

        // write name
        // same with address
        //write hours
        self.file.write_all(&name_len.to_le_bytes())?;
        bytes_written += 4;
        self.file.write_all(name_bytes)?;
        bytes_written += name_bytes.len() as u64;
        self.file.write_all(&address_len.to_le_bytes())?;
        bytes_written += 4;
        self.file.write_all(address_bytes)?;
        bytes_written += address_bytes.len() as u64;
        self.file.write_all(&employee.hours.to_le_bytes())?;
        bytes_written += 4;
        Ok(bytes_written)
    }

    pub fn read_one_employee(&mut self) -> io::Result<(Option<(Employee, u64)>)> {
        let mut bytes_read: u64 = 0;
        let mut len_buf = [0u8; 4];
        match self.file.read_exact(&mut len_buf) {
            Ok(_) => {}
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                return Ok(None);
            }
            Err(e) => return Err(e),
        }
        bytes_read += 4;
        let mut field_len = u32::from_le_bytes(len_buf) as usize;
        let mut name_buf = vec![0u8; field_len];
        //read name bytes
        self.file.read_exact(&mut name_buf)?;
        bytes_read += field_len as u64;
        let name = String::from_utf8(name_buf)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid name"))?;

        //read length of address field
        self.file.read_exact(&mut len_buf)?;
        bytes_read += 4;
        field_len = u32::from_le_bytes(len_buf) as usize;
        let mut address_buf = vec![0u8; field_len];
        self.file.read_exact(&mut address_buf)?;
        bytes_read += field_len as u64;
        let address = String::from_utf8(address_buf)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid address"))?;
        let mut hours_buf = [0u8; 4];
        self.file.read_exact(&mut hours_buf)?;
        bytes_read += 4;
        let hours = u32::from_le_bytes(hours_buf);
        let emp = Employee {
            name,
            address,
            hours,
        };
        Ok(Some((emp, bytes_read)))
    }

    pub fn list_employees(&mut self) -> io::Result<(Vec<Employee>)> {
        self.file.seek(SeekFrom::Start(HEADER_LEN))?;
        let mut employees = Vec::<Employee>::new();
        loop {
            match self.read_one_employee()? {
                Some((employee, _)) => employees.push(employee),
                None => break,
            }
        }
        Ok(employees)
    }

    pub fn add_employee(&mut self, emp_string: &str) -> io::Result<()> {
        //split into vector
        let parts: Vec<&str> = emp_string.split(',').collect();
        if parts.len() != 3 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid employee details - need 3 values with comma separated",
            ));
        }
        let hours = parts[2]
            .parse::<u32>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid hours value"))?;
        let emp = Employee {
            name: parts[0].to_string(),
            address: parts[1].to_string(),
            hours: hours,
        };
        let size = self.write_one_employee(emp)?;
        Self::update_filesize_in_header(&mut self.file, size)?;
        Ok(())
    }
}
