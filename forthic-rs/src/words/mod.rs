pub mod definition;
pub mod end_array;
pub mod end_module;
pub mod start_module;

use crate::errors::InterpreterError;
use crate::interpreter::Interpreter;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub enum Word {
    Definition(definition::DefinitionWord),
    EndArray(end_array::EndArrayWord),
    StartModule(start_module::StartModuleWord),
    EndModule(end_module::EndModuleWord),
}

/// Forthic words can be executed by the interpreter or compiled into Forthic definitions
#[enum_dispatch(Word)]
pub trait ExecutableWord {
    /// Called when a Forthic word is executed by the interpreter
    /// Words take parameters from the interpreter `stack` and return values to it.
    fn execute(&self, interpreter: &Interpreter) -> Result<(), InterpreterError>;
}
