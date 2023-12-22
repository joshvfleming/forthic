use super::{CoreModule, Modular};
use crate::errors::InterpreterError;
use crate::interpreter::Interpreter;
use crate::words::Word;

pub struct GlobalModule {
    core_module: CoreModule,
}

impl Modular for GlobalModule {
    fn find_word(&self, name: &str) -> Result<Option<Word>, InterpreterError> {
        if let Ok(Some(word)) = self.core_module.find_word(name) {
            return Ok(Some(word));
        }

        if let Some(word) = self.find_literal_word(name) {
            return Ok(Some(word));
        }

        Ok(None)
    }

    fn add_word(&mut self, word: Word) -> Result<(), InterpreterError> {
        self.core_module.add_word(word)
    }

    fn add_memo_words(&self, word: Word) -> Result<(), InterpreterError> {
        self.core_module.add_memo_words(word)
    }
}

impl GlobalModule {
    pub fn new(interpreter: &Interpreter) -> GlobalModule {
        GlobalModule {
            core_module: CoreModule::new("<GLOBAL>", interpreter, ""),
        }
    }

    pub fn find_literal_word(&self, name: &str) -> Option<Word> {
        None
    }
}
