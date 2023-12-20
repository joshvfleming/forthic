use super::ExecutableWord;
use crate::errors::InterpreterError;
use crate::interpreter::Interpreter;

/// This indicates the start of a module
///
/// See `docs/ARCHITECTURE.md` for more details on modules.
pub struct StartModuleWord {
    name: String,
}

impl ExecutableWord for StartModuleWord {
    fn execute(&self, interpreter: &Interpreter) -> Result<(), InterpreterError> {
        // The app module is the only module with a blank name
        if self.name == "" {
            //interpreter.module_stack_push(interpreter.app_module);
            return Ok(());
        }

        // If the module is used by the current module, push it onto the module stack;
        // otherwise, create a new module and push that onto the module stack.
        //let module = interpreter.cur_module().find_module(&self.name);

        // Check app module
        //if module.is_none() {
        //  module = interpreter.app_module.find_module(&self.name);
        //}

        //if module.is_none() {
        //    module = Some(Module::new(&self.name, interpreter));
        //    interpreter.cur_module().register_module(&self.name, module.unwrap());
        //}

        //interpreter.module_stack_push(module.unwrap())?;

        Ok(())
    }
}

impl StartModuleWord {
    fn new(name: &str) -> StartModuleWord {
        StartModuleWord {
            name: name.to_string(),
        }
    }
}
