use super::{random::Random, settings::Settings};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use pyo3::{ToPyObject, Python, PyObject};

pub enum Value {
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    Array(Vec<Value>),
    Dict(HashMap<String, Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Array(_) => write!(f, "array"),
            Value::Dict(_) => write!(f, "dict"),
        }
    }
}

impl ToPyObject for Value {
    fn to_object(&self, py: Python) -> PyObject {
        match self {
            Value::String(s) => s.to_object(py),
            Value::Int(i) => i.to_object(py),
            Value::Float(f) => f.to_object(py),
            Value::Bool(b) => b.to_object(py),
            Value::Dict(d) => d.to_object(py),
            Value::Array(a) => a.to_object(py),
        }
    }
}

pub type Generator = fn(settings: &Settings) -> Value;
pub type Generators = Vec<Generator>;

pub struct RandomGenerator {}

impl RandomGenerator {
    pub fn random_int(_settings: &Settings) -> Value {
        let value = Random::int();

        Value::Int(value)
    }

    pub fn random_string(_settings: &Settings) -> Value {
        let value = Random::string();

        Value::String(value)
    }

    pub fn random_float(_settings: &Settings) -> Value {
        let value = Random::float();

        Value::Float(value)
    }

    pub fn random_bool(_settings: &Settings) -> Value {
        let value = Random::bool();

        Value::Bool(value)
    }

    pub fn random_dict(settings: &Settings) -> Value {
        let value = Random::dict(settings);
        
        Value::Dict(value)
    }

    pub fn random_array(settings: &Settings) -> Value {
        let value = Random::array(settings);
        
        Value::Array(value)
    }
}