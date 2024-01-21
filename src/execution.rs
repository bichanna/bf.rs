pub enum ExecutionType {
    Interpreter,
}

impl From<String> for ExecutionType {
    fn from(value: String) -> Self {
        match &value[..] {
            "-i" => Self::Interpreter,
            _ => Self::Interpreter,
        }
    }
}
