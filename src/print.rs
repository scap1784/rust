
pub fn run() {
    //print to console
    println!("Hello, World!");

    //placeholders
    println!("Number: {}", 21);

    //multiple placeholders
    println!("{} is from {}", "Jeff", "Texas");

    //Positional Arguments
    println!("{} is from {} and {0} likes to {}", "Jeff", "Texas", "code");

    //Named Arguments
    println!("{name} likes to play {activity}", name="Jeff", activity="EUC");

    // Placeholder Traits
    println!("Binary: {:b}, Hex {:x}, Octal {:o}", 10, 10, 10);

    // Debug trait
    println!("Debug: {:?}", (12, true, "HI"));

    //Basic Math
    println!("10 + 10 = {}", 10 + 10);
}