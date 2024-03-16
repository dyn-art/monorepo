#[derive(Debug, Clone)]
pub struct NoneErr {
    message: &'static str,
}

impl NoneErr {
    pub fn new(message: &'static str) -> Self {
        Self { message }
    }
}

impl std::fmt::Display for NoneErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for NoneErr {}
