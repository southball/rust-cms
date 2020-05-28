/// An error class with send trait.
#[derive(Debug)]
pub struct SendError {
    pub message: String,
}

impl From<String> for SendError {
    fn from(message: String) -> SendError {
        SendError { message }
    }
}

impl std::fmt::Display for SendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SendError: {}", self.message)
    }
}

impl std::error::Error for SendError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
