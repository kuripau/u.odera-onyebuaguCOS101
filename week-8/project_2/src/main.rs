use std::io;

fn employement() -> Vec<String> {
    let mut emplyment: Vec<String> = Vec::new();

    let mut input1 = String::new();
    println!("How many candidates were interviewed?");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let employment_number: i32 = input1.trim().parse().expect("Invalid input");

    for count in 0..employment_number {
        let mut input2 = String::new();
        println!("Enter interviewee name {}", count + 1);
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let new_employemnt: String = input2.trim().to_string();
        emplyment.push(new_employemnt);
    }

    emplyment 
}

fn yearss(names: &Vec<String>) -> Vec<u32> {
    let mut sponge_bob: Vec<u32> = Vec::new(); 

    for name in names {
        let mut input2_years = String::new();
        println!("Enter years of programming experience for {}", name);
        io::stdin().read_line(&mut input2_years).expect("Failed to read input");
        let new_years: u32 = input2_years.trim().parse().expect("Invalid input");
        sponge_bob.push(new_years);
    }

    sponge_bob
}

fn main() {
    println!("Welcome to Ey emplyment");

    let names = employement();
    let years = yearss(&names);
     let mut best_index = 0;
    for i in 0..years.len() {
        if years[i] > years[best_index] {
            best_index = i;
        }
    }
    if names.is_empty() {
        println!("No candidates entered.");
    } else {
        println!("Most experienced candidate: {} with {} year(s) of experience.",names[best_index], years[best_index]);
    }
}



    




