/// A value is an immutable type that can be stored in a variable.
pub trait Value {
    fn get_value(&self) -> &dyn Value;
}
