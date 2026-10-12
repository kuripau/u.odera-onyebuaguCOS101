use std::io;

fn main() {
    println!("Enter lower bound");
    let mut input_1 = String::new();
    io::stdin().read_line(&mut input_1).expect("Failed to read input");
    let lower_bound: i32 = input_1.trim().parse().expect("Failed to  input");

    println!("Enter the upper bound");
    let mut input_2 = String::new();
    io::stdin().read_line(&mut input_2).expect("Failed to read input");
    let upper_bound: i32 = input_2.trim().parse().expect("Failed to  input");

    for x in lower_bound..upper_bound { // upper bound is not inclusive
        println!("Count level is {}", x);
    }
}
