use std::fs;

fn get_values(memory: &Vec<u32>, pointer: usize) -> (u32, u32) {
    let (p1, p2) = (memory[pointer + 1], memory[pointer + 2]);
    (memory[p1 as usize], memory[p2 as usize])
}

fn run_program(mut memory: Vec<u32>) -> Vec<u32> {
    let mut pointer: usize = 0;

    loop {
        let cur_code = memory[pointer];
        match cur_code {
            1 => {
                let (v1, v2) = get_values(&memory, pointer);
                let target = memory[(pointer + 3) as usize];
                memory[target as usize] = v1 + v2;
            },
            2 => {
                let (v1, v2) = get_values(&memory, pointer);
                let target = memory[(pointer + 3) as usize];
                memory[target as usize] = v1 * v2;
            },
            99 => {
                break;
            },
            _ => {  }
        }
        pointer += 4;
    }
    memory
}

fn search_solution(memory: Vec<u32>, target: u32) -> u32 {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut memory = memory.clone();
            memory[1] = noun;
            memory[2] = verb;
            let result = run_program(memory);
            if result[0] == target {
                return 100 * noun + verb;
            }
        }
    }

    0
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect(":(");

    let mut memory: Vec<u32> = input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    
    memory[1] = 12;
    memory[2] = 2;
    println!("{}", run_program(memory.clone())[0]);
    println!("{}", search_solution(memory, 19690720))
}
