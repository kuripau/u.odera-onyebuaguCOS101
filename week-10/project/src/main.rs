
struct Electronics{
    Brand:String,
    price:u32,
    

}

impl Electronics{
 fn calculate_purchase_cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    let brand1 = Electronics{
            Brand:String::from("Hp"),
            price:650000,
            
    };
    let brand2 = Electronics{
        Brand:String::from("IBM"),
        price:755000,
        
    };
    let brand3 = Electronics{
        Brand:String::from("Toshiba"),
        price:550000,
        
    };
    let brand4 = Electronics{
        Brand:String::from("Dell"),
        price:850000,
        
    };
    
     
      let quantity =  3;
      let cost1 = brand1.calculate_purchase_cost(quantity);
      let cost2 = brand2.calculate_purchase_cost(quantity);
      let cost3 = brand3.calculate_purchase_cost(quantity);
      let cost4 = brand4.calculate_purchase_cost(quantity);

      println!("The brand is {} and the price for 3 is {}", brand1.Brand, cost1);
      println!("The brand is {} and the price for 3 is {}", brand2.Brand, cost2);
      println!("The brand is {} and the price for 3 is {}", brand3.Brand, cost3);
      println!("The brand is {} and the price for 3 is {}", brand4.Brand, cost4);

let total_cost = cost1+cost2+cost3+cost4;
      println!("the total cost is {}", total_cost);
}


