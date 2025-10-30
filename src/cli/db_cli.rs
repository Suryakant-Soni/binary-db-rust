use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "dbtcp", version, about = "A simple DB file manager")]
pub struct DBCli{
    // Open a new file
    #[arg(short = 'n', long)]
    newfile: bool,
    
    // Create new employee
    #[arg(short = 'a', long)]
    addemployee: Option<String>,  
    
   #[arg(short = 'l', long)]
    // List all employees
    listemployees: bool, 
}

impl DBCli{
    fn get_new_file(&self) -> bool {
        self.newfile
    }
    fn get_list_employees(&self) -> bool {
        self.listemployees
    }
    fn get_add_employee(&self) -> &Option<String> {
       &self.addemployee
    }
   fn add_employee(&self) {
       println!("Adding employee from method");
   }
    pub fn run_cli(&self) {
        println!("Running CLI");
        if self.get_new_file() {
            println!("Creating new file");
        }
        if self.get_list_employees() {
            self.add_employee();
        }
        if let Some(argstring) = self.get_add_employee(){
            println!("Adding employee: {}", argstring);
        }
    }
}
