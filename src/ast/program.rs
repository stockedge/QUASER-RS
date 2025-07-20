use serde::{Deserialize, Serialize};
use super::statement::Statement;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub return_var: String,
}