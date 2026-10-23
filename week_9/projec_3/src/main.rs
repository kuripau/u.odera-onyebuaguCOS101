use std::io::Write;

fn main() {
  
  let headings = "Commisioner , Ministry , Zone \n";
  let commissioners = vec![
        "Aigbogun Alamba Daudu",     
        "Murtala Afeez Bendu",      
        "Okorocha Calistus Ogbona",  
        "Adewale Jimoh Akanbi",      
        "Osazuwa Faith Etieye",      
    ];
  let ministries = vec! [
     "Internal Affairs",         
        "Justice",                   
        "Defense",                   
        "Power & Steel",             
        "Petroleum",                 
  ];

let zones = vec! [
    "South West",               
        "North East",                
        "South South",               
        "South West",               
        "South East", 
                       
];

let mut file = std::fs::File::create("minister_file.csv").expect("Create failed");
file.write_all("List of ministers merged\n"
    .as_bytes()).expect("Write failed");
    file.write_all(headings.as_bytes()).expect("write failed");

    for i in 0..commissioners.len(){
    let record = format!(
    " {},  {}, {}\n",
    commissioners[i], ministries[i], zones[i]
);
  file.write_all(record.as_bytes());
}
    println!("File succesfully created");
}

