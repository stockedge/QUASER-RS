use crate::ast::*;
use crate::runtime::{ExecutionState, Result, QuasarError};

pub fn apply_internal_rules(state: &mut ExecutionState) -> Result<bool> {
    let mut changed = false;
    let mut new_statements = Vec::new();
    
    let statements = state.program.statements.clone();
    for stmt in &statements {
        match &stmt.expression {
            Expression::Variable(src_var) => {
                if let Some(value) = state.lookup_var(src_var) {
                    state.set_var(stmt.variable.clone(), value.clone());
                    changed = true;
                } else {
                    new_statements.push(stmt.clone());
                }
            }
            
            Expression::Projection { index, variable } => {
                if let Some(value) = state.lookup_var(variable) {
                    if let Some(certain_val) = value.as_certain() {
                        if let Value::Tuple(elements) = certain_val {
                            if *index < elements.len() {
                                let projected = ConformValue::certain(elements[*index].clone());
                                state.set_var(stmt.variable.clone(), projected);
                                changed = true;
                            } else {
                                return Err(QuasarError::InvalidOperation(
                                    format!("Tuple index {} out of bounds", index)
                                ));
                            }
                        } else {
                            return Err(QuasarError::TypeError(
                                "Projection can only be applied to tuples".to_string()
                            ));
                        }
                    } else {
                        new_statements.push(stmt.clone());
                    }
                } else {
                    new_statements.push(stmt.clone());
                }
            }
            
            Expression::If { condition, then_block, else_block } => {
                if let Some(cond_value) = state.lookup_var(condition) {
                    let has_true = cond_value.possibilities.contains(&Value::Primitive(PrimitiveValue::Boolean(true)));
                    let has_false = cond_value.possibilities.contains(&Value::Primitive(PrimitiveValue::Boolean(false)));
                    
                    if has_true && !has_false {
                        expand_block(&stmt.variable, then_block, &mut new_statements);
                        changed = true;
                    } else if !has_true && has_false {
                        if let Some(else_block) = else_block {
                            expand_block(&stmt.variable, else_block, &mut new_statements);
                            changed = true;
                        }
                    } else if has_true && has_false {
                        let then_result_var = format!("{}_then", stmt.variable);
                        let else_result_var = format!("{}_else", stmt.variable);
                        
                        expand_block(&then_result_var, then_block, &mut new_statements);
                        if let Some(else_block) = else_block {
                            expand_block(&else_result_var, else_block, &mut new_statements);
                        }
                        
                        new_statements.push(Statement {
                            variable: stmt.variable.clone(),
                            expression: Expression::Join(vec![then_result_var, else_result_var]),
                        });
                        changed = true;
                    } else {
                        new_statements.push(stmt.clone());
                    }
                } else {
                    new_statements.push(stmt.clone());
                }
            }
            
            Expression::Fold { list, initial, block } => {
                let should_expand = if let (Some(list_value), Some(initial_value)) = (state.lookup_var(list), state.lookup_var(initial)) {
                    if let (Some(certain_list), Some(_)) = (list_value.as_certain(), initial_value.as_certain()) {
                        matches!(certain_list, Value::List(_))
                    } else {
                        false
                    }
                } else {
                    false
                };
                
                if should_expand {
                    let list_value = state.lookup_var(list).unwrap();
                    let certain_list = list_value.as_certain().unwrap();
                    if let Value::List(elements) = certain_list {
                        let elements_clone = elements.clone();
                        let mut accumulator_var = initial.clone();
                        
                        for (i, element) in elements_clone.iter().enumerate() {
                            let iter_var = format!("{}_iter_{}", stmt.variable, i);
                            let acc_var = format!("{}_acc_{}", stmt.variable, i);
                            
                            state.set_var(iter_var.clone(), ConformValue::certain(element.clone()));
                            
                            let tuple_var = format!("{}_tuple_{}", stmt.variable, i);
                            new_statements.push(Statement {
                                variable: tuple_var.clone(),
                                expression: Expression::Tuple(vec![accumulator_var.clone(), iter_var]),
                            });
                            
                            expand_block_with_param(&acc_var, block, &tuple_var, &mut new_statements);
                            
                            accumulator_var = acc_var;
                        }
                        
                        new_statements.push(Statement {
                            variable: stmt.variable.clone(),
                            expression: Expression::Variable(accumulator_var),
                        });
                        changed = true;
                    }
                } else {
                    new_statements.push(stmt.clone());
                }
            }
            
            Expression::PendingCall(_call_id) => {
                new_statements.push(stmt.clone());
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

pub async fn check_pending_calls(state: &mut ExecutionState) -> Result<bool> {
    let mut changed = false;
    let mut completed_indices = Vec::new();
    
    for (index, pc) in state.pending_calls.iter().enumerate() {
        if pc.handle.is_finished() {
            completed_indices.push(index);
        }
    }
    
    for index in completed_indices.into_iter().rev() {
        let pc = state.pending_calls.remove(index);
        match pc.handle.await {
            Ok(Ok(result)) => {
                let var_name = pc.assignment_var.clone();
                state.set_var(var_name.clone(), result);
                
                for stmt in &mut state.program.statements {
                    if let Expression::PendingCall(id) = &stmt.expression {
                        if id == &pc.id {
                            stmt.expression = Expression::Variable(var_name.clone());
                            changed = true;
                            break;
                        }
                    }
                }
            }
            Ok(Err(e)) => return Err(e),
            Err(_) => return Err(QuasarError::RuntimeError("Task panicked".to_string())),
        }
    }
    
    Ok(changed)
}

fn expand_block(result_var: &str, block: &Block, statements: &mut Vec<Statement>) {
    let empty_tuple_var = format!("{}_empty", result_var);
    statements.push(Statement {
        variable: empty_tuple_var.clone(),
        expression: Expression::Tuple(vec![]),
    });
    
    expand_block_with_param(result_var, block, &empty_tuple_var, statements);
}

fn expand_block_with_param(result_var: &str, block: &Block, param_var: &str, statements: &mut Vec<Statement>) {
    let param_subst = |var: &str| {
        if var == &block.parameter {
            param_var.to_string()
        } else {
            var.to_string()
        }
    };
    
    for block_stmt in &block.body {
        let new_stmt = Statement {
            variable: if block_stmt.variable == block.return_var {
                result_var.to_string()
            } else {
                block_stmt.variable.clone()
            },
            expression: substitute_expression(&block_stmt.expression, &param_subst),
        };
        statements.push(new_stmt);
    }
    
    if block.body.is_empty() || block.body.last().unwrap().variable != block.return_var {
        statements.push(Statement {
            variable: result_var.to_string(),
            expression: Expression::Variable(param_subst(&block.return_var)),
        });
    }
}

fn substitute_expression<F>(expr: &Expression, subst: &F) -> Expression 
where
    F: Fn(&str) -> String
{
    match expr {
        Expression::Variable(v) => Expression::Variable(subst(v)),
        Expression::Tuple(vars) => Expression::Tuple(vars.iter().map(|v| subst(v)).collect()),
        Expression::ExternalCall { function, argument } => Expression::ExternalCall {
            function: function.clone(),
            argument: subst(argument),
        },
        Expression::Projection { index, variable } => Expression::Projection {
            index: *index,
            variable: subst(variable),
        },
        Expression::Fold { list, initial, block } => Expression::Fold {
            list: subst(list),
            initial: subst(initial),
            block: block.clone(),
        },
        Expression::If { condition, then_block, else_block } => Expression::If {
            condition: subst(condition),
            then_block: then_block.clone(),
            else_block: else_block.clone(),
        },
        Expression::Join(vars) => Expression::Join(vars.iter().map(|v| subst(v)).collect()),
        _ => expr.clone(),
    }
}