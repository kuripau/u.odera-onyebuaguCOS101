use std::io;

fn main() {
    println!("Enter if your experienced or not (true/false): ");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let experience: bool = experience.trim().parse().expect("Failed to input");

    check_experience(experience);

    fn check_experience(is_experienced: bool) {
        if is_experienced {
            println!("Enter your age: ");
            let mut age = String::new();
            io::stdin().read_line(&mut age).expect("Failed to read input");
            let age: f64 = age.trim().parse().expect("Failed to input");
            
            if age >= 40.0 {
                println!("Your incentive is 1,560,000");
            } else if age >= 30.0 && age < 40.0 {
                println!("Your incentive is 1,480,000");
            } else if age < 28.0 {
                println!("Your incentive is 1,300,000");
            } 
        } else {
            println!("Your incentive is 100,000");
        }
    }
}