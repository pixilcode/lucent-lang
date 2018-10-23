// TODO Move this to VM module

use std::convert::From;
use value::Value;
use value::ValueArray;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum OpCode {
    OpConstant,
    OpConstantLong,
    OpReturn,
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    UnexpectedEndOfChunk,
    Invalid(u8),
}

impl OpCode {
    pub fn to_byte(&self) -> u8 {
        match self {
            OpCode::OpConstant => 0,
            OpCode::OpConstantLong => 1,
            OpCode::OpReturn => 2,
            OpCode::OpNegate => 3,
            OpCode::OpAdd => 4,
            OpCode::OpSubtract => 5,
            OpCode::OpMultiply => 6,
            OpCode::OpDivide => 7,
            OpCode::UnexpectedEndOfChunk => 255,
            OpCode::Invalid(code) => *code,
        }
    }
}

impl From<u8> for OpCode {
    fn from(byte: u8) -> OpCode {
        match byte {
            0 => OpCode::OpConstant,
            1 => OpCode::OpConstantLong,
            2 => OpCode::OpReturn,
            3 => OpCode::OpNegate,
            4 => OpCode::OpAdd,
            5 => OpCode::OpSubtract,
            6 => OpCode::OpMultiply,
            7 => OpCode::OpDivide,
            255 => OpCode::UnexpectedEndOfChunk,
            _ => OpCode::Invalid(byte),
        }
    }
}

#[derive(Debug)]
pub struct Chunk {
    code: Vec<u8>,
    lines: Vec<u32>,
    constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: vec![],
            lines: vec![], // TODO Implement compressed line storage (see chapter 1 of part 3)
            constants: ValueArray::new(),
        }
    }

    pub fn write_chunk(mut self, op_code: OpCode, line: u32) -> Self {
        self.write_byte(op_code.to_byte(), line);
        self
    }

    pub fn write_constant(mut self, constant: Value, line: u32) -> Self {
        let location = self.constants.write_value(constant);
        if location <= u8::max_value() as usize {
            self.write_byte(OpCode::OpConstant.to_byte(), line);
            self.write_byte(location as u8, line);
        } else {
            self.write_byte(OpCode::OpConstantLong.to_byte(), line);
            self.write_byte((location >> 8) as u8, line);
            self.write_byte(location as u8, line);
        }
        self
    }

    fn write_byte(&mut self, byte: u8, line: u32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn get_size(&self) -> usize {
        self.code.len()
    }

    pub fn get_byte(&self, offset: usize) -> Option<u8> {
        self.code.get(offset).cloned()
    }

    pub fn get_constant(&self, constant_index: usize) -> Option<Value> {
        self.constants.get_constant(constant_index)
    }

    pub fn get_long_constant(
        &self,
        constant_index_first_byte: usize,
        constant_index_second_byte: usize,
    ) -> Option<Value> {
        self.get_constant(((constant_index_first_byte) << 8) + (constant_index_second_byte))
    }

    pub fn get_line(&self, offset: usize) -> Option<u32> {
        self.lines.get(offset).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::HashMap;

    #[test]
    fn test_chunks() {
        let chunk = Chunk::new();
        let chunk = chunk.write_chunk(OpCode::OpReturn, 1);
        assert_eq!(OpCode::OpReturn.to_byte(), chunk.get_byte(0).unwrap());
    }

    #[test]
    fn test_op_codes() {
        let mut map = HashMap::new();
        map.insert(OpCode::OpConstant, 0);
        map.insert(OpCode::OpConstantLong, 1);
        map.insert(OpCode::OpReturn, 2);
        map.insert(OpCode::OpNegate, 3);
        map.insert(OpCode::OpAdd, 4);
        map.insert(OpCode::OpSubtract, 5);
        map.insert(OpCode::OpMultiply, 6);
        map.insert(OpCode::OpDivide, 7);
        map.insert(OpCode::Invalid(254), 254);
        map.insert(OpCode::UnexpectedEndOfChunk, 255);

        map.iter().for_each(|(code, byte)| {
            let chunk = Chunk::new().write_chunk(code.clone(), 1);
            assert_eq!(*byte, chunk.get_byte(0).unwrap());
            assert_eq!(*code, OpCode::from(*byte));
        });
    }

    #[test]
    fn test_constant() {
        let chunk = Chunk::new();
        let chunk = chunk.write_constant(1.2, 1);

        assert_eq!(OpCode::OpConstant.to_byte(), chunk.get_byte(0).unwrap());
        assert_eq!(
            1.2,
            chunk
                .get_constant(chunk.get_byte(1).unwrap() as usize)
                .unwrap()
        )
    }

    #[test]
    fn test_long_constant() {
        let chunk = Chunk::new();
        let chunk = write_constants(chunk, u8::max_value() as usize + 1);
        let chunk = chunk.write_constant(0f64, 1);

        assert_eq!(
            OpCode::OpConstantLong.to_byte(),
            chunk.get_byte(512).unwrap()
        );
        assert_eq!(
            0f64,
            chunk
                .get_long_constant(
                    chunk.get_byte(513).unwrap() as usize,
                    chunk.get_byte(514).unwrap() as usize
                ).unwrap()
        );
    }

    // A recursive function that writes 'fill' number of constants to a chunk
    fn write_constants(chunk: Chunk, fill: usize) -> Chunk {
        if fill <= 0 {
            chunk
        } else {
            write_constants(chunk.write_constant(fill as f64, 1), fill - 1)
        }
    }
}
