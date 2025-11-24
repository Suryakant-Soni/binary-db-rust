use crate::constant;
use crate::file;
use clap::Parser;
use std::io;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(name = "dbtcp", version, about = "A simple DB file manager")]
pub struct DBCli {
    // Create new employee
    #[arg(short = 'a', long)]
    addemployee: Option<String>,

    #[arg(short = 'l', long)]
    // List all employees
    listemployees: bool,
}

impl DBCli {
    pub fn run_cli(&self) -> io::Result<()> {
        let mut file_obj = file::file::FileDb::orchestrate_file_db(Path::new(constant::FILE_NAME))?;
        if self.get_flag_list_employees() {
            let employees = file_obj.list_employees();
        }
        if let Some(t) = self.get_flag_add_employee() {
            file_obj.add_employee(t)
        }
    }
    fn get_flag_list_employees(&self) -> bool {
        self.listemployees
    }
    fn get_flag_add_employee(&self) -> &Option<String> {
        &self.addemployee
    }
}
