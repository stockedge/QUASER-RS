use quasar::ast::*;
use quasar::interpreter::execute;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = create_sample_program();
    
    let _result = execute(program, true).await?;
    
    Ok(())
}

fn create_sample_program() -> Program {
    let statements = vec![
        Statement {
            variable: "image_patch".to_string(),
            expression: Expression::Primitive(Value::Primitive(PrimitiveValue::String(
                "image_patch_object".to_string()
            ))),
        },
        
        Statement {
            variable: "drinks".to_string(),
            expression: Expression::ExternalCall {
                function: "find".to_string(),
                argument: "image_patch".to_string(),
            },
        },
        
        Statement {
            variable: "drink_patches".to_string(),
            expression: Expression::Primitive(Value::List(vec![])),
        },
        
        Statement {
            variable: "final_patches".to_string(),
            expression: Expression::Fold {
                list: "drinks".to_string(),
                initial: "drink_patches".to_string(),
                block: Block {
                    parameter: "acc_and_drink".to_string(),
                    body: vec![
                        Statement {
                            variable: "acc".to_string(),
                            expression: Expression::Projection {
                                index: 0,
                                variable: "acc_and_drink".to_string(),
                            },
                        },
                        Statement {
                            variable: "drink".to_string(),
                            expression: Expression::Projection {
                                index: 1,
                                variable: "acc_and_drink".to_string(),
                            },
                        },
                        Statement {
                            variable: "drink_exists".to_string(),
                            expression: Expression::ExternalCall {
                                function: "exists".to_string(),
                                argument: "drink".to_string(),
                            },
                        },
                        Statement {
                            variable: "updated_acc".to_string(),
                            expression: Expression::If {
                                condition: "drink_exists".to_string(),
                                then_block: Block {
                                    parameter: "_".to_string(),
                                    body: vec![
                                        Statement {
                                            variable: "simple_query_result".to_string(),
                                            expression: Expression::ExternalCall {
                                                function: "simple_query".to_string(),
                                                argument: "drink".to_string(),
                                            },
                                        },
                                        Statement {
                                            variable: "yes".to_string(),
                                            expression: Expression::Primitive(Value::Primitive(
                                                PrimitiveValue::String("yes".to_string())
                                            )),
                                        },
                                        Statement {
                                            variable: "should_add".to_string(),
                                            expression: Expression::If {
                                                condition: "simple_query_result".to_string(),
                                                then_block: Block {
                                                    parameter: "_".to_string(),
                                                    body: vec![],
                                                    return_var: "acc".to_string(),
                                                },
                                                else_block: Some(Block {
                                                    parameter: "_".to_string(),
                                                    body: vec![],
                                                    return_var: "acc".to_string(),
                                                }),
                                            },
                                        },
                                    ],
                                    return_var: "should_add".to_string(),
                                },
                                else_block: Some(Block {
                                    parameter: "_".to_string(),
                                    body: vec![],
                                    return_var: "acc".to_string(),
                                }),
                            },
                        },
                    ],
                    return_var: "updated_acc".to_string(),
                },
            },
        },
    ];
    
    Program {
        statements,
        return_var: "final_patches".to_string(),
    }
}