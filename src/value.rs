// TODO Move all of this to VM module
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    // template: Option<Rc<Template>>, // TODO Implement templates
    data: DataType,
}

impl Value {
    // Float datatype functions
    /// Creates a Value with the Float data type from a given float
    pub fn float(val: f64) -> Self {
        Value {
            data: DataType::Float(val),
        }
    }

    /// Returns true if the value contains a float
    ///
    /// This function should always be used before
    /// `map_float`, `as_float`, or `compare_floats`
    ///
    /// # Examples
    /// ```
    /// use lucent_lang::value::Value;
    ///
    /// let f = Value::float(0.0);
    /// assert!(f.is_float());
    ///
    /// let u = Value::unit();
    /// assert!(!u.is_float());
    /// ```
    pub fn is_float(&self) -> bool {
        match self.data {
            DataType::Float(_) => true,
            _ => false,
        }
    }

    /// Takes a function and maps that function over
    /// the float value of the Value object
    ///
    /// Before using this function, it is imperative
    /// that `is_float` is called to ensure that the
    /// value is a float value
    ///
    /// # Panics
    /// This function panics if the value is not a float
    ///
    /// # Examples
    /// ```
    /// use lucent_lang::value::Value;
    ///
    /// let f = Value::float(5.0);
    ///
    /// if f.is_float() {
    ///     let f = f.map_float(|a| a + 5.0);
    ///     assert_eq!(Value::float(10.0), f);
    /// } else {
    ///     panic!("f is not a float");
    /// }
    /// ```
    pub fn map_float<F>(&self, map: F) -> Value
    where
        F: Fn(f64) -> f64,
    {
        match self.data {
            DataType::Float(val) => Value::float(map(val)),
            _ => panic!("Tried to map non-float value as a float"),
        }
    }

    /// Get the float value of a Value object
    ///
    /// Before using this function, it is imperative
    /// that `is_float` is called to ensure that the
    /// value is a float value
    ///
    /// # Panics
    /// This function panics if the value is not a float
    ///
    /// # Examples
    /// ```
    /// use lucent_lang::value::Value;
    ///
    /// let f = Value::float(1.0);
    ///
    /// if f.is_float() {
    ///     assert_eq!(1.0, f.as_float());
    /// } else {
    ///     panic!("f is not a float");
    /// }
    /// ```
    pub fn as_float(&self) -> f64 {
        match self.data {
            DataType::Float(val) => val,
            _ => panic!("Tried to get float from non-float value."),
        }
    }

    fn compare_floats(a: Value, b: Value, decimal_places: u8) -> bool {
        match (a.data, b.data) {
            (DataType::Float(a), DataType::Float(b)) => {
                let factor = 10.0f64.powi(i32::from(decimal_places));
                let a = (a * factor).trunc();
                let b = (b * factor).trunc();
                (a - b).abs() <= 0.0
            }
            _ => panic!("One or more values are not floats"),
        }
    }

    // Unit datatype functions
    /// Creates a Value with the Unit data type
    pub fn unit() -> Self {
        Value {
            data: DataType::Unit,
        }
    }

    /// Returns true if the value is a unit value
    ///
    /// # Examples
    /// ```
    /// use lucent_lang::value::Value;
    ///
    /// let u = Value::unit();
    /// assert!(u.is_unit());
    /// ```
    pub fn is_unit(&self) -> bool {
        match self.data {
            DataType::Unit => true,
            _ => false,
        }
    }

    // Compare values
    pub fn compare_values(a: Value, b: Value) -> bool {
        match (a.data, b.data) {
            (DataType::Float(a), DataType::Float(b)) => {
                Value::compare_floats(Value::float(a), Value::float(b), 10)
            }
            (DataType::Unit, DataType::Unit) => true,
            _ => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.data {
                DataType::Float(f) => f.to_string(),
                DataType::Unit => "unit".to_string(),
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Float(f64),
    Unit,
}

#[derive(Default, Debug)]
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
