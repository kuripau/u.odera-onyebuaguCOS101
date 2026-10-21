use std::io::Write;



fn main() {
   let headings = "Student Name,  Matric Number,  Department,  Level\n";
  let students = vec![
        vec!["Oluchi Mordi", "ACC10211111", "Accounting", "300"],
        vec!["Adams Aliyu", "ECO10110101", "Economics", "100"],
        vec!["Shania Bolade", "CSC10328828", "Computer", "200"],
        vec!["Adekunle Gold", "EEE11020202", "Electrical", "200"],
        vec!["Bianca Edemoh", "MEE10202001", "Mechanical", "100"],
    ];


   let mut file = std::fs::File::create("PAU-SIMS.csv").expect("Create failed");
   file.write_all("list of all students matric number department and level\n"
    .as_bytes()).expect("Write failed");
   file.write_all(headings.as_bytes()).expect("write failed");


//csv need a comma and . join joins the vectors 
for student in students {
    let mut line = student.join(",");
    line.push('\n');
    file.write_all(line.as_bytes()).expect("write failed");
}


   println!("Data created");

}
