            use std::io::Write;

    fn main() {

        // Defining the header and the rows

     let header = ("LAGER", "STOUT" , "NON-ALCHOHOLIC");
     let row = ("33 Export", "Legend" , "Maltina");
     let row1 = ("Desperados", "Turbo King", "Amstel Malta");
     let row2 = ("Goldberg", "Williams", "Malta Gold");
     let row3 = ("Gulder", "", "Fayrouz");
     let row4 = ("Heineken" ,"", "");
     let row5 = ("Star" ,"", "",);

                        // Input for the header using commas as separators 
     let mut file = std::fs::File::create("drinks.csv").expect("Failed to create");
    file.write_all(header.0.as_bytes()).expect("Failed to write");
    file.write_all(",".as_bytes()).expect("Failed to write separator");
    file.write_all(header.1.as_bytes()).expect("Failed to write");
    file.write_all(",".as_bytes()).expect("Failed to write");
    file.write_all(header.2.as_bytes()).expect("Failed to write");
    file.write_all("\n".as_bytes()).expect("Failed to create a new line");

    file.write_all(row.0.as_bytes()).expect("Failed to write");
    file.write_all(",".as_bytes()).expect("Failed to write");
    file.write_all(row.1.as_bytes()).expect("Failed to write");
    file.write_all(",".as_bytes()).expect("Failed to write");
    file.write_all(row.2.as_bytes()).expect("Failed to write");
    file.write_all("\n".as_bytes()).expect("Failed to write");

    file.write_all(row1.0.as_bytes()).expect("Failed to write");
    file.write_all(",".as_bytes()).expect("Failed to write");
    file.write_all(row1.1.as_bytes()).expect("Failed to write");
    file.write_all(",".as_bytes()).expect("Failed to write");
    file.write_all(row2.2.as_bytes()).expect("Failed to write");
    file.write_all("\n".as_bytes()).expect("Failed to write");

    file.write_all(row2.0.as_bytes()).expect("Failed to write");
    file.write_all(",".as_bytes()).expect("Failed to write");
    file.write_all(row2.1.as_bytes()).expect("Failed to write");
    file.write_all(",".as_bytes()).expect("Failed to write");
    file.write_all(row2.2.as_bytes()).expect("Failed to write");
    file.write_all("\n".as_bytes()).expect("Failed to write");

    file.write_all(row3.0.as_bytes()).expect("Failed to write");
    file.write_all(",".as_bytes()).expect("Failed to write");
    file.write_all(row3.1.as_bytes()).expect("Failed to write");
    file.write_all(",".as_bytes()).expect("Failed to write");
    file.write_all(row3.2.as_bytes()).expect("Failed to write");
    file.write_all("\n".as_bytes()).expect("Failed to write");

    file.write_all(row4.0.as_bytes()).expect("Failed to write");
    file.write_all(",".as_bytes()).expect("Failed to write");
    file.write_all(row4.1.as_bytes()).expect("Failed to write");
    file.write_all(",".as_bytes()).expect("Failed to write");
    file.write_all(row4.2.as_bytes()).expect("Failed to write");
    file.write_all("\n".as_bytes()).expect("Failed to write");

    file.write_all(row5.0.as_bytes()).expect("Failed to write");
    file.write_all(",".as_bytes()).expect("Failed to write");
    file.write_all(row5.1.as_bytes()).expect("Failed to write");
    file.write_all(",".as_bytes()).expect("Failed to write");
    file.write_all(row5.2.as_bytes()).expect("Failed to write");

    println!("File has been successfully created");
             }








       

     

     
    
    