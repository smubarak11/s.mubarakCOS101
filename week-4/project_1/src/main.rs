       use std::io;
        fn main(){

        let mut input1 = String::new();
        let mut input2 = String::new();
        let mut input3 = String::new();

           // For co-efficient of X^2
           println!("Input The Co-efficient of X^2. ");
          io::stdin().read_line(&mut input1).expect("Input is Invalid");
          let  a:f64 = input1.trim().parse().expect("Invalid Number");

          // For co-efficient of X
          println!("Input the Co-efficient of X. ");
          io::stdin().read_line(&mut input2).expect("Input is Invalid");
          let  b:f64 = input2.trim().parse().expect("Invalid Number");

          // For the Constant
          println!("Input Constant");
          io::stdin().read_line(&mut input3).expect("Input is Invalid");
          let  c:f64 = input3.trim().parse().expect("Invalid Number");

                   // Determinant And Calculation
                                            
                let d:f64 = b*b - 4.0*a*c;

                 let  x1:f64 = (-b + d.sqrt()) / (2.0 * a);
                 let  x2:f64 = (-b - d.sqrt()) / (2.0 * a);
                  // To Print The Answer Based on the determinant

                    if d > 0.0     {
        
            println!("The Equation Has Two Distincts roots {}, {}", x1, x2);
        }
            else if d  == 0.0
            { 
                println!("The Equation Has Exactly One Root {}, {}", x1, x2);
        }
             else if d < 0.0
             {
                println!(" The Equation Has No Real roots!!");
            }

           
                         

                 }

            







