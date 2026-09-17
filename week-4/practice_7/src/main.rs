     use std::io;
      fn main() {
        // While Loop\
        println!("Enter a number");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to read input ");
        let mut num:i32 = input1.trim().parse().expect("Failed to input");
        

               while num > 10 {
                println!("inside loop value number is {}", num);
                num+=1;
        }
        println!("outside loop value number is {}", num);
    }