               use std::io::Write;

        fn main() {
            let announce = "Week 9 - Rust Input and Output\n";
            let dept = "Department of Software Engineering";


            let mut file = std::fs::File::create("Data.txt").expect("Create failed");
            file.write_all("Welcome to rust programming\n".as_bytes()).expect("Write Failed");
            file.write_all(announce.as_bytes()).expect("Write Failed");
            file.write_all(dept.as_bytes()).expect("Write Failed");

            println!("\nData written to file.");
        }
