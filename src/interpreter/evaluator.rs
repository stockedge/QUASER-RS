use crate::ast::*;
use crate::runtime::{ExecutionState, Result};

pub fn evaluate_join(state: &mut ExecutionState) -> Result<bool> {
    let mut changed = false;
    let mut new_statements = Vec::new();
    
    let statements = state.program.statements.clone();
    for stmt in &statements {
        match &stmt.expression {
            Expression::Join(vars) => {
                let mut all_resolved = true;
                let mut joined_value = ConformValue::certain(Value::Primitive(PrimitiveValue::Null));
                let mut first = true;
                
                for var in vars {
                    if let Some(value) = state.lookup_var(var) {
                        if first {
                            joined_value = value.clone();
                            first = false;
                        } else {
                            joined_value = joined_value.union(value);
                        }
                    } else {
                        all_resolved = false;
                        break;
                    }
                }
                
                if all_resolved && !first {
                    state.set_var(stmt.variable.clone(), joined_value);
                    changed = true;
                } else {
                    new_statements.push(stmt.clone());
                }
            }
            
            Expression::Primitive(value) => {
                state.set_var(stmt.variable.clone(), ConformValue::certain(value.clone()));
                changed = true;
            }
            
            Expression::AbstractPrimitive(cvalue) => {
                state.set_var(stmt.variable.clone(), cvalue.clone());
                changed = true;
            }
            
            Expression::Tuple(vars) => {
                let mut all_resolved = true;
                let mut elements = Vec::new();
                
                for var in vars {
                    if let Some(value) = state.lookup_var(var) {
                        if let Some(certain_val) = value.as_certain() {
                            elements.push(certain_val.clone());
                        } else {
                            all_resolved = false;
                            break;
                        }
                    } else {
                        all_resolved = false;
                        break;
                    }
                }
                
                if all_resolved {
                    state.set_var(
                        stmt.variable.clone(), 
                        ConformValue::certain(Value::Tuple(elements))
                    );
                    changed = true;
                } else {
                    new_statements.push(stmt.clone());
                }
            }
            
            _ => {
                new_statements.push(stmt.clone());
            }
        }
    }
    
    if changed {
        state.program.statements = new_statements;
    }
    
    Ok(changed)
}