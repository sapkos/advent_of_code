use std::fs;

fn total_mass(size: i32) -> i32 {
    let mut needed_fuel = size / 3 - 2;
    let mut fuel_sum = 0;
    while needed_fuel >= 0 {
        fuel_sum += needed_fuel;
        needed_fuel = needed_fuel / 3 - 2;
    }
    fuel_sum
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect(":(");

    let sizes = input.lines()
        .map(|x| x.parse::<i32>().unwrap());

    let total_size: i32 = sizes
        .clone()
        .map(|x| x / 3 - 2)
        .sum();

    let real_sizes: i32 = sizes
        .clone()
        .map(total_mass)
        .sum();
    println!("{}", total_size);
    println!("{}", real_sizes);
}
