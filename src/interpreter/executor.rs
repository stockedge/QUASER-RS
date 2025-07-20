use crate::ast::Program;
use crate::runtime::{ExecutionState, Result};
use crate::interpreter::{apply_internal_rules, evaluate_join, find_dispatchable_calls, dispatch_calls, check_pending_calls};
use tokio::time::{sleep, Duration};

pub async fn execute(program: Program, with_approval: bool) -> Result<ExecutionState> {
    let mut state = ExecutionState::new(program);
    
    println!("=== Starting QUASAR Execution ===\n");
    
    loop {
        let dispatchable = find_dispatchable_calls(&state);
        
        if !dispatchable.is_empty() {
            println!("Found {} dispatchable calls", dispatchable.len());
            dispatch_calls(&mut state, dispatchable, with_approval).await?;
        }
        
        loop {
            let mut changed = false;
            
            changed |= check_pending_calls(&mut state).await?;
            
            changed |= apply_internal_rules(&mut state)?;
            
            changed |= evaluate_join(&mut state)?;
            
            if !changed {
                break;
            }
        }
        
        if state.pending_calls.is_empty() && find_dispatchable_calls(&state).is_empty() {
            break;
        }
        
        if !state.pending_calls.is_empty() {
            println!("Waiting for {} pending calls...", state.pending_calls.len());
            sleep(Duration::from_millis(100)).await;
        }
    }
    
    println!("\n=== Execution Complete ===");
    println!("Final scope:");
    for (var, value) in &state.scope {
        println!("  {} = {:?}", var, value);
    }
    
    if let Some(return_value) = state.lookup_var(&state.program.return_var) {
        println!("\nReturn value: {:?}", return_value);
    }
    
    Ok(state)
}