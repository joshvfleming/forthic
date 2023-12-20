use crate::value::Value;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    String(String),
    Comment(String),
    StartArray,
    EndArray,
    StartModule(String),
    EndModule,
    StartDefinition(String),
    EndDefinition,
    StartMemo(String),
    Word(String),
    EOS,
}

impl Value for Token {
    fn get_value(&self) -> &dyn Value {
        self
    }
}