          use std::io;
          fn main() {
            // To determine Incentives
             println!("INCENTIVES DETERMINER.");

             let mut input = String::new();
              let mut experience = String::new();
              

                       // Age Input
             println!("Input Your Age Please.");
             io::stdin().read_line(&mut input).expect("Invalid Input");
            let age:i8 = input.trim().parse().expect("Input is invalid");
                  // Experience Level Input

             println!("\nInput Your Experience Level (experienced , inexperienced).");
             io::stdin().read_line(&mut experience).expect("Invalid Input");
             let experience = experience.trim();

             
                   // To Match Incentive with age and experience

             if  age >= 40 && experience == "experienced" {
                println!("Your Incentive is 1,560,000");
             }
             else if age >= 30 && age <= 39 && "experienced" == experience {
                println!("Your Incentive is 1,480,000");
             }
             else if age <= 28 && experience == "experienced" {
                println!("Your Incentive is 1,300,000");
             }   
             else if experience == "inexperienced" {
                println!("Your Incentive is 100,000");
             }   

             


          }
