use std::fs;
use permutohedron::LexicalPermutation;

mod intcode;

use intcode::IntcodeComputer;

fn generate_permutations(mut list: Vec<i128>) -> Vec<Vec<i128>> {
    let mut permutations = Vec::new();
    loop {
        permutations.push(list.to_vec());
        if !list.next_permutation() {
            break;
        }
    }
    permutations
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect(":(");

    let memory: Vec<i128> = input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut computer = IntcodeComputer::new(memory.clone(), 0);
    computer.set_input(1);
    computer.run();
    let mut computer = IntcodeComputer::new(memory.clone(), 0);
    computer.set_input(2);
    computer.run();

}
