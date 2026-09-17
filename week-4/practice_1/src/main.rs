       fn main() {
              use std::io;
        // Student Information System
      
    println!("\nStudent Information Management System!");
      // Input User Name
    println!("\nInput Your Name Please");
        let mut name = String::new();
       io::stdin().read_line(&mut name).expect("Please Re-check the data provided");

       println!("Your name is {}", name);

         // Input User Age
         println!("\nInput Your Age Please");

         let mut age = String::new();
        io::stdin().read_line(&mut age).expect("Re-check the data provided");

        println!("Your Age is {}", age);


}
