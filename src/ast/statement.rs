use serde::{Deserialize, Serialize};
use super::expression::Expression;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statement {
    pub variable: String,
    pub expression: Expression,
}