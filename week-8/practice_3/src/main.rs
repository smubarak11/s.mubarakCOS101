                  // To get print value 
                  fn value(n:Option<&char>) {
                    println!("Element of vector {:?}",n);
                  }
                                fn main() {
                let v = vec!['R', 'U', 'S', 'T', 'A', 'C', 'I', 'A', 'N'];
                    let mut input1 = String::new();
                    println!("Input a value between 1 and 8");
                    std::io::stdin().read_line(&mut input1).expect("Failed to read input");

                    // index no.
                    let index:usize = input1.trim().parse().expect("Invalid Input");

                     // Getting the value 
                     let ch:Option<&char> = v.get(index);

                     value(ch);
                                   }
