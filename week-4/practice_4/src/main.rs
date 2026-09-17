  // Rust program to determine age limit
     use std::io;

       fn main() {
        let mut input1 = String::new();
        let mut input2 = String::new();

        // Name Input
         println!("Input Name Here. ");
          io::stdin().read_line(&mut input1).expect("Not a valid Input");

           // Age Input

           println!("Input Age Here. ");
           io::stdin().read_line(&mut input2).expect("Not a valid Input");
           let age:u8 = input2.trim().parse().expect("Not a valid String");

             // Age Verification
             if age >= 21 {
                println!("Welcome to the party {}!", input1);
             }   else {
                  println!("Oops, You are not allowed to enter the party {}", input1);
             }
         }

