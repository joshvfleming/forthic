use super::ExecutableWord;
use crate::errors::InterpreterError;
use crate::interpreter::Interpreter;

pub struct EndModuleWord {}

impl ExecutableWord for EndModuleWord {
    fn execute(&self, interpreter: &Interpreter) -> Result<(), InterpreterError> {
        interpreter.module_stack_pop()?;
        Ok(())
    }
}
