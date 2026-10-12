fn main() {
    let price: f64 = 210000.00;
    let rate: f64 = 5.0;
    let time: f64 = 3.0;
    
    //depreciation
    let a = price * (1.0 - rate / 100.0).powf(time);
    
    println!("The depreciation after 3 years is {}", a);
}