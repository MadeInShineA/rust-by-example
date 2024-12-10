fn main() {
    println!(
        "This is the first argument {} this is the second one {}",
        1, 2
    );

    println!(
        "This is the second argument {1} this is the first one {0}",
        1, 2
    );

    println!(
        "This is the cool argument {cool} this is the bad one {bad}",
        cool = "yay",
        bad = "nooo"
    );

    // This doesn't work
    // println!("{number:padding_char$>number_to_pad_to$}", number=1, padding_char='a', number_to_pad_to = 10);

    println!("Binary {:b}", 1234);

    let name: &str = "MadeIn";
    let surname: &str = "ShineA";
    println!("{name} {surname}");

    // Activities

    println!("My name is {0}, {1} {0}", "Bond", "James");

    const PI: f64 = 3.141592;
    println!("Pi is roughly {PI:.3}");
}
