use chunk::{Chunk, OpCode};
use disassembler::disassemble_instruction;
use value::{self, Value};

const STACK_MAX: usize = 256;

pub struct VM {
    debug: DebugFlags,
}

impl VM {
    pub fn new() -> Self {
        VM {
            debug: DebugFlags::new(),
        }
    }

    pub fn new_debugger(debug: DebugFlags) -> Self {
        VM { debug }
    }

    pub fn interpret(&self, chunk: Chunk) -> VMResult {
        let ip = 0;
        self.run(chunk, ip)
    }

    fn run(&self, chunk: Chunk, ip: usize) -> VMResult {
        let mut stack: Stack = Stack::new();
        let mut ip = ip;
        loop {
            if self.debug.print_stack {
                print!(
                    "          {}\n",
                    stack
                        .clone()
                        .into_iter()
                        .fold(String::new(), |collector, value| collector
                            + "[ "
                            + &value::string_from(value)
                            + " ]")
                );
            }

            if self.debug.print_instructions {
                let (_, command) = disassemble_instruction(&chunk, ip, String::new());
                print!("{}", command);
            }

            let opcode = OpCode::from(match chunk.get_byte(ip) {
                Some(i) => i,
                None => return VMResult::RuntimeError,
            });

            ip = ip + 1;
            ip = match opcode {
                OpCode::Return => {
                    /*if self.debug {
                        println!(
                            "{}",
                            value::string_from(match stack.pop() {
                                Some(i) => i,
                                None => return VMResult::CompileError,
                            })
                        );
                    }*/

                    return VMResult::Okay(match stack.pop() {
                        Some(i) => i,
                        None => 0f64,
                    });
                }
                OpCode::Constant => {
                    let constant = match chunk.get_byte(ip) {
                        Some(i) => i,
                        None => return VMResult::RuntimeError,
                    };
                    let constant = match chunk.get_constant(constant as usize) {
                        Some(i) => i,
                        None => return VMResult::RuntimeError,
                    };

                    if let Err(_) = stack.push(constant) {
                        return VMResult::RuntimeError;
                    }

                    if self.debug.print_constants {
                        println!("{}", value::string_from(constant));
                    }

                    ip + 1
                }
                OpCode::ConstantLong => {
                    let first_byte = match chunk.get_byte(ip) {
                        Some(i) => i,
                        None => return VMResult::RuntimeError,
                    };
                    let second_byte = match chunk.get_byte(ip + 1) {
                        Some(i) => i,
                        None => return VMResult::RuntimeError,
                    };
                    let constant =
                        match chunk.get_long_constant(first_byte as usize, second_byte as usize) {
                            Some(i) => i,
                            None => return VMResult::RuntimeError,
                        };

                    if let Err(_) = stack.push(constant) {
                        return VMResult::RuntimeError;
                    }

                    if self.debug.print_constants {
                        println!("{}", value::string_from(constant));
                    }

                    ip + 2
                }
                OpCode::Negate => {
                    let val = match stack.pop() {
                        Some(i) => i,
                        None => return VMResult::RuntimeError,
                    };
                    if let Err(_) = stack.push(-val) {
                        return VMResult::RuntimeError;
                    }
                    ip
                }
                OpCode::Add => {
                    stack = match VM::binary_op(stack, |a, b| a + b) {
                        Ok(stack) => stack,
                        Err(result) => return result,
                    };
                    ip
                }
                OpCode::Subtract => {
                    stack = match VM::binary_op(stack, |a, b| a - b) {
                        Ok(stack) => stack,
                        Err(result) => return result,
                    };
                    ip
                }
                OpCode::Multiply => {
                    stack = match VM::binary_op(stack, |a, b| a * b) {
                        Ok(stack) => stack,
                        Err(result) => return result,
                    };
                    ip
                }
                OpCode::Divide => {
                    stack = match VM::binary_op(stack, |a, b| a / b) {
                        Ok(stack) => stack,
                        Err(result) => return result,
                    };
                    ip
                }
                OpCode::UnexpectedEndOfChunk => return VMResult::CompileError,
                OpCode::Invalid(_) => return VMResult::CompileError,
            }
        }
    }

    fn binary_op<F>(mut stack: Stack, op: F) -> Result<Stack, VMResult>
    where
        F: Fn(Value, Value) -> Value,
    {
        let b = match stack.pop() {
            Some(i) => i,
            None => return Err(VMResult::RuntimeError),
        };
        let a = match stack.pop() {
            Some(i) => i,
            None => return Err(VMResult::RuntimeError),
        };
        if let Err(_) = stack.push(op(a, b)) {
            Err(VMResult::RuntimeError)
        } else {
            Ok(stack)
        }
    }
}

#[derive(Clone, Debug)]
struct Stack(Vec<Value>);

impl Stack {
    fn new() -> Self {
        Stack(Vec::new())
    }

    fn push(&mut self, val: Value) -> Result<(), ()> {
        if self.0.len() >= STACK_MAX {
            Err(())
        } else {
            self.0.push(val);
            Ok(())
        }
    }

    fn pop(&mut self) -> Option<Value> {
        self.0.pop()
    }
}

impl IntoIterator for Stack {
    type Item = Value;
    type IntoIter = ::std::vec::IntoIter<Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.clone().into_iter()
    }
}

pub struct DebugFlags {
    print_instructions: bool,
    print_stack: bool,
    print_constants: bool,
}

impl DebugFlags {
    pub fn new() -> Self {
        DebugFlags {
            print_instructions: false,
            print_stack: false,
            print_constants: false,
        }
    }

    pub fn set_flag(self, flag: DebugFlag, value: bool) -> Self {
        match flag {
            DebugFlag::PrintInstructions => DebugFlags {
                print_instructions: value,
                ..self
            },
            DebugFlag::PrintStack => DebugFlags {
                print_stack: value,
                ..self
            },
            DebugFlag::PrintConstants => DebugFlags {
                print_constants: value,
                ..self
            },
        }
    }
}

pub enum DebugFlag {
    PrintInstructions,
    PrintStack,
    PrintConstants,
}

pub enum VMResult {
    Okay(Value),
    CompileError,
    RuntimeError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_return() {
        let chunk = Chunk::new()
            .write_constant(1.0, 1)
            .write_chunk(OpCode::Return, 1);
        return_equals(1.0, chunk);
    }

    #[test]
    fn test_math() {
        test_math_op(OpCode::Add, 1.0, 1.0, 2.0); // 1 + 1 = 2
        test_math_op(OpCode::Subtract, 2.0, 1.0, 1.0); // 2 - 1 = 1
        test_math_op(OpCode::Multiply, 1.0, 2.0, 2.0); // 1 * 2 = 2
        test_math_op(OpCode::Divide, 2.0, 2.0, 1.0); // 2 / 2 = 1
    }

    fn test_math_op(op: OpCode, operand_a: Value, operand_b: Value, result: Value) {
        let chunk = build_binary_op_chunk(operand_a, operand_b, op);
        return_equals(result, chunk);
    }

    fn build_binary_op_chunk(a: Value, b: Value, op: OpCode) -> Chunk {
        Chunk::new()
            .write_constant(a, 1)
            .write_constant(b, 1)
            .write_chunk(op, 1)
            .write_chunk(OpCode::Return, 1)
    }

    fn return_equals(val: Value, chunk: Chunk) {
        if let VMResult::Okay(i) = VM::new().interpret(chunk) {
            assert_eq!(val, i);
        } else {
            panic!("return resulted in an error")
        }
    }
}
