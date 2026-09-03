  fn main() {

    let t: f64 = 2.0 * 450_000.0;
    let m: f64 = 1.0 * 1_500_000.0;
    let h: f64 = 3.0 * 750_000.0;
    let d: f64 = 3.0 * 2_850_000.0;
    let a: f64 = 1.0 * 250_000.0;

              // Total Sum of all items

          let s = t + m + h + d + a;
           println!("Total sum of sales record = {}", s);

                // Average sales 

                let avg = s / 5.0; 

           println!("Average of sales = {}",  avg);
                   }