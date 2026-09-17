    // Code to calculate the area of a triangle using base and height
       use std::io;
         fn  main() {
            let mut base = String::new();
            let mut height = String::new();

              // To Input Base
          println!("Enter Base of triangle: ");
          io::stdin().read_line(&mut base).expect("Not a valid input");
          let base:f32 = base.trim().parse().expect("Input is invalid");

            // For Height
            println!("Enter Height of triangle: ");
            io::stdin().read_line(&mut height).expect("Not a valid input");
            let height:f32 = height.trim().parse().expect("Input is invalid");

            // To calculate Area
              if base>0.0 {
                let area:f32 = (base * height) / 2.0;
                println!("The Area Of The Triangle is {}", area);
            }

             
         }