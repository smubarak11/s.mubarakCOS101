     
     fn main() {
             // Introduction to the restaurant

        println!("\nGood day and Welcome to the COS Restaurant");
        

        let p = String::from("Poundo Yam and Edinkainko Soup".to_string());
        let f = String::from("Fried Rice and Chicken".to_string());
        let a = String::from("Amala and Ewedu Soup".to_string());
        let e = String::from("Eba and Egusi Soup".to_string());
        let w = String::from("White Rice and Stew".to_string());

        println!(" We have \n{}, \n{}, \n{}, \n{}, \n{}", p,f,a,e,w);
                       // Customer Ordering Process
        use std::io;
        let mut  order = String::new();
        let mut total = 0;
        let mut firstorder = true; 
        loop {
            // clears the screen to avoid repating 
            order.clear();
            // condition to print different things during first and concesutive orders

            if firstorder{ 
                println!("What would you like to order. \nType done when you have ordered all you want so we can proceed to checkout.{}", order);
                firstorder = false;
        }          
        else {
            println!("Is that all (Type done if done, If not, What else would you like).");
        }
           
          io::stdin().read_line(&mut order).expect("Please check your order and Re-input");
        let realorder = order.trim().to_lowercase();

            if realorder == "done" {
                println!("Thanks for ordering from COS Restaurant");

                         break;   
                          }
                     
                          // match price to amount  
               let amount = match realorder.as_str() {
               "poundo yam and edinkainko soup" => 3200,
               "fried rice and chicken" => 3000,
               "amala and ewedu soup" => 2500,
                "eba and egusi soup" => 2000,
                "white rice and stew" => 2000,
                _=>0,       
            };
                         total += amount;
                         
                         let mut finalprice = total;
                                        
                         if total > 10_000 {
                            println!("You are eligible for our  discount.");
                             let discount = total * 10 / 100;
                             finalprice = total - discount;
                             println!("Your discount is N {}",discount)
                         
                        };
                              

                         println!("Your bill is now N {}", finalprice);
                        
                    }

             
           
        }
              
     
                


