// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language.

pub fn run() {

    let name = "Jeff";
    let mut age = 37;
    println!("My name is {} and I am {}", name, age);
    age = 38; // I had a birthday.
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 007;
    println!("ID: {}", ID);

    //Assign multiple vars at once
    let (my_name, my_age ) = ("Jeff", 38);
    println!("{} is {}", my_name, my_age);
}