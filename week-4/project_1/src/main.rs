use std::io;

fn main() {
  println!("Enter the value of a: ");
  let mut a = String::new();
  io::stdin().read_line(&mut a).expect("Failed to read input");  
  let a: f64 = a.trim().parse().expect("Failed to input"); 

  println!("Enter the value of b: ");
  let mut b = String::new();
  io::stdin().read_line(&mut b).expect("Failed to read input");  
  let b: f64 = b.trim().parse().expect("Failed to input");

  println!("Enter the value of c: ");
  let mut c = String::new();
  io::stdin().read_line(&mut c).expect("Failed to read input");  
  let c: f64 = c.trim().parse().expect("Failed to input");

  // discriminant formula
  let d = b * b - 4.0 * a * c;

  if d > 0.0 {
    let root1 = (-b + d.sqrt()) / (2.0 * a);
    let root2 = (-b - d.sqrt()) / (2.0 * a);
    println!("The 2 roots are: {}, {}", root1, root2);
  } else if d == 0.0 {
    let root = -b / (2.0 * a);
    println!("The root is: {}", root);
  } else {
    println!("No real roots");
  }
}