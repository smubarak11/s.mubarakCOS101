fn main() {
    let p:f64 = 10000.00;
    let r:f64 = 2.00;
    let t:f64 = 3.00;


      // simple interest 
      let a = p * ( 1.0 + (r / 100.0)) * t;
      println!("Amount is {}", a);
      let si = a - p;
      println!("Simple Interest is {}", si);
   }   