                        use std::io;
                fn main() {
                   
                   let v = vec!['C', 'O', 'M', 'P', 'U', 'T', 'E', 'R'];

                   println!("Enter an index number from 0-7");

                   let mut input1 = String::new();
                   io::stdin().read_line(&mut input1).expect("Failed to read line");
                      // Indexing

                let index:usize = input1.trim().parse().expect("Failed to read input");
                            
                            let ch: char = v[index];

                            println!("{} is the index number for {}", index, ch);
                



                }
