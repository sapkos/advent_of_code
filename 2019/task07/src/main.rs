use std::fs;
use permutohedron::LexicalPermutation;

mod intcode;

use intcode::IntcodeComputer;

fn generate_permutations(mut list: Vec<i32>) -> Vec<Vec<i32>> {
    let mut permutations = Vec::new();
    loop {
        permutations.push(list.to_vec());
        if !list.next_permutation() {
            break;
        }
    }
    permutations
}

fn part_one(input: Vec<i32>) -> i32 {
    let amplifiers: Vec<i32> = vec![0, 1, 2, 3, 4];

    let mut max_thrust = 0;
    for permutation in generate_permutations(amplifiers) {
        let mut output = 0;
        for p in permutation {
            let inputs: Vec<i32> = vec![output, p];
            let mut computer = IntcodeComputer::new(
                input.clone(), 0,
            );
            output = computer.run();
        }
        if output > max_thrust {
            max_thrust = output
        }
    }
    max_thrust
}

fn part_two(input: Vec<i32>) -> i32 {
    let amplifiers: Vec<i32> = vec![5, 6, 7, 8, 9];
    let mut max_thrust = 0;
    for permutation in generate_permutations(amplifiers) {
        let mut computers: Vec<IntcodeComputer> = Vec::new();
        for _ in 0..5 {
            computers.push(IntcodeComputer::new(input.clone(), 0))
        }
        // first I set phase to each computer
        println!("setting phase");
        for i in 0..5 {
            computers[i].set_input(permutation[i]);
            computers[i].run();
        }
        let mut output_signal = 0;
        println!("main loop starts");
        loop {
            for i in 0..5 {
                println!("computer {} is working", i);
                computers[i].set_input(output_signal);
                output_signal = computers[i].run();
            }
            if computers[4].halt {
                break;
            }
        }
        if output_signal > max_thrust {
            max_thrust = output_signal
        }
    }
    max_thrust
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect(":(");

    let memory: Vec<i32> = input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    // println!("Part One: {}", part_one(memory.clone()));
    println!("Part Two: {}", part_two(memory.clone()))
}
