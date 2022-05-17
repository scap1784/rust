// Length is fixed.
// Same data types.

pub fn run() {
    let mut numbers: [i32;5] = [1,2,3,4,5];
    let numbers2 = [1,2,3,4,5];

    println!("{:?} with size {}", numbers, numbers.len());
    println!("{:?}", numbers2);
    println!("first element {}", numbers[0]);
    numbers[0] = 10;
    println!("first element {}", numbers[0]);

    println!("Array stack allocated and usage {} bytes", std::mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    let slice2 = &numbers[1..4];
    println!("Slice: {:?}", slice2);
}