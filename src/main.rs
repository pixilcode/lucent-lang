extern crate clox;

use clox::chunk::{Chunk, OpCode};
use clox::virtual_machine::{DebugFlag, DebugFlags, VM};

fn main() {
    let chunk = Chunk::new();
    let chunk = chunk
        .write_constant(1.2, 1)
        .write_chunk(OpCode::OpNegate, 1)
        .write_constant(4.3, 1)
        .write_chunk(OpCode::OpReturn, 2);
    let debug = DebugFlags::new().set_flag(DebugFlag::PrintInstructions, true);
    VM::debug(chunk, debug); // TODO Create a debug object and pass them into interpret or create seperate debug method
}
