use std::io;



fn office() {
    println!("What level of experience are you?");
    let mut admin = String::new();
    io::stdin().read_line(&mut admin).expect("Failed to read input");
    let admin_choice: u32 = admin.trim().parse().expect("failed input");

    if admin_choice <= 2 {
        println!("You are an intern");
    } else if admin_choice <= 5 {
        println!("You are an administrator");
    } else if admin_choice <= 8 {
        println!("You are a senior administrator");
    } else if admin_choice <= 10 {
        println!("Office Manager");
    } else if admin_choice <= 13 {
        println!("Director");
    } else {
        println!("You are a CEO");
    }
}

fn academic() {
    println!("What level of experience are you?");
    let mut aca = String::new();
    io::stdin().read_line(&mut aca).expect("Failed to read input");
    let aca_choice: u32 = aca.trim().parse().expect("Failed input");

    if aca_choice <=2 {
        println!("You are nothing lmao");
    }
   else if aca_choice <= 5{
        println!("You are a research assistant");
    }
   else if aca_choice <=8{
        println!("You are a PHd candidate");
    }
  else  if aca_choice <=10{
        println!("Past-Doc Researcher");
    }
  else  if aca_choice <=12{
        println!("Senior Lecturer");
    }
    else {
        println!("Dean");
    }
}

fn lawyer() {
    println!("What level of experience are you?");
    let mut law = String::new();
    io::stdin().read_line(&mut law).expect("Failed to read input");
    let law_choice: u32 = law.trim().parse().expect("Failed input");

    if law_choice <=2 {
        println!("You are a paralegal");
    }
    else if law_choice <=5 {
        println!("Junior Associate");
    }
    else if law_choice <=8 {
        println!("Associate");
    }
    else if law_choice <=10 {
        println!("Senior Associtate 1-2");
    }
    else if law_choice <=13 {
        println!("Senior Associtate 3-4");
    }
    else {
        println!("Partner");
    }
}

fn teacher() {
    println!("What level of experience are you?");
    let mut teach = String::new();
    io::stdin().read_line(&mut teach).expect("Failed to read input");
    let teach_choice: u32 = teach.trim().parse().expect("Failed input");

    if teach_choice <=2 {
        println!("Placement");
    }
    else if teach_choice <=5 {
        println!("Classroom Teacher");
    }
    else if teach_choice <=8 {
        println!("Senior Teacher");
    }
    else if teach_choice <=10 {
        println!("Leading Teacher");
    }
    else if teach_choice <=13{
        println!("Deputy Principal");
    }
    else {
        println!("Principlal");
    }
}


fn main() {
     let titak: Vec<fn()> = vec![office, academic, lawyer, teacher];
    println!("Welcome to the public service library, what are you?");
    println!("Office Admin");
    println!("Academic");
    println!("Lawyer");
    println!("Teacher");

    let (chudi, name) = loop {
        println!("Enter the letter corresponding to your service (OA,A,L,T)");

        let mut psl = String::new();
        io::stdin().read_line(&mut psl).expect("Failed to read input");
        let chudi = psl.trim().to_uppercase();

        match chudi.as_str() {
            "OA" => break (chudi, "Office Admin"),
            "A" => break (chudi, "Academic"),
            "L" => break (chudi, "Lawyer"),
            "T" => break (chudi, "Teacher"),
            _ => println!("Invalid service selected, try again"),
        }
    };

    println!("You selected: {}", name);

    if chudi == "OA" {
        titak[0]();
    }
    else if chudi == "A"{
        titak[1]();
    }
    else if chudi == "L"{
        titak[2]();
    }
    else if chudi == "T"{
        titak[3]()
    }
    else {
        println!("Invalid letter selected");
    }
}