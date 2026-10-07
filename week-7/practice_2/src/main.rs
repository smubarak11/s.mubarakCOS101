            use std::io;

             fn checker(){
                let mut input = String::new();
                    println!("Enter a Character");
                    io::stdin().read_line(&mut input).expect("Failed to read input");
                     let ch:char = input.trim().parse().expect("Invalid Input");

                      if ch >= '0' &&   ch <= '9' {
                        println!("Character {} is valid", ch);
                      }
                      else {
                        println!("Character {} is invalid", ch);
                      }
                  }

                      fn main() {
                            // reacalling the function
                        println!("Welcome! This program is to check if a charcter variable contains a dugut or not");
                      
                       checker()
 
                         }