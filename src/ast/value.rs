use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PrimitiveValue {
    Boolean(bool),
    Integer(i64),
    Float(ordered_float::OrderedFloat<f64>),
    String(String),
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Value {
    Primitive(PrimitiveValue),
    List(Vec<Value>),
    Tuple(Vec<Value>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConformValue {
    pub possibilities: BTreeSet<Value>,
}

impl ConformValue {
    pub fn certain(value: Value) -> Self {
        let mut possibilities = BTreeSet::new();
        possibilities.insert(value);
        ConformValue { possibilities }
    }

    pub fn uncertain(values: impl IntoIterator<Item = Value>) -> Self {
        ConformValue {
            possibilities: values.into_iter().collect(),
        }
    }

    pub fn union(&self, other: &Self) -> Self {
        ConformValue {
            possibilities: self.possibilities.union(&other.possibilities).cloned().collect(),
        }
    }

    pub fn is_certain(&self) -> bool {
        self.possibilities.len() == 1
    }

    pub fn as_certain(&self) -> Option<&Value> {
        if self.is_certain() {
            self.possibilities.iter().next()
        } else {
            None
        }
    }
}