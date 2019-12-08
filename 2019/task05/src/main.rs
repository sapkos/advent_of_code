use std::io;
use std::fs;

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

    pub fn output(x: i32) {
        println!("Output: {}", x)
    }
}



pub struct IntcodeComputer {
    memory: Vec<i32>,
    pointer: usize,
}

impl IntcodeComputer {
    fn get_opcode(value: i32) -> i32 {
        value % 100
    }

    fn read_value(&self, value: i32, parameter_mode: i32) -> i32 {
        match parameter_mode {
            0 => {
                self.memory[value as usize]
            },

            1 => {
                value
            },

            _ => {
                0
            }
        }
    }

    fn read_parameter_modes(value: i32, no_parameters: i32) -> Vec<i32> {
        let mut parameter_modes: Vec<i32> = Vec::new();
        let mut used_parameters = no_parameters;
        let mut value = value;
        while (value != 0) | (used_parameters > 0) {
            parameter_modes.push(value % 10);
            value /= 10;
            used_parameters -= 1; 
        }
        parameter_modes
    }

    fn read_arguments(&self, modes: i32, no_parameters: i32) -> Vec<i32> {
        let parameter_modes = IntcodeComputer::read_parameter_modes(modes, no_parameters);
        let mut arguments: Vec<i32> = Vec::new();
        let mut local_pointer = self.pointer;
        for parameter_mode in parameter_modes {
            local_pointer += 1;
            arguments.push(
                self.read_value(self.memory[local_pointer], parameter_mode)
            );
        }
        arguments
    }

    pub fn run(&mut self) -> Vec<i32> {
        self.pointer = 0;
        loop {
            let mut modes = self.memory[self.pointer];
            let opcode = IntcodeComputer::get_opcode(modes);
            modes /= 100; 
            let mut no_operation = 4;
            match opcode {
                1 => {
                    let arguments = self.read_arguments(modes, 2);
                    let target = self.memory[self.pointer + 3];
                    self.memory[target as usize] = Operator::add(arguments[0], arguments[1]);
                },

                2 => {
                    let arguments = self.read_arguments(modes, 2);
                    let target = self.memory[self.pointer + 3];
                    self.memory[target as usize] = Operator::multiply(arguments[0], arguments[1]);
                },

                3 => {
                    let target = self.memory[self.pointer + 1];
                    self.memory[target as usize] = Operator::input();
                    no_operation = 2;
                },

                4 => {
                    let arguments = self.read_arguments(modes, 1);
                    Operator::output(arguments[0]);
                    no_operation = 2;
                },

                5 => {
                    let arguments = self.read_arguments(modes, 2);
                    no_operation = 3;
                    if arguments[0] != 0 {
                        self.pointer = arguments[1] as usize;
                        no_operation = 0;
                    }
                }

                6 => {
                    let arguments = self.read_arguments(modes, 2);
                    no_operation = 3;
                    if arguments[0] == 0 {
                        self.pointer = arguments[1] as usize;
                        no_operation = 0;
                    }
                }

                7 => {
                    let arguments = self.read_arguments(modes, 2);
                    let target = self.memory[self.pointer + 3];
                    if arguments[0] < arguments[1] {
                        self.memory[target as usize] = 1;
                    } else {
                        self.memory[target as usize] = 0;
                    }
                    no_operation = 4;
                }

                8 => {
                    let arguments = self.read_arguments(modes, 2);
                    let target = self.memory[self.pointer + 3];
                    if arguments[0] == arguments[1] {
                        self.memory[target as usize] = 1;
                    } else {
                        self.memory[target as usize] = 0;
                    }
                    no_operation = 4;
                }

                99 => {
                    break;
                },

                _ => {}
            }
            self.pointer += no_operation;
        }
    self.memory.clone()
    }
}


fn main() {
    let input = fs::read_to_string("input.txt")
        .expect(":(");

    let memory: Vec<i32> = input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut computer = IntcodeComputer {
        memory: memory,
        pointer: 0
    };

    computer.run();
}
