use std::io;

pub struct Operator {}

impl Operator {
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    pub fn multiply(x: i32, y: i32) -> i32 {
        x * y
    }

    pub fn input() -> i32 {
        println!("Provide input: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect(":(");

        input.trim().parse().expect("invalid input")
    }

    pub fn output(x: i32) -> i32 {
        x
    }
}
