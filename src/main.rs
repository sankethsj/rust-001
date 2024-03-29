use std::{fmt, ops::Add};

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
            let real_number: f32 = self.real;
            let mut sign: &str = "+";
            let mut imaginary: f32 = self.imag;
            if self.imag < 0.0 {
                sign = "-";
                imaginary = imaginary * (-1.0);
            }
            write!(
                f,
                "{real} {sign} {imag}i",
                real = real_number,
                imag = imaginary
            )
        }
    }

    let c = Complex {
        real: 3.1,
        imag: 4.2,
    };
    println!("{}", c);

    impl Add for Complex {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Complex {
                real: self.real + rhs.real,
                imag: self.imag + rhs.imag,
            }
        }
    }

    let d = Complex {
        real: -4.5,
        imag: -7.8,
    };
    println!("Complex d : {}", d);

    let c_plus_d = c + d;
    println!("Adding c and d complex gives : {}", c_plus_d);

    trait CustomTraitWithOptionalTrait {
        fn print_complex(&self);
        // this is an optional trait, we can either use in our implementation or not
        // to make this optional, we have to declare function body and return default value
        // notice in above trait 'print_complex' we don't have function body, so it is a required implmentation
        fn get_real(&self) -> f32 {
            0.0
        }
    }

    impl CustomTraitWithOptionalTrait for Complex {
        fn print_complex(&self) {
            println!(
                "Your complex number is :: Real = {real} and imaginary = {imag}",
                real = self.real,
                imag = self.imag
            )
        }
    }

    let e: Complex = Complex {
        real: 6.5,
        imag: -7.0,
    };
    e.print_complex();

    // we can call optional trait methods that are not implemented, it will give us default output 
    println!("Get real default output : {output}", output=e.get_real());

    // let's have an optional trait, but we implement it
    trait CustomTraitWithOptionalTraitButWeImplement {
        fn get_real_now(&self) -> f32 {
            0.0
        }
    }

    impl CustomTraitWithOptionalTraitButWeImplement for Complex {
        fn get_real_now(&self) -> f32 {
            self.real
        }
    }
    println!("Get real correct output : {output}", output=e.get_real_now());

}

fn main() {
    // today's date : 08-01-2024
    various_prints();
    structures();
}
