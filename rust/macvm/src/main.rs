#![allow(unused)]

#[derive(Debug, Clone, PartialEq)]
enum Opcode {
    PSH(isize),
    ADD,
    POP,
    SET,
    HLT
}

#[derive(Debug, Clone)]
struct Vm {
    input: Vec<Opcode>,
    ip: usize
}

impl Vm {
    fn new(input: Vec<Opcode>) -> Self {
        Self {
            input,
            ip: 0
        }
    }

    fn fetch(&self) -> Opcode {
        // self.input.nth(self.ip).expect("Cannot peek: `ip` is invalid.")
        if let Some(opcode) = self.input.get(self.ip) {
            opcode.clone()
        } else {
            panic!("Cannot fetch next Opcode: `ip` is invalid.")
        }
    }

    fn eval(&mut self) {
        for opcode in self.clone().input {
            while self.fetch() != Opcode::HLT {
                let next_opcode = self.fetch();
                println!("{:?}", next_opcode);
                self.ip += 1;
            }
        }
    }
}

fn main() {
    type OP = Opcode;

    // let instr = vm.input[vm.ip];
    let program = vec![OP::PSH(5), OP::PSH(6), OP::ADD, OP::POP, OP::HLT];
    let mut vm = Vm::new(program);
    vm.eval();
}
