pub fn various_prints() {
    println!("Hello, World!");

    // placeholder arguments
    println!("Hello, {}! Your age is {}.", "Enku", 25);

    // positional arguments
    // whatever values passed will be stringified
    println!("Hello, {0}! {0}'s age is {1}.", "Enku", 25);

    // named arguments
    println!(
        "Hello, {username}! You last logged in {days} days ago.",
        username = "Enku",
        days = 3
    );

    // Different formatting can be invoked by specifying the format character
    // after a `:`. works only on numbers/ints of-course
    println!("Base 10:               {}", 2540);
    println!("Base 2 (binary):       {:b}", 2540);
    println!("Base 8 (octal):        {:o}", 2540);
    println!("Base 16 (hexadecimal): {:x}", 2540);
    println!("Base 16 (hexadecimal): {:X}", 2540);

    // You can pad numbers with extra zeroes,
    println!("{number} Million = {number:0<7}", number = 1);
    println!("{number} Billion = {number:0<10}", number = 1);
}
