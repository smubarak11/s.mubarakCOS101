        fn main() {
                                               // The Introduction
        println!("THIS IS THE INTERVIEW CHECKER FOR EY GLOBAL. ");

                        // TO DETERMINE HOW MANY PEOPLE WOULD BE INTERVIEWED AND THE HANDLING OF THAT NUMBER 

        println!("How many candidates do you want to interview ?");
        
        let mut cand = String::new();
        std::io::stdin().read_line(&mut cand).expect("Failed to read input");
        let cand:u8 = cand.trim().parse().expect("Invalid Input");

                                // New empty variable for the person with the highest experience both name and level of experience
        let mut high_name = String::new();
        let mut high_exp = String::new();

                        // For Loop that is to be terminated when the number interviwed corresponds is reached
        for count in 0..cand {

        let mut name = String::new();
                                  // Name Input
        println!("Please input the name of candidate {} .", count+1);
        std::io::stdin().read_line(&mut name).expect("Failed to read input");
        name = name.trim().to_string();

                                     // Experience  level input
        let mut exp = String::new();
        println!("Please input the number of years of programming experience you have.");
        std::io::stdin().read_line(&mut exp).expect("Failed to read input");
        exp = exp.trim().parse().expect("Invalid Input");
         

              // To track the highest experience level

         if exp > high_exp {
            high_exp = exp;           // automstically tracks each exp level and changes once it spots an higher one
            high_name = name;
         }
        }
       println!("Congratulations {}! You are the successful candidate with {} years of experience", high_name, high_exp);

       }

        


        