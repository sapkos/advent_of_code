mod operator;
use operator::Operator;

pub struct IntcodeComputer {
    pub memory: Vec<i32>,
    pub pointer: usize,
    pub halt: bool,
    input: i32,
    output: i32,
    has_input: bool,
    pub iteration: i32
}

impl IntcodeComputer {

    pub fn new(memory: Vec<i32>, pointer: usize) -> IntcodeComputer {
        IntcodeComputer {
            memory: memory,
            pointer: pointer,
            output: -1,
            input: -1,
            iteration: 0,
            has_input: false,
            halt: false
        }
    }
    
    pub fn set_input(&mut self, input: i32) {
        self.input = input;
        self.has_input = true
    }


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

    pub fn run(&mut self) -> i32 {
        // self.pointer = 0;
        loop {
            self.iteration += 1;
            let mut modes = self.memory[self.pointer];
            let opcode = IntcodeComputer::get_opcode(modes);
            modes /= 100; 
            let mut no_operation = 4;
            println!("opcode {}", opcode);
            match opcode {
                1 => {
                    let arguments = self.read_arguments(modes, 2);
                    let target = self.memory[self.pointer + 3];
                    self.memory[target as usize] = Operator::add(arguments[0], arguments[1]);
                    println!("adding {}, {} and writing to {}", arguments[0], arguments[1], target)
                },

                2 => {
                    let arguments = self.read_arguments(modes, 2);
                    let target = self.memory[self.pointer + 3];
                    self.memory[target as usize] = Operator::multiply(arguments[0], arguments[1]);
                    println!("multiplying {}, {} and writing to {}", arguments[0], arguments[1], target)
                },

                3 => {
                    let target = self.memory[self.pointer + 1];
                    if self.has_input {
                        self.memory[target as usize] = self.input;
                        println!("writing {} to {}", self.input, target);
                        self.has_input = false;
                        no_operation = 2;
                    } else {
                        println!("set input first");
                        break;
                    }
                },

                4 => {
                    let arguments = self.read_arguments(modes, 1);
                    println!("output: {}", arguments[0]);
                    self.output = Operator::output(arguments[0]);
                    no_operation = 2;
                },

                5 => {
                    let arguments = self.read_arguments(modes, 2);
                    no_operation = 3;
                    println!("comparing {} to 0", arguments[0]);
                    if arguments[0] != 0 {
                        self.pointer = arguments[1] as usize;
                        println!("moved pointer to {}", arguments[1]);
                        no_operation = 0;
                    }
                }

                6 => {
                    let arguments = self.read_arguments(modes, 2);
                    no_operation = 3;
                    println!("comparing {} to 0", arguments[0]);
                    if arguments[0] == 0 {
                        self.pointer = arguments[1] as usize;
                        println!("moved pointer to {}", arguments[1]);
                        no_operation = 0;
                    }
                }

                7 => {
                    let arguments = self.read_arguments(modes, 2);
                    let target = self.memory[self.pointer + 3];
                    println!("comparing {} to {}", arguments[0], arguments[1]);
                    if arguments[0] < arguments[1] {
                        println!("set memory at {} to 1", target);
                        self.memory[target as usize] = 1;
                    } else {
                        println!("set memory at {} to 0", target);
                        self.memory[target as usize] = 0;
                    }
                    no_operation = 4;
                }

                8 => {
                    let arguments = self.read_arguments(modes, 2);
                    let target = self.memory[self.pointer + 3];
                    println!("comparing {} to {}", arguments[0], arguments[1]);
                    if arguments[0] == arguments[1] {
                        println!("set memory at {} to 1", target);
                        self.memory[target as usize] = 1;
                    } else {
                        println!("set memory at {} to 0", target);
                        self.memory[target as usize] = 0;
                    }
                    no_operation = 4;
                }

                99 => {
                    println!("HALT");
                    self.halt = true;
                    break;
                },

                _ => {}
            }
            self.pointer += no_operation;
        }
    self.output
    }
}


