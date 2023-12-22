pub mod app_module;
pub mod global_module;

use std::collections::HashMap;

use crate::errors::InterpreterError;
use crate::interpreter::Interpreter;
use crate::interpreter_error;
use crate::value::Value;
use crate::words::Word;
use enum_dispatch::enum_dispatch;

struct Variable {
    value: Option<Box<dyn Value>>,
    has_value: bool,
}

impl Variable {
    pub fn new() -> Variable {
        Variable {
            value: None,
            has_value: false,
        }
    }

    pub fn set_value(&mut self, value: Box<dyn Value>) {
        self.value = Some(value);
        self.has_value = true;
    }

    pub fn get_value(&self) -> &Option<Box<dyn Value>> {
        &self.value
    }
}

#[enum_dispatch]
pub enum Module {
    Core(CoreModule),
}

#[enum_dispatch(Module)]
pub trait Modular {
    /// Searches module for a word with the specified `name`
    fn find_word(&self, name: &str) -> Result<Option<Word>, InterpreterError>;

    /// Adds a `word` to a module
    fn add_word(&mut self, word: Word) -> Result<(), InterpreterError>;

    /// Adds memo words to a module based on a core definition word
    fn add_memo_words(&self, word: Word) -> Result<(), InterpreterError>;
}

/// Modules store Forthic words and variables
pub struct CoreModule {
    name: String,
    words: Vec<Word>,
    exportable: Vec<String>,
    variables: HashMap<String, Variable>,
    modules: HashMap<String, Module>,
    source: String,
}

impl Modular for CoreModule {
    /// Searches module for a word with the specified `name`
    fn find_word(&self, name: &str) -> Result<Option<Word>, InterpreterError> {
        Err(interpreter_error!("Not implemented"))
    }

    /// Adds a `word` to a module
    fn add_word(&mut self, word: Word) -> Result<(), InterpreterError> {
        self.words.push(word);
        Ok(())
    }

    /// Adds memo words to a module based on a core definition word
    ///
    /// For a word named "MY-MEMO", this adds the following words:
    ///    * MY-MEMO  (memoizes the execution of the provided definition word)
    ///    * MY-MEMO!  (re-runs MY-MEMO to update its memoized value)
    ///    * MY-MEMO!@  (runs MY-MEMO! and returns then returns the new memo value)
    fn add_memo_words(&self, word: Word) -> Result<(), InterpreterError> {
        Ok(())
    }
}

impl CoreModule {
    pub fn new(name: &str, interpreter: &Interpreter, source: &str) -> CoreModule {
        CoreModule {
            name: name.to_string(),
            words: Vec::new(),
            exportable: Vec::new(),
            variables: HashMap::new(),
            modules: HashMap::new(),
            source: source.to_string(),
        }
    }
}
