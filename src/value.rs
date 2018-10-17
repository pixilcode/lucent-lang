pub type Value = f64;

pub fn string_from(value: Value) -> String {
    format!("{}", value)
}

#[derive(Debug)]
pub struct ValueArray(Vec<Value>);

impl ValueArray {
    pub fn new() -> Self {
        ValueArray(vec![])
    }

    pub fn write_value(&mut self, value: Value) -> usize {
        self.0.push(value);
        self.0.len() - 1
    }

    pub fn get_constant(&self, constant_index: usize) -> Option<Value> {
        self.0.get(constant_index).cloned() // Cloned to get Option<Value> instead of Option<&Value>
    }

    pub fn get_size(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {}
