fn main() {
     //initilization of tuple with data type
     let datatype_tuple: (&str, f32 , u8) = ("RUst", 3.14,100);
     println!("Tuple contents = {:?}", datatype_tuple);

     //initiliztion of tuple without data type 
     let no_datatype_tuple = ("Rust","fun",100);
     println!("Tuple contents = {:?}", no_datatype_tuple);

     //acessing tuple elements at index 0 
     println!("Value at Index 0 = {}", datatype_tuple.0);

     //accesing tuple elemnts at index 1
     println!("Value at Index 1 = {}", datatype_tuple.1);

     //accesing tuple elemts at index 2 
     println!("Value at Index 2 = {}", datatype_tuple.2);
}   
