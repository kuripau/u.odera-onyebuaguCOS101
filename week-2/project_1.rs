fn main() {
 let p: f64 = 520000000.00;
    let n: f64 = 5.00;
    let r: f64 = 10.0;
//compund intrest formula:A-P
//where A =p(1+(r/100))^n
let a=p *(1.0+(r/100.0)).powf(n);
println!("amount is {}",a );
let ci = a - p;
println!("compound intrest is {}", ci);
}