                           use std::io::Write;

        fn main() {

                  // For the table header

        let header = ("Commissioner", "Ministry", "Zone");

               // Defining the Commissioner dataset using a vector

        let comm = vec![
        "Aigbogun Alamba Daudu", 
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona", 
        "Adewale Jimoh Akanbi", 
        "Osazuwa Faith Etieye"];
                  
                           // Defining Ministers dataset using a vector

        let min = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum", ];
           
                          // Defining the Zone dataset using a vector

        let zone = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East", ];
       

                   // Creating the csv file 
        
        let  mut file = std::fs::File::create("EFCC Merge.csv").expect("Failed to create file");
        
            // Writing the header

        file.write_all(header.0.as_bytes()).expect("Failed to write");
        file.write_all(",".as_bytes()).expect("Failed to write separator");
        file.write_all(header.1.as_bytes()).expect("Failed to write");
        file.write_all(",".as_bytes()).expect("Failed to write separator");
        file.write_all(header.2.as_bytes()).expect("Failed to write");
        file.write_all("\n".as_bytes()).expect("Failed to create a new line");


                 // For the first row index 0

        file.write_all(comm[0].as_bytes()).expect("Failed to write");
        file.write_all(",".as_bytes()).expect("Failed to write separator");
        file.write_all(min[0].as_bytes()).expect("Failed to write");
        file.write_all(",".as_bytes()).expect("Failed to write separator");
        file.write_all(zone[0].as_bytes()).expect("Failed to write");
        file.write_all("\n".as_bytes()).expect("Failed to create a new line");

                     //  For the second row index 1 

        file.write_all(comm[1].as_bytes()).expect("Failed to write");
        file.write_all(",".as_bytes()).expect("Failed to write separator");
        file.write_all(min[1].as_bytes()).expect("Failed to write");
        file.write_all(",".as_bytes()).expect("Failed to write separator");
        file.write_all(zone[1].as_bytes()).expect("Failed to write separator");
        file.write_all("\n".as_bytes()).expect("Failed to create a new line");

                    // For the third row index 2

        file.write_all(comm[2].as_bytes()).expect("Failed to write");
        file.write_all(",".as_bytes()).expect("Failed to write separator");
        file.write_all(min[2].as_bytes()).expect("Failed to write");
        file.write_all(",".as_bytes()).expect("Failed to write separator");
        file.write_all(zone[2].as_bytes()).expect("Failed to write");
        file.write_all("\n".as_bytes()).expect("Failed to crete a new line");

                         // For the forth row index 3

        file.write_all(comm[3].as_bytes()).expect("Failed to write");
        file.write_all(",".as_bytes()).expect("Failed to write separator");
        file.write_all(min[3].as_bytes()).expect("Failed to write");
        file.write_all(",".as_bytes()).expect("Failed to write separator");
        file.write_all(zone[3].as_bytes()).expect("Failed to write");
        file.write_all("\n".as_bytes()).expect("Failed to create a new line");

                     // Fifth row index 4

        file.write_all(comm[4].as_bytes()).expect("Failed to write");
        file.write_all(",".as_bytes()).expect("Failed to write separator");
        file.write_all(min[4].as_bytes()).expect("Failed to write");
        file.write_all(",".as_bytes()).expect("Failed to write separator");
        file.write_all(zone[4].as_bytes()).expect("Failed to create a new line");

                      }



            
        


              

        