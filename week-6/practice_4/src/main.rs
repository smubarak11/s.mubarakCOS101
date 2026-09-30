     fn main() {
        let fullname = "Shefiu Mubarak";
        let department = "Software Engineering";
        let uni = "Pan-Atlantic University";

        // Using .push_str

        
        let mut school = "School of Science".to_string();

             school.push_str(" and Technology");



             println!("My name is {}", fullname);
             println!("The length of my full name is {}", fullname.len());

             println!("I am a student of {}", department);

             println!("{}", uni);
             println!("{}", school);
     }