use super::ExecutableWord;
use crate::errors::InterpreterError;
use crate::interpreter::Interpreter;
use crate::token::Token;
use std::any::Any;

/// This represents the end of an array
pub struct EndArrayWord {}

impl ExecutableWord for EndArrayWord {
    fn execute(&self, interpreter: &Interpreter) -> Result<(), InterpreterError> {
        let mut items: Vec<Box<dyn Any>> = Vec::new();
        loop {
            let item = interpreter.stack_pop()?;

            if let Some(t) = item.downcast_ref::<Token>() {
                if *t == Token::StartArray {
                    break;
                }
            } else {
                items.push(item);
            }
        }
        items.reverse();
        interpreter.stack_push(Box::new(items))?;
        Ok(())
    }
}
