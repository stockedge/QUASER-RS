use crate::ast::*;
use crate::runtime::{ExecutionState, PendingCall, get_external_function, Result, QuasarError};
use tokio::task;

#[derive(Debug, Clone)]
pub struct DispatchableCall {
    pub assignment_var: String,
    pub function: String,
    pub argument: ConformValue,
}

pub fn find_dispatchable_calls(state: &ExecutionState) -> Vec<DispatchableCall> {
    let mut calls = Vec::new();
    
    for stmt in &state.program.statements {
        if let Expression::ExternalCall { function, argument } = &stmt.expression {
            if let Some(arg_value) = state.lookup_var(argument) {
                calls.push(DispatchableCall {
                    assignment_var: stmt.variable.clone(),
                    function: function.clone(),
                    argument: arg_value.clone(),
                });
            }
        }
    }
    
    calls
}

pub async fn dispatch_calls(
    state: &mut ExecutionState,
    calls: Vec<DispatchableCall>,
    with_approval: bool,
) -> Result<()> {
    for call in calls {
        if with_approval {
            println!("\n=== External Call Request ===");
            println!("Function: {}", call.function);
            println!("Argument: {:?}", call.argument);
            println!("Approve? (y/n): ");
            
            use tokio::io::{AsyncBufReadExt, BufReader};
            let stdin = tokio::io::stdin();
            let mut reader = BufReader::new(stdin);
            let mut input = String::new();
            reader.read_line(&mut input).await.map_err(|e| {
                QuasarError::RuntimeError(format!("Failed to read input: {}", e))
            })?;
            
            if !input.trim().eq_ignore_ascii_case("y") {
                println!("Call rejected by user");
                continue;
            }
        }
        
        let call_id = state.generate_call_id();
        
        let function_name = call.function.clone();
        let argument = call.argument.clone();
        
        let handle = task::spawn(async move {
            if let Some(func) = get_external_function(&function_name) {
                func.call(&argument).await
            } else {
                Err(QuasarError::ExternalFunctionError(
                    format!("Unknown function: {}", function_name)
                ))
            }
        });
        
        state.pending_calls.push(PendingCall {
            id: call_id.clone(),
            assignment_var: call.assignment_var.clone(),
            handle,
        });
        
        for stmt in &mut state.program.statements {
            if stmt.variable == call.assignment_var {
                stmt.expression = Expression::PendingCall(call_id.clone());
                break;
            }
        }
    }
    
    Ok(())
}