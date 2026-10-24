           use std::io::Write;

    fn main() {                      
                             // For the header of the table

        let header = ("STUDENT NAME", "MATRIC. NUMBER", "DEPARTMENT", "LEVEL");
 
                            // Creating a tuple for the information of the table because it is dynamically typed

        let students = vec![ 
        ("Oluchi Mordi", "ACC10211111","Accounting","300"),
        ("Adams Aliyu","ECO10110101","Economics","100"),
        ("Shania Bolade","CSC10328828","Computer","200"),
        ("Adekunle Gold", "EEE11020202", "Electrical","200"),
        ("Blanca Edemoh", "MEE10202001", "Mechanical", "100"),
        ];

                // Creating the csv file

        let mut file = std::fs::File::create("PAU-SMIS.csv").expect("Failed to create file");

                        // For the table header
                        
        file.write_all(header.0.as_bytes()).expect("Failed to write");
        file.write_all(",".as_bytes()).expect("Failed to write separator");
        file.write_all(header.1.as_bytes()).expect("Failed to write");
        file.write_all(",".as_bytes()).expect("Failed to write separator");
        file.write_all(header.2.as_bytes()).expect("Failed to write");
        file.write_all(",".as_bytes()).expect("Failed to write separator");
        file.write_all(header.3.as_bytes()).expect("Failed to write");
         file.write_all("\n".as_bytes()).expect("Failed to add new line");

                                 // For the first row of the table

        for row in &students{
                                   
                                    // passing the value of the row by refernce using a for loop  
       
        file.write_all(row.0.as_bytes()).expect("Failed to write");
        file.write_all(",".as_bytes()).expect("Failed to write separator");
        file.write_all(row.1.as_bytes()).expect("Failed to write");
        file.write_all(",".as_bytes()).expect("Failed to write separator");
        file.write_all(row.2.as_bytes()).expect("Failed to write");
        file.write_all(",".as_bytes()).expect("Failed to write separator");
        file.write_all(row.3.as_bytes()).expect("Failed to write");
        file.write_all("\n".as_bytes()).expect("Failed to add new line");

         }



    }