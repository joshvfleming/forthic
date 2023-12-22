use crate::errors::InterpreterError;
use crate::interpreter_error;
use crate::modules::{app_module::AppModule, global_module::GlobalModule, Modular, Module};
use crate::token::Token;
use crate::tokenizer::Tokenizer;
use crate::words::{definition::DefinitionWord, ExecutableWord, Word};
use std::any::Any;

pub struct Interpreter {
    stack: Vec<Box<dyn Any>>,
    global_module: Option<GlobalModule>,
    module_stack: Vec<Box<dyn Modular>>,
    is_compiling: bool,
    cur_definition: Option<DefinitionWord>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let interpreter = Interpreter {
            stack: Vec::new(),
            global_module: None,
            module_stack: Vec::new(),
            is_compiling: false,
            cur_definition: None,
        };

        Interpreter {
            global_module: Some(GlobalModule::new(&interpreter)),
            module_stack: vec![AppModule::new(&interpreter).into()],
            ..interpreter
        }
    }

    pub fn run(&mut self, source: &str) -> Result<(), InterpreterError> {
        let mut tokenizer = Tokenizer::new(source);
        loop {
            let token = tokenizer.next_token()?;
            match token {
                Token::EOS => break,
                t => self.handle_token(t)?,
            }
        }

        Ok(())
    }

    pub fn run_in_module(&self, module: &Module, source: &str) -> Result<(), InterpreterError> {
        Err(interpreter_error!("Not implemented"))
    }

    pub fn stack_push(&self, value: Box<dyn Any>) -> Result<(), InterpreterError> {
        Err(interpreter_error!("Not implemented"))
    }

    pub fn stack_pop(&self) -> Result<Box<dyn Any>, InterpreterError> {
        Err(interpreter_error!("Not implemented"))
    }

    pub fn module_stack_push(&self, module: &Module) -> Result<(), InterpreterError> {
        Err(interpreter_error!("Not implemented"))
    }

    pub fn module_stack_pop(&self) -> Result<Module, InterpreterError> {
        Err(interpreter_error!("Not implemented"))
    }

    pub fn find_module(&self, name: &str) -> Result<Module, InterpreterError> {
        Err(interpreter_error!("Not implemented"))
    }

    pub fn start_profiling(&self) -> Result<(), InterpreterError> {
        Err(interpreter_error!("Not implemented"))
    }

    pub fn add_timestamp(&self, label: &str) -> Result<(), InterpreterError> {
        Err(interpreter_error!("Not implemented"))
    }

    pub fn stop_profiling(&self) -> Result<(), InterpreterError> {
        Err(interpreter_error!("Not implemented"))
    }

    pub fn count_word(&self, w: &Word) -> Result<(), InterpreterError> {
        Err(interpreter_error!("Not implemented"))
    }

    pub fn start_profile_word(&self, w: &Word) -> Result<(), InterpreterError> {
        Err(interpreter_error!("Not implemented"))
    }

    pub fn end_profile_word(&self) -> Result<(), InterpreterError> {
        Err(interpreter_error!("Not implemented"))
    }

    fn get_app_module(&self) -> Result<&Box<dyn Modular>, InterpreterError> {
        if let Some(m) = self.module_stack.first() {
            return Ok(m);
        }

        Err(interpreter_error!("No application module found"))
    }

    /// Searches the interpreter for a word
    ///
    /// The module stack is searched top down. If the words cannot be found, the global module is searched.
    /// Note that the bottom of the module stack is always the application module.
    fn find_word(&self, name: &str) -> Result<Option<Word>, InterpreterError> {
        for module in self.module_stack.iter().rev() {
            if let Ok(Some(word)) = module.find_word(name) {
                return Ok(Some(word));
            }
        }

        if let Some(global_module) = &self.global_module {
            if let Ok(Some(word)) = global_module.find_word(name) {
                return Ok(Some(word));
            }
        }

        Err(interpreter_error!("Not implemented"))
    }

    fn handle_token(&mut self, token: Token) -> Result<(), InterpreterError> {
        match token {
            t @ Token::String(_) => self.handle_string_token(t)?,
            t @ Token::Comment(_) => self.handle_comment_token(t)?,
            t @ Token::StartArray => self.handle_start_array_token(t)?,
            t @ Token::EndArray => self.handle_end_array_token(t)?,
            t @ Token::StartModule(_) => self.handle_start_module_token(t)?,
            t @ Token::EndModule => self.handle_end_module_token(t)?,
            t @ Token::StartDefinition(_) => self.handle_start_definition_token(t)?,
            t @ Token::EndDefinition => self.handle_end_definition_token(t)?,
            t @ Token::StartMemo(_) => self.handle_start_memo_token(t)?,
            t @ Token::Word(_) => self.handle_word_token(t)?,
            _ => Err(interpreter_error!("Unknown token"))?,
        }
        Err(interpreter_error!("Not implemented"))
    }

    fn handle_string_token(&self, token: Token) -> Result<(), InterpreterError> {
        Err(interpreter_error!("Not implemented"))
    }

    fn handle_comment_token(&self, token: Token) -> Result<(), InterpreterError> {
        Err(interpreter_error!("Not implemented"))
    }

    fn handle_start_array_token(&self, token: Token) -> Result<(), InterpreterError> {
        Err(interpreter_error!("Not implemented"))
    }

    fn handle_end_array_token(&self, token: Token) -> Result<(), InterpreterError> {
        Err(interpreter_error!("Not implemented"))
    }

    fn handle_start_module_token(&self, token: Token) -> Result<(), InterpreterError> {
        Err(interpreter_error!("Not implemented"))
    }

    fn handle_end_module_token(&self, token: Token) -> Result<(), InterpreterError> {
        Err(interpreter_error!("Not implemented"))
    }

    fn handle_start_definition_token(&self, token: Token) -> Result<(), InterpreterError> {
        Err(interpreter_error!("Not implemented"))
    }

    fn handle_end_definition_token(&self, token: Token) -> Result<(), InterpreterError> {
        Err(interpreter_error!("Not implemented"))
    }

    fn handle_start_memo_token(&self, token: Token) -> Result<(), InterpreterError> {
        Err(interpreter_error!("Not implemented"))
    }

    fn handle_word_token(&mut self, token: Token) -> Result<(), InterpreterError> {
        if let Token::Word(w) = token {
            if let Some(word) = self.find_word(&w)? {
                self.handle_word(word)?;
            } else {
                Err(interpreter_error!(format!("Unknown word: '{w}'")))?;
            }
        }

        Ok(())
    }

    fn handle_word(&mut self, word: Word) -> Result<(), InterpreterError> {
        if self.is_compiling {
            match &mut self.cur_definition {
                Some(def) => def.add_word(word),
                None => Err(interpreter_error!(
                    "Interpreter is compiling, but there is no current definition"
                ))?,
            }
        } else {
            self.count_word(&word)?;
            word.execute(self)?;
        }

        Ok(())
    }
}
