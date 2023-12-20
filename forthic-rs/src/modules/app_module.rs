use super::{CoreModule, Modular};
use crate::errors::InterpreterError;
use crate::interpreter::Interpreter;
use crate::words::Word;
use std::collections::HashMap;

pub struct AppModule {
    screens: HashMap<String, String>,
    core_module: CoreModule,
}

impl AppModule {
    pub fn new(interpreter: &Interpreter) -> AppModule {
        AppModule {
            screens: HashMap::new(),
            core_module: CoreModule::new("", interpreter, ""),
        }
    }

    pub fn set_screen(&mut self, name: &str, content: &str) {
        self.screens.insert(name.to_string(), content.to_string());
    }

    pub fn get_screen(&self, name: &str) -> Option<&String> {
        self.screens.get(name)
    }
}

impl Modular for AppModule {
    fn find_word(&self, name: &str) -> Result<Option<Word>, InterpreterError> {
        self.core_module.find_word(name)
    }

    fn add_word(&mut self, word: Word) -> Result<(), InterpreterError> {
        self.core_module.add_word(word)
    }

    fn add_memo_words(&self, word: Word) -> Result<(), InterpreterError> {
        self.core_module.add_memo_words(word)
    }
}
