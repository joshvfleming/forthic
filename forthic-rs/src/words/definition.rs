use super::{ExecutableWord, Word};
use crate::errors::InterpreterError;
use crate::interpreter::Interpreter;

pub struct DefinitionWord {
    name: String,
    words: Vec<Word>,
}

impl ExecutableWord for DefinitionWord {
    fn execute(&self, interpreter: &Interpreter) -> Result<(), InterpreterError> {
        for w in &self.words {
            interpreter.start_profile_word(w)?;
            w.execute(interpreter)?;
            interpreter.end_profile_word()?;
        }
        Ok(())
    }
}

impl DefinitionWord {
    pub fn new(name: &str) -> DefinitionWord {
        DefinitionWord {
            name: name.to_string(),
            words: Vec::new(),
        }
    }

    pub fn add_word(&mut self, word: Word) {
        self.words.push(word);
    }
}
