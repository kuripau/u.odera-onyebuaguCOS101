use std::io;

fn trap_area(){
    let mut input_trap1 = String::new();
    let mut input_trap2 = String::new();
    let mut height_trap = String::new();


    println!("Enter in input:");
    io::stdin().read_line(&mut input_trap1).expect("Failed to read input");
    let trap_input1:f32 = input_trap1.trim().parse().expect("Not a valid number");

    println!("Enter in input:");
    io::stdin().read_line(&mut input_trap2).expect("Failed to read input");
    let trap_input2:f32 = input_trap2.trim().parse().expect("Not a valid number");

    println!("Enter in input");
    io::stdin().read_line(&mut height_trap).expect("Failed to read input");
    let heighttrap_input:f32 = height_trap.trim().parse().expect("Not a valid number");

    let trapa:f32 = heighttrap_input/2.0 * (trap_input1+trap_input2);
    println!("Area of the trapezuim is {}",trapa);
}

fn Rhomb_area(){
        let mut diagonal1 = String::new();
        let mut diagonal2 = String::new();

        println!("Enter in input for diagonal");
        io::stdin().read_line(&mut diagonal1).expect("Failed to read input");
        let rhom_input1:f32 = diagonal1.trim().parse().expect("Not a valid number");

        println!("Enter in input for other diagonal");
        io::stdin().read_line(&mut diagonal2).expect("Failed to read input");
        let rhom_input2:f32 = diagonal2.trim().parse().expect("not a valid number");

        let rhomba:f32 = 0.5*(rhom_input1*rhom_input2);
        println!("Area of the rhombus is {}",rhomba);

}

fn parra_area(){
    let mut base_parra = String::new();
    let mut altitude_parra = String::new();

    println!("Enter in input for base");
    io::stdin().read_line(&mut base_parra).expect("Failed to read input");
    let base_inputparra:f32 = base_parra.trim().parse().expect("Not a valid number");

    println!("Enter in input for the alltitude");
    io::stdin().read_line(&mut altitude_parra).expect("Failed to read input");
    let altitude_parra1:f32 = altitude_parra.trim().parse().expect("Not a valid number");

    let parraa:f32 = base_inputparra*altitude_parra1;
    println!("Area of the parralaelogram is {}",parraa);
}

fn cube_surfacearea(){
    let mut side_cube1 = String::new();
    let mut side_cube2 = String::new();

    println!("Enter the side for cube");
    io::stdin().read_line(&mut side_cube1).expect("Failed to read input");
    let side_cube1lol:f32 = side_cube1.trim().parse().expect("Not a valid number");

    println!("Enter the other cube side");
    io::stdin().read_line(&mut side_cube2).expect("Failed to read input");
    let side_cube2lol:f32 = side_cube2.trim().parse().expect("Not a valid number");

    let cubesurfacearea:f32 = 6.0*side_cube1lol*side_cube2lol;
    println!("surface area of the cube is {}",cubesurfacearea);


}
fn cylinder_volume(){
    let mut raduis_cylinder1 = String::new();
    let mut raduis_cylinder2 = String::new();
    let mut heoght_cylinder = String::new();

    println!("Enter the raduis of the cylinder");
    io::stdin().read_line(&mut raduis_cylinder1).expect("Failed to read input");
    let raduiscylinder1:f32 = raduis_cylinder1.trim().parse().expect("Not a valid number");

    println!("Enter the other raduis");
    io::stdin().read_line(&mut raduis_cylinder2).expect("Failed to read input");
    let raduiscylinder2:f32 = raduis_cylinder2.trim().parse().expect("Not a valid number");

    println!("Enter the height of the cylinder");
    io::stdin().read_line(&mut heoght_cylinder).expect("Failed to read input");
    let height_Cylinder:f32 = heoght_cylinder.trim().parse().expect("Not a valid number");

    let cylindervolume:f32 = 3.14 * raduiscylinder1 *raduiscylinder2*height_Cylinder;
    println!("the volume of the cylinder is {}", cylindervolume);
}

fn main() {
    println!("What calculations would you like to do");
    println!("\n Trapezuim-area");
        println!("Rhombus-area");
        println!("Parallelgram-area");
        println!("cube-surface area");
        println!("cylinder-volume");
        println!("Enter the letter corresponding to your shape choice (T, R, P, C, CY):");

let mut calculations= String::new();
        io::stdin().read_line(&mut calculations).expect("Failed to read input");
        let laterns= calculations.trim().to_uppercase();

let (letter_area,shapes) = match laterns.as_str() {
    "T" => ("Trapezuim-area",trap_area as fn()),
     "R" => ("Rhombus-area",Rhomb_area as fn()),
     "P" => ("Parallelgram-area",parra_area as fn()),
     "C" => ("cube-surface area",cube_surfacearea as fn()),
     "CY" => ("cylinder-volume",cylinder_volume as fn()),
    _ => {
                println!("Invalid area code selected!");
                return; 
            }
        };

println!("Your soultion is {}", letter_area);
shapes();

}


