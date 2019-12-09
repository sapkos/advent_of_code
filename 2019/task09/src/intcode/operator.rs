use std::io;

pub struct Operator {}

impl Operator {
    pub fn add(x: i128, y: i128) -> i128 {
        x + y
    }

    pub fn multiply(x: i128, y: i128) -> i128 {
        x * y
    }

    pub fn input() -> i128 {
        println!("Provide input: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect(":(");

        input.trim().parse().expect("invalid input")
    }

    pub fn output(x: i128) -> i128 {
        x
    }
}
