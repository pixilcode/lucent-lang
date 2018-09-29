use chunk::Chunk;
use chunk::OpCode;

pub fn disassemble_chunk(chunk: &Chunk, header: &str) -> String {
	let result = format!("== {} ==\n", header);
	
	disassemble_loop(chunk, 0, result)
}

fn disassemble_loop(chunk: &Chunk, instruction: usize, result: String) -> String {
	if instruction >= chunk.get_size() {
		return result;
	}
	
	let (next, result) = disassemble_instruction(chunk, instruction, result);
	
	disassemble_loop(chunk, next, result)
}

fn disassemble_instruction(chunk: &Chunk, offset: usize, result: String) -> (usize, String) {
	let result = format!("{}{:04} ", result, offset);
	
	let instruction = match chunk.get_instruction(offset) {
		Some(i) => OpCode::from(i),
		None => OpCode::Invalid(u8::max_value()),
	};
	
	match instruction {
		OpCode::OpReturn => simple_instruction("OP_RETURN", offset, result),
		OpCode::Invalid(code) => (offset + 1, format!("{} Unknown opcode {}", result, code))
	}
}

fn simple_instruction(name: &str, offset: usize, result: String) -> (usize, String) {
	(offset + 1, format!("{}{}\n", result, name))
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_disassemble_chunk() {
		let chunk = Chunk::new().write_chunk(OpCode::OpReturn);
		assert_eq!("\
== test code ==
0000 OP_RETURN
",
			disassemble_chunk(&chunk, "test code"));
	}
}
