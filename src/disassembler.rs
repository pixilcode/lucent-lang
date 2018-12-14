use crate::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, header: &str) -> String {
    let result = format!("== {} ==\n", header);

    disassemble_loop(chunk, 0, result.as_str())
}

fn disassemble_loop(chunk: &Chunk, instruction: usize, result: &str) -> String {
    if instruction >= chunk.get_size() {
        return String::from(result);
    }

    let (next, result) = disassemble_instruction(chunk, instruction, result);

    disassemble_loop(chunk, next, result.as_str())
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize, result: &str) -> (usize, String) {
    let current_line = match chunk.get_line(offset) {
        Some(line) => line,
        None => 0,
    };

    let previous_line = if offset > 0 {
        match chunk.get_line(offset - 1) {
            Some(line) => line,
            None => 0,
        }
    } else {
        0
    };

    let result = format!(
        "{}{:04} {:04} ",
        result,
        offset,
        if current_line == 0 {
            String::from("?")
        } else if current_line == previous_line {
            String::from("|")
        } else {
            current_line.to_string()
        }
    );

    let instruction = match chunk.get_byte(offset) {
        Some(i) => OpCode::from(i),
        None => OpCode::UnexpectedEndOfChunk,
    };

    match instruction {
        OpCode::Constant => {
            constant_instruction("OP_CONSTANT", false, chunk, offset, result.as_str())
        }
        OpCode::ConstantLong => {
            constant_instruction("OP_CONSTANT", true, chunk, offset, result.as_str())
        }
        OpCode::Return => simple_instruction("OP_RETURN", offset, result),
        OpCode::Negate => simple_instruction("OP_NEGATE", offset, result),
        OpCode::Add => simple_instruction("OP_ADD", offset, result),
        OpCode::Subtract => simple_instruction("OP_SUBTRACT", offset, result),
        OpCode::Multiply => simple_instruction("OP_MULTIPLY", offset, result),
        OpCode::Divide => simple_instruction("OP_DIVIDE", offset, result),
        OpCode::UnexpectedEndOfChunk => (offset + 1, format!("{}UNEXPECTED_END_OF_CHUNK", result)),
        OpCode::Invalid(code) => (offset + 1, format!("{}UNKNOWN_OPCODE {}", result, code)),
    }
}

fn simple_instruction(name: &str, offset: usize, result: String) -> (usize, String) {
    (offset + 1, format!("{}{}\n", result, name))
}

fn constant_instruction(
    name: &str,
    is_long: bool,
    chunk: &Chunk,
    offset: usize,
    result: &str,
) -> (usize, String) {
    let constant = match chunk.get_byte(offset + 1) {
        Some(i) => i as usize,
        None => return (offset + 2, format!("{}UNEXPECTED_END_OF_CHUNK", result)),
    };

    let constant = if is_long {
        (constant << 8) + match chunk.get_byte(offset + 2) {
            Some(i) => i as usize,
            None => return (offset + 3, format!("{}UNEXPECTED_END_OF_CHUNK", result)),
        }
    } else {
        constant
    };

    let offset = if is_long { offset + 3 } else { offset + 2 };

    let value = match chunk.get_constant(constant) {
        Some(i) => i,
        None => {
            return (
                offset,
                format!("{}{:<16} {:4} UNDEFINED_CONSTANT", result, name, constant),
            )
        }
    };
    (
        offset,
        format!("{}{:<16} {:4} '{}'\n", result, name, constant, value),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disassemble_chunk() {
        let chunk = Chunk::new().write_constant(1.2, 1);
        let chunk = chunk.write_chunk(&OpCode::Return, 1);
        assert_eq!(
            "\
== test code ==
0000 1    OP_CONSTANT         0 '1.2'
0002 |    OP_RETURN
",
            disassemble_chunk(&chunk, "test code")
        );
    }
}
