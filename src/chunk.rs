use std::convert::From;

pub enum OpCode {
	OpReturn,
	Invalid(u8),
}

impl OpCode {
	fn to_byte(&self) -> u8 {
		match self {
			OpCode::OpReturn => 0,
			OpCode::Invalid(code) => code.clone(),
		}
	}
}

impl From<u8> for OpCode {
	fn from(byte: u8) -> OpCode {
		match byte {
			0 => OpCode::OpReturn,
			_ => OpCode::Invalid(byte),
		}
	}
}

pub struct Chunk(Vec<u8>);

impl Chunk {
	pub fn new() -> Chunk {
		Chunk(vec![])
	}
	
	pub fn write_chunk(mut self, op_code: OpCode) -> Chunk {
		self.0.push(op_code.to_byte());
		self
	}
	
	pub fn get_size(&self) -> usize {
		self.0.len()
	}
	
	pub fn get_instruction(&self, offset: usize) -> Option<OpCode> {
		self.0.get(offset).map(|b| OpCode::from(*b))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
    #[test]
    fn test_chunks() {
    	let chunk = Chunk::new();
    	let chunk = chunk.write_chunk(OpCode::OpReturn);
    	assert_eq!(0, chunk.get_instruction(0).unwrap().to_byte());
    }
}
