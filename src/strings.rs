// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {

    let mut hello = String::from("Hello");
    println!("{} is {} in length", hello, hello.len());
    hello.push(',');
    println!("{} is {} in length", hello, hello.len());
    hello.push_str(" World!");
    println!("{} is {} in length", hello, hello.len());
    println!("Capacity in bytes: {}", hello.capacity());
    println!("Is Empty: {}", hello.is_empty());
    println!("Contain World? {}", hello.contains("World"));
    println!("Replace: {}", hello.replace("World", "Jeff"));
    println!("{} is {} in length", hello, hello.len());
    let new = hello.replace("World", "John");
    println!("{} is {} in length", new, new.len());

    // Assertion testing
    assert_eq!(12, new.len())

}