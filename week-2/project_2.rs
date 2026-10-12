fn main() {
	 let amount = vec![450000,1500000,750000,2850000,250000];
	 let total_sum_amount: i32 = amount.iter().sum();
    println!("The total sum for amount is: {}",total_sum_amount );

    let qty = vec!{2,1,3,3,1};
    let total_sum_qty: i32 = qty.iter().sum();
    println!("the totla sum of cars is: {}",total_sum_qty );

    let average = total_sum_amount as f64 / total_sum_qty as f64;

    println!("average: {}", average);
}