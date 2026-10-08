        use std::io;
                       // Function for area of trapezium

         fn area_of_trapezium (h:f32, sl:f32, ll:f32)-> f32 {
              h / 2.0 * (sl + ll)
             }

              // Function for Area of Rhombus
              fn area_of_rhombus(d1:f32, d2:f32)->f32 {
                d1 * d2 / 2.0
              }
                    // Function for area of parallelogram
                    fn area_of_parallelogram(b:f32, height:f32)->f32 {
                        b * height
             }
                        // Function for area of cube
                       fn surfacearea_cube(side:f32)->f32 {
                       6.0 * side * side
             }
                              // Function for volume of a cylinder
                              fn volume_of_cylinder(r:f32, h:f32) -> f32 {

                                3.142 * r * r * h  
                    }           

                  fn main() {
                 // Introductions

                 println!("\nThis is the MTH 101 shape calculator.");

                   // Display the shapes to calculate

                   println!("\nArea of Trapezium.\nArea of Rhombus.\nArea of Parallelogram.\nSurface area of cube.\nVolume of a Cylinder. ");

                 println!("\nWhat are you going to calculate today");

                 // Accept user input

                 let mut input = String::new();
                  io::stdin().read_line(&mut input).expect("Invalid Shape Inputted");
                 let shape = input.trim().to_lowercase();

               println!("You are calculating the {}", shape);
                  
                  // Handle input if input is trapezium

                  if shape == "area of trapezium" {
                    println!("\nInput the height of the Trapezium.");

                     let mut height = String::new();
                     io::stdin().read_line(&mut height).expect("Invalid Integer Input");
                     let h:f32 = height.trim().parse().expect("Invalid Input");

                     println!("\nInput the Smaller Length.");

                     let mut small_length = String::new();
                     io::stdin().read_line(&mut small_length).expect("Invalid Integer Input");
                     let sl:f32 = small_length.trim().parse().expect("Invalid Input");

                     println!("\nInput the Longer Length.");

                     let mut large_length = String::new();
                     io::stdin().read_line(&mut large_length).expect("Invalid Integer Input");
                     let ll:f32 = large_length.trim().parse().expect("Invalid Input");

                        let areat = area_of_trapezium(h, sl, ll);
                        println!("The area of the Trapezium is {}", areat);

                                                  }

                          // Handling the input if its rhombus

                          else if shape == "area of rhombus" {
                            println!("\nInput the first diagonal of the rhombus");

                            let mut dia1 = String::new();
                            io::stdin().read_line(&mut dia1).expect("Invalid Integer Input");
                            let d1:f32 = dia1.trim().parse().expect("Invalid Input");

                            println!("\nInput the second diagonal of the rhombus.");

                            let mut dia2 = String::new();
                            io::stdin().read_line(&mut dia2).expect("Invalid Integer Input");
                            let d2:f32 = dia2.trim().parse().expect("Invalid Input");
                                     let arear = area_of_rhombus(d1, d2);
                                     println!("The Area of the Rhombus is {}", arear);
                           
                          }
                                     // Handling the Parallelogram input

                               else if shape == "area of parallelogram" {
                                println!("\nInput the length of the base of the Parallelogram");

                                let mut base = String::new();
                                io::stdin().read_line(&mut base).expect("Invalid Integer Input");
                                let b:f32 = base.trim().parse().expect("Invalid Input");

                                println!("\nInput the Vertical Height of the Parallelogram.");

                                let mut vertical = String::new();
                                io::stdin().read_line(&mut vertical).expect("Invalid Integer Input");
                                let height:f32 = vertical.trim().parse().expect("Invalid Input");

                                    let areap = area_of_parallelogram(b , height); 
                                    println!("The area of the Parallelogram is {}", areap);
                                 }

                                           // Handling the input for surface area of cube

                                    else if shape == "surface area of cube" {
                                        println!("\nInput the length of the side of the cube");
                                        let mut side = String::new();
                                        io::stdin().read_line(&mut side).expect("Invalid Integer Input");
                                        let side:f32 = side.trim().parse().expect("Invalid Input");

                                        let areac = surfacearea_cube (side); 
                                        println!("The surface area of the cube is {}", areac);
                                        }

                                           // To handle the volume of a cylinder 
                                           
                                        else if shape == "volume of a cylinder" {
                                         println!("\nInput the height of the Cylinder.");

                                         let mut height = String::new();
                                         io::stdin().read_line(&mut height).expect("Invalid Integer Input ");
                                         let h:f32 = height.trim().parse().expect("Invalid Input");

                                         println!("\nInput the radius of the cylinder");

                                         let mut radius = String::new();
                                         io::stdin().read_line(&mut radius).expect("Invalid Integer Input");
                                         let r:f32 = radius.trim().parse().expect("Invalid Input");
                                              
                                              
                                         let volume =  volume_of_cylinder(r , h); 
                                         println!("The volume of the cylinder is {:?}", volume); 
                                        }

                             }
                  
            
               



        