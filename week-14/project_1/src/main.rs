use std::io;
use std::io::Read;
fn main() {
   

    let mut input1 = String::new();
    println!("What is your role.(Administrator, Project manager, Vendor, Employee, Customer)");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let role = input1.trim().to_lowercase();
    
    if role == "administrator" {
        let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }
    else if role == "project manager" {
        let mut file = std::fs::File::open("project_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }
    else if role == "vendor" {
        let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }
    else if role == "employee" {
        let mut file = std::fs::File::open("staff_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }
    else if role == "customer" {
        let mut file = std::fs::File::open("customer_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }
}
