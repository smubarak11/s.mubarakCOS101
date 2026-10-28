           // structure for the laptop qualities
struct Laptop {
    brand:String,
    quantity:u8,
    price_each:u32
}
              // Logic to calculate the total cost\

impl Laptop {
    fn total_cost (&self)->u32 {
        self.price_each * 3  // The self to point to which brand and hardcoded 3 for the fixed quantity
    }
}
fn main() {
    // initialize the struct
    let lap1 = Laptop {
      brand:String::from("HP"),
      quantity:10,
      price_each:650_000
    };

    let lap2 = Laptop {
        brand:String::from("IBM"),
        quantity:8,
        price_each:755_000
    };

    let lap3 = Laptop {
        brand:String::from("Toshiba"),
        quantity:10,
        price_each:550_000
    };
    let lap4 = Laptop {
        brand:String::from("Dell"),
        quantity:4,
        price_each:850_000
    };

     println!("The quantity of each brand is {} is {}, {} is {}, {} is {},{} is {}", lap1.brand,lap1.quantity,lap2.brand,lap2.quantity,lap3.brand,lap3.quantity,lap4.brand,lap4.quantity);
                    // To calculate the total for each brand 

    let cost1 = lap1.total_cost();
    let cost2 = lap2.total_cost();
    let cost3 = lap3.total_cost();
    let cost4 = lap4.total_cost();  

    let overall_cost = cost1 + cost2 + cost3 + cost4;
      
                                      // To print the total for each brand

     println!("\nThe total cost for 3 units of {}, {}, {}, {} each is {}", lap1.brand, lap2.brand, lap3.brand, lap4.brand, overall_cost);
    
}