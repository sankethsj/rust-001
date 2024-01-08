use std::fmt::{self};

fn various_prints() {
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

fn structures() {
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let mut p = Person {
        name: "Enku",
        age: 25,
    };
    println!(
        "Person name is '{name}' and age is '{age}'.",
        name = p.name,
        age = p.age
    );

    impl fmt::Display for Person<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Person : {{ 'name': {name}, 'age': {age} }}",
                name = self.name,
                age = self.age
            )
        }
    }
    p.name = "Enku Manku";
    p.age = 27;
    println!("{}", p);

    // implementing Complex structure 3.3 + 5.1i
    struct Complex {
        real: f32,
        imag: f32,
    }

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Complex : {{ 'real': {real}, 'imag': {imag} }}",
                real = self.real,
                imag = self.imag
            )
        }
    }

    let c = Complex {
        real: 3.1,
        imag: 4.2,
    };
    println!("{}", c);
}

fn main() {
    // today's date : 08-01-2024
    various_prints();
    structures();
}
