                fn main() {
                 // Creating a vector named city 
                 let mut city:Vec<String> = Vec::new();
                 // Print The City Vector
                 println!("The City vector has element {}",city.len());
                 // Push new elements into the vector 
                 let mut input1 = String::new();
                 println!("How many cities do you want to enter");
                 std::io::stdin().read_line(&mut input1).expect("Failed to read input");
                 let city_num:i32 = input1.trim().parse().expect("Invalid Input");
                for count in 0..city_num {
                    let mut input2 = String::new();
                    println!("Enter city {}", count+1);
                    std::io::stdin().read_line(&mut input2).expect("Failed to read Input");
                    let new_city:String = input2.trim().parse().expect("Invalid Input");
                    city.push(new_city);
                      }
                      println!("Your preferred ccities are :\n");
                      let mut count=1;
                      // Loop to iterate elements in vector
                      for i in city {
                        // iterating through  i on the vector
                        println!("{} {}",count, i);
                        count+=1
                      }

                }