     fn main() {
        println!("Hello, This is the public service APS level checker");
                           // Using a vector to represent the fields
        let v = vec!["Office Admin", "Academic", "Lawyer", "Teacher"];

        println!("Please select your field from {:?}", v);
                    // To the user to input their Field
        let mut field = String::new();
        std::io::stdin().read_line(&mut field).expect("Failed to read input");
        let field = field.trim().to_lowercase();

        let mut role = String::new();
                                
                                // Handling the Office Admin Field using vectors to hold the roles

         if field == "office admin" {
         let role1 = vec!["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"];
         println!("What is your role in the {} field {:?}", field, role1);
         let mut input1 = String::new();
         std::io::stdin().read_line(&mut input1).expect("Failed to read input");
          role = input1.trim().to_lowercase();
         
         }

                            // Handling the Academic field   input is for the roles and field is for thefields 
        else if field == "academic" {
        let role2 = vec!["Research Assistant","PhD Candidate","Post-Doc Researcher","Senior Lecturer","Dean"];
        println!("What is your role in the {} field {:?}", field,role2);
        let mut input2 = String::new();
        std::io::stdin().read_line(&mut input2).expect("Failed to read input");
         role = input2.trim().to_lowercase();
        
        }

                    // Handling the Lawyer Field 

        else if field == "lawyer" {
        let role3 = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2","Senior Associate 3-4", "Partner"];
        println!("What is your role in the {} field {:?}", field, role3);
        let mut input3 = String::new();
        std::io::stdin().read_line(&mut input3).expect("Failed to read input");
         role = input3.trim().to_lowercase();
        
        }
                               // Handling the Teacher Field

        else if field == "teacher" {
        let role4 = vec!["Placement","Classroom Teacher","Senior Teacher", "Leading Teacher","Deputy Principal","Principal"];
        println!("What is your role in the {} field {:?}",field, role4);
        let mut input4 = String::new();
        std::io::stdin().read_line(&mut input4).expect("Failed to read input");
         role = input4.trim().to_lowercase();
        
        }
                                      // Prompt them to input their number of years  of experience

       println!("How many years of experience do you have (1 - 13)");
       let mut exp = String::new();
       std::io::stdin().read_line(&mut exp).expect("Failed to read input");
       let exp:u8 = exp.trim().parse().expect("Invalid Input");

       println!("You have {} years of experience in the {:?} field", exp, field);

                                  // Assigning the level for each level of experience or their roles

        if exp <= 2  || role == "intern" || role == "paralegal" || role == "placement" {
        println!("Your APS level is APS 1-2");
       }
          // For the second APS level

        else if exp >=3 && exp <=5 || role == "administrator" || role == "research assistant" || role == "junior associate" || role == "classroom teacher" {
        println!("Your APS level is APS 3-5");
      }
                // For the second APS level

        else if exp >=5 && exp<=8 || role == "senior administrator" || role == "phd candidate" || role == "associate" || role == "senior teacher"{
        println!("Your APS level is APS 5-8");
    }

                  // Third APS level
        else if exp >=8 && exp<=10 || role == "office manager" || role == "post-doc researcher" || role == "senior associate 1-2" || role == "leading teacher" {
        println!("Your APS level is EL1 8-10");
    }
                      
                      // Fourth APS level
        else if exp>=10 && exp<=13 || role == "director" || role == "senior lecturer" || role == "senior associate 3-4" || role == "deputy principal" {
        println!("Your APS level is APS EL2 10-13");
    }

        else if exp>= 13 || role == "ceo" || role == "dean" || role == "partner" ||role == "principal" {
            println!("Your APS level is SES");
        } 

      }
   
        

     
 