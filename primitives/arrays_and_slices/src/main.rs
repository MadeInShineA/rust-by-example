use std::mem;

fn display_array<T: std::fmt::Display>(array: &[T]) {
    for (index, element) in array.iter().enumerate() {
        println!("Index {index}: {element}");
    }
}

fn main() {
    let array: [u32; 5] = [1, 2, 3, 4, 5];

    println!(
        "The size of the array in memory is {} bytes",
        mem::size_of_val(&array)
    );

    display_array(&array);
}
