   // Program To Calculate The Area Of a triangle
       fn main() {
         use std::io;
               // Making the sides Mutable
           let mut side1 = String::new();
           let mut side2 = String::new();
           let mut side3 = String::new();

        // For The First Side Of The triangle
        println!("Enter first side of the triangle .");
         io::stdin().read_line(&mut side1).expect("Input is Incorrect");
         let a:f32 = side1.trim().parse().expect("Number is Invalid");

          // For the second side of the triangle
       println!("Enter second side of the triangle .");
       io::stdin().read_line(&mut side2).expect("Input is Incorrect");
       let b:f32 = side2.trim().parse().expect("Number is Invalid");

          // for the third side
          println!("Enter third side of the triangle .");
          io::stdin().read_line(&mut side3).expect("Input is Invalid");
          let c:f32 = side3.trim().parse().expect("Number is Invalid");

          // to calculate the area
             let s:f32 = (a + b + c) / 2.0;
             let mut area:f32 = s * (s-a) * (s-b) * (s-c);
             area = area.sqrt();

             println!("The Area Of Your Triangle Is {}", area);


                 }
            





           
       
