use std::collections::HashMap;
use tokio::task::JoinHandle;
use crate::ast::{Program, ConformValue};
use super::error::Result;

pub struct PendingCall {
    pub id: String,
    pub assignment_var: String,
    pub handle: JoinHandle<Result<ConformValue>>,
}

pub struct ExecutionState {
    pub program: Program,
    pub pending_calls: Vec<PendingCall>,
    pub scope: HashMap<String, ConformValue>,
    pub call_counter: usize,
}

impl ExecutionState {
    pub fn new(program: Program) -> Self {
        ExecutionState {
            program,
            pending_calls: Vec::new(),
            scope: HashMap::new(),
            call_counter: 0,
        }
    }
    
    pub fn generate_call_id(&mut self) -> String {
        self.call_counter += 1;
        format!("?S{}", self.call_counter)
    }
    
    pub fn lookup_var(&self, name: &str) -> Option<&ConformValue> {
        self.scope.get(name)
    }
    
    pub fn set_var(&mut self, name: String, value: ConformValue) {
        self.scope.insert(name, value);
    }
}