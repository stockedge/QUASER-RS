use serde::{Deserialize, Serialize};
use super::value::{Value, ConformValue};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Primitive(Value),
    
    Variable(String),
    
    Tuple(Vec<String>),
    
    ExternalCall {
        function: String,
        argument: String,
    },
    
    Projection {
        index: usize,
        variable: String,
    },
    
    Fold {
        list: String,
        initial: String,
        block: Block,
    },
    
    If {
        condition: String,
        then_block: Block,
        else_block: Option<Block>,
    },
    
    PendingCall(String),
    
    AbstractPrimitive(ConformValue),
    
    AbstractList(Vec<(Value, bool)>),
    
    Join(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub parameter: String,
    pub body: Vec<Statement>,
    pub return_var: String,
}

use super::statement::Statement;