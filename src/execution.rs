use instruction_set::{InstructionArgument, Register};

use cpu::CPU;

pub enum ArgumentSize {
    Bit64,
    Bit32,
    Bit16,
    Bit8,
}


impl CPU {
    pub fn first_argument_size(&self, arg: &InstructionArgument) -> ArgumentSize {
        match *arg {
            InstructionArgument::OneRegister { ref register, .. } => get_register_size(register),
            InstructionArgument::TwoRegister { ref register1,
                                               ref register2,
                                               reverse_direction,
                                               .. } => {
                if reverse_direction {
                    get_register_size(register2)
                } else {
                    get_register_size(register1)
                }
            }
            InstructionArgument::Immediate8 { .. } => ArgumentSize::Bit8,
            InstructionArgument::Immediate32 { .. } => ArgumentSize::Bit32,
            InstructionArgument::Immediate8BitRegister { .. } => ArgumentSize::Bit8,
            InstructionArgument::Immediate32BitRegister { .. } => ArgumentSize::Bit32,
        }
    }

    pub fn second_argument_size(&self, arg: &InstructionArgument) -> ArgumentSize {
        match *arg {
            InstructionArgument::OneRegister { .. } => panic!("Only one argument available"),
            InstructionArgument::TwoRegister { ref register1,
                                               ref register2,
                                               reverse_direction,
                                               .. } => {
                if reverse_direction {
                    get_register_size(register1)
                } else {
                    get_register_size(register2)
                }
            }
            InstructionArgument::Immediate8 { .. } => panic!("Only one argument available"),
            InstructionArgument::Immediate32 { .. } => panic!("Only one argument available"),
            InstructionArgument::Immediate8BitRegister { ref register, .. } => {
                get_register_size(register)
            }
            InstructionArgument::Immediate32BitRegister { ref register, .. } => {
                get_register_size(register)
            }
        }
    }

    pub fn first_argument_i8(&self, arg: InstructionArgument) -> i8 {
        panic!("Not implemented");
    }

    pub fn first_argument_i16(&self, arg: InstructionArgument) -> i16 {
        panic!("Not implemented");
    }

    pub fn first_argument_i32(&self, arg: InstructionArgument) -> i32 {
        match arg {
            InstructionArgument::OneRegister { register, .. } => {
                self.get_register_value_i32(register)
            },
            InstructionArgument::TwoRegister { register1, .. } => {
                self.get_register_value_i32(register1)
            },
            InstructionArgument::Immediate8 { immediate, .. } => immediate as i32,
            InstructionArgument::Immediate32 { immediate, .. } => immediate as i32,
            InstructionArgument::Immediate8BitRegister { immediate, .. } => immediate as i32,
            InstructionArgument::Immediate32BitRegister  { immediate, .. } => immediate as i32,
        }
    }

    pub fn first_argument_i64(&self, arg: InstructionArgument) -> i64 {
        match arg {
            InstructionArgument::OneRegister { register, .. } => {
                self.get_register_value_i64(register)
            },
            InstructionArgument::TwoRegister { register1, .. } => {
                self.get_register_value_i64(register1)
            },
            InstructionArgument::Immediate8 { immediate, .. } => immediate as i64,
            InstructionArgument::Immediate32 { immediate, .. } => immediate as i64,
            InstructionArgument::Immediate8BitRegister { immediate, .. } => immediate as i64,
            InstructionArgument::Immediate32BitRegister  { immediate, .. } => immediate as i64,
        }
    }


    pub fn second_argument_i8(&self, arg: InstructionArgument) -> i8 {
        panic!("Not implemented");
    }

    pub fn second_argument_i16(&self, arg: InstructionArgument) -> i16 {
        panic!("Not implemented");
    }

    pub fn second_argument_i32(&self, arg: InstructionArgument) -> i32 {
        panic!("Not implemented");
    }

    pub fn second_argument_i64(&self, arg: InstructionArgument) -> i64 {
        panic!("Not implemented");
    }


    // register operations
    fn get_register_value_i32(&self, register: Register) -> i32 {
        match register {
            Register::RAX => panic!("Cannot get 32bit value from 64bit register"),
            Register::RBX => panic!("Cannot get 32bit value from 64bit register"),
            Register::RCX => panic!("Cannot get 32bit value from 64bit register"),
            Register::RDX => panic!("Cannot get 32bit value from 64bit register"),
            Register::RSP => panic!("Cannot get 32bit value from 64bit register"),
            Register::RBP => panic!("Cannot get 32bit value from 64bit register"),
            Register::RSI => panic!("Cannot get 32bit value from 64bit register"),
            Register::RDI => panic!("Cannot get 32bit value from 64bit register"),

            Register::RIP => self.instruction_pointer as i32,

            Register::EAX => self.rax as i32,
            Register::EBX => self.rbx as i32,
            Register::ECX => self.rcx as i32,
            Register::EDX => self.rdx as i32,
            Register::ESP => self.rsp as i32,
            Register::EBP => self.rbp as i32,
            Register::ESI => self.rsi as i32,
            Register::EDI => self.rdi as i32,

            Register::ES => 0,
            Register::CS => 0,
            Register::SS => 0,
            Register::DS => 0,
            Register::FS => 0,
            Register::GS => 0,
        }
    }

    fn get_register_value_i64(&self, register: Register) -> i64 {
        match register {
            Register::RAX => self.rax,
            Register::RBX => self.rbx,
            Register::RCX => self.rcx,
            Register::RDX => self.rdx,
            Register::RSP => self.rsp,
            Register::RBP => self.rbp,
            Register::RSI => self.rsi,
            Register::RDI => self.rdi,

            Register::RIP => self.instruction_pointer as i64,

            Register::EAX => self.rax as i32 as i64,
            Register::EBX => self.rbx as i32 as i64,
            Register::ECX => self.rcx as i32 as i64,
            Register::EDX => self.rdx as i32 as i64,
            Register::ESP => self.rsp as i32 as i64,
            Register::EBP => self.rbp as i32 as i64,
            Register::ESI => self.rsi as i32 as i64,
            Register::EDI => self.rdi as i32 as i64,

            Register::ES => 0,
            Register::CS => 0,
            Register::SS => 0,
            Register::DS => 0,
            Register::FS => 0,
            Register::GS => 0,
        }
    }

    // stack operations
    pub fn stack_push(&mut self, data: Vec<u8>) {
        for v in data {
            self.rsp -= 1;
            self.stack[self.rsp as usize] = v;
        }
    }
}

pub fn convert_i8_to_u8vec(value: i8) -> Vec<u8> {
    vec![value as u8]
}

pub fn convert_i16_to_u8vec(value: i16) -> Vec<u8> {
    vec![
        (value & 0x00FF) as u8,
        (value & 0xFF00 >> 8) as u8,
    ]
}

pub fn convert_i32_to_u8vec(value: i32) -> Vec<u8> {
    vec![
        (value & 0x000000FF) as u8,
        (value & 0x0000FF00 >> 8) as u8,
        (value & 0x00FF0000 >> 16) as u8,
        (value & 0xFF000000 >> 24) as u8,
    ]
}

pub fn convert_i64_to_u8vec(value: i64) -> Vec<u8> {
    vec![
        (value & 0x00000000000000FF) as u8,
        (value & 0x000000000000FF00 >> 8) as u8,
        (value & 0x0000000000FF0000 >> 16) as u8,
        (value & 0x00000000FF000000 >> 24) as u8,

        (value & 0x000000FF00000000 >> 32) as u8,
        (value & 0x0000FF0000000000 >> 40) as u8,
        (value & 0x00FF000000000000 >> 48) as u8,
        (value & 0xFF00000000000000 >> 56) as u8,
    ]
}


fn get_register_size(reg: &Register) -> ArgumentSize {
    match *reg {
        Register::RAX | Register::RBX | Register::RCX | Register::RDX | Register::RSP |
        Register::RBP | Register::RSI | Register::RDI | Register::RIP => ArgumentSize::Bit64,
        Register::EAX | Register::EBX | Register::ECX | Register::EDX | Register::ESP |
        Register::EBP | Register::ESI | Register::EDI => ArgumentSize::Bit32,
        Register::ES | Register::CS | Register::SS | Register::DS | Register::FS | Register::GS => {
            ArgumentSize::Bit16
        }
    }
}
