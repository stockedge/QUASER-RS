use async_trait::async_trait;
use crate::ast::{Value, ConformValue};
use super::error::Result;
use std::time::Duration;
use tokio::time::sleep;

#[async_trait]
pub trait ExternalFunction: Send + Sync {
    async fn call(&self, args: &ConformValue) -> Result<ConformValue>;
}

pub struct FindFunction;

#[async_trait]
impl ExternalFunction for FindFunction {
    async fn call(&self, _args: &ConformValue) -> Result<ConformValue> {
        println!("External: Calling find() function...");
        sleep(Duration::from_secs(1)).await;
        
        let patches = vec![
            Value::Primitive(crate::ast::PrimitiveValue::String("patch1".to_string())),
            Value::Primitive(crate::ast::PrimitiveValue::String("patch2".to_string())),
        ];
        
        Ok(ConformValue::certain(Value::List(patches)))
    }
}

pub struct SimpleQueryFunction;

#[async_trait]
impl ExternalFunction for SimpleQueryFunction {
    async fn call(&self, _args: &ConformValue) -> Result<ConformValue> {
        println!("External: Calling simple_query() function...");
        sleep(Duration::from_millis(500)).await;
        
        Ok(ConformValue::certain(Value::Primitive(
            crate::ast::PrimitiveValue::String("yes".to_string())
        )))
    }
}

pub struct ExistsFunction;

#[async_trait]
impl ExternalFunction for ExistsFunction {
    async fn call(&self, _args: &ConformValue) -> Result<ConformValue> {
        println!("External: Calling exists() function...");
        sleep(Duration::from_millis(300)).await;
        
        Ok(ConformValue::certain(Value::Primitive(
            crate::ast::PrimitiveValue::Boolean(true)
        )))
    }
}

pub fn get_external_function(name: &str) -> Option<Box<dyn ExternalFunction>> {
    match name {
        "find" => Some(Box::new(FindFunction)),
        "simple_query" => Some(Box::new(SimpleQueryFunction)),
        "exists" => Some(Box::new(ExistsFunction)),
        _ => None,
    }
}