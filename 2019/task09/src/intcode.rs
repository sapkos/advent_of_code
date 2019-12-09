mod operator;
use operator::Operator;

pub struct IntcodeComputer {
    pub memory: Vec<i128>,
    pub pointer: usize,
    pub halt: bool,
    pub iteration: i128,
    input: i128,
    output: i128,
    relative_base: i128,
    has_input: bool
}

impl IntcodeComputer {

    pub fn new(memory: Vec<i128>, pointer: usize) -> IntcodeComputer {
        IntcodeComputer {
            memory: memory,
            pointer: pointer,
            output: -1,
            input: -1,
            relative_base: 0,
            iteration: 0,
            has_input: false,
            halt: false
        }
    }
    
    pub fn set_input(&mut self, input: i128) {
        self.input = input;
        self.has_input = true
    }


    fn get_opcode(value: i128) -> i128 {
        value % 100
    }

    fn read(&mut self, pointer: usize) -> i128 {
        let cur_memory_len = self.memory.len();
        if pointer >= cur_memory_len {
            self.memory.resize(pointer + 1, 0)
        }
        self.memory[pointer]
    }

    fn write(&mut self, pointer: usize, value: i128) {
        let cur_memory_len = self.memory.len();
        if pointer >= cur_memory_len {
            self.memory.resize(pointer + 1, 0)
        }
        self.memory[pointer] = value;

    }

    fn read_value(&mut self, value: i128, parameter_mode: i128) -> i128 {
        match parameter_mode {
            0 => {
                self.read(value as usize)
            },

            1 => {
                value
            },

            2 => {
                self.read((self.relative_base + value) as usize)
            }

            _ => {
                0
            }
        }
    }

    fn read_parameter_modes(value: i128, no_parameters: i128) -> Vec<i128> {
        let mut parameter_modes: Vec<i128> = Vec::new();
        let mut used_parameters = no_parameters;
        let mut value = value;
        while (value != 0) | (used_parameters > 0) {
            parameter_modes.push(value % 10);
            value /= 10;
            used_parameters -= 1; 
        }
        parameter_modes
    }

    fn read_arguments(&mut self, modes: i128, no_parameters: i128) -> Vec<i128> {
        let parameter_modes = IntcodeComputer::read_parameter_modes(modes, no_parameters);
        let mut arguments: Vec<i128> = Vec::new();
        let mut local_pointer = self.pointer;
        for parameter_mode in parameter_modes {
            local_pointer += 1;
            arguments.push(
                self.read_value(self.memory[local_pointer], parameter_mode)
            );
        }
        arguments
    }

    fn read_write_mod(&mut self, modes: i128, no_parameters: i128) -> i128 {
        IntcodeComputer::read_parameter_modes(modes, no_parameters).pop().unwrap()
    }

    fn read_target(&mut self, value: i128, modes: i128, no_parameters: i128) -> i128 {
        let write_mod = self.read_write_mod(modes, no_parameters);
        match write_mod {
            0 => {
                value
            }
            1 => {
                value
            }
            2 => {
                self.relative_base + value
            }
            _ => {
                1
            }
        }
    }

    pub fn run(&mut self) -> i128 {
        // self.pointer = 0;
        loop {
            self.iteration += 1;
            let mut modes = self.read(self.pointer);
            println!("full opcode {}", modes);
            let opcode = IntcodeComputer::get_opcode(modes);
            modes /= 100;
            let mut no_operation = 4;
            println!("opcode {}", opcode);
            match opcode {
                1 => {
                    let arguments = self.read_arguments(modes, 2);
                    let value = self.memory[self.pointer + 3];
                    let target = self.read_target(value, modes, 3);
                    self.write(target as usize, Operator::add(arguments[0], arguments[1]));
                    println!("adding {}, {} and writing to {}", arguments[0], arguments[1], target)
                },

                2 => {
                    let arguments = self.read_arguments(modes, 2);
                    let value = self.memory[self.pointer + 3];
                    let target = self.read_target(value, modes, 3);
                    println!("multiplying {}, {} and writing to {}", arguments[0], arguments[1], target);
                    self.write(target as usize, Operator::multiply(arguments[0], arguments[1]));
                },

                3 => {
                    println!("modes: {}", modes);
                    let mut target = self.read(self.pointer + 1);
                    if modes == 2 {
                        target = self.read(self.pointer + 1) + self.relative_base;
                    }
                    if self.has_input {
                        self.write(target as usize, self.input);
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
                    let value = self.memory[self.pointer + 3];
                    let target = self.read_target(value, modes, 3);
                    println!("comparing {} to {}", arguments[0], arguments[1]);
                    if arguments[0] < arguments[1] {
                        println!("set memory at {} to 1", target);
                        self.write(target as usize, 1);
                    } else {
                        println!("set memory at {} to 0", target);
                        self.write(target as usize, 0);
                    }
                    no_operation = 4;
                }

                8 => {
                    let arguments = self.read_arguments(modes, 2);
                    let value = self.memory[self.pointer + 3];
                    let target = self.read_target(value, modes, 3);
                    println!("comparing {} to {}", arguments[0], arguments[1]);
                    if arguments[0] == arguments[1] {
                        println!("set memory at {} to 1", target);
                        self.write(target as usize, 1);
                    } else {
                        println!("set memory at {} to 0", target);
                        self.write(target as usize, 0);
                    }
                    no_operation = 4;
                }

                9 => {
                    let arguments = self.read_arguments(modes, 1);
                    println!("change rel base {}", arguments[0]);
                    self.relative_base += arguments[0];
                    no_operation = 2;
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
