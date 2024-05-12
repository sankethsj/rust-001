use std::{fmt, ops::Add};

pub fn structures() {
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
    println!("Get real default output : {output}", output = e.get_real());

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
    println!(
        "Get real correct output : {output}",
        output = e.get_real_now()
    );
}
