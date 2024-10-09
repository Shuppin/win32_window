/// A type alias for the result of operations that can return a `PalmError`.
pub type PalmResult<T> = std::result::Result<T, PalmError>;

/// Enumeration representing the different kinds of errors that can occur in the Palm application.
#[derive(Debug)]
pub enum PalmErrorKind {
    /// Represents an error where there is not enough memory available.
    NotEnoughMemory,
}

impl PalmErrorKind {
    /// Returns a human-readable description of the error kind.
    ///
    /// # Returns
    ///
    /// A string slice that provides a description of the error.
    pub fn description(&self) -> &str {
        match self {
            Self::NotEnoughMemory => "Not enough memory",
        }
    }
}

impl Into<PalmError> for PalmErrorKind {
    /// Converts a `PalmErrorKind` into a `PalmError`.
    ///
    /// # Returns
    ///
    /// A `PalmError` that encapsulates the kind of error and its description.
    fn into(self) -> PalmError {
        PalmError {
            msg: self.description().to_string(),
            kind: self,
        }
    }
}

/// Represents an error that can occur within the Palm application, including its kind and a message.
///
/// # Example
/// ```rust
/// use palm::error::{PalmError, PalmErrorKind};
/// let error: PalmError = PalmErrorKind::NotEnoughMemory.into();
/// ```
#[derive(Debug)]
pub struct PalmError {
    kind: PalmErrorKind,
    msg: String,
}

impl PalmError {
    /// Creates a new `PalmError` with an additional message.
    ///
    /// # Arguments
    ///
    /// * `msg` - A string slice that contains the additional message to be associated with the error.
    ///
    /// # Returns
    ///
    /// The updated `PalmError` instance with the new message.
    pub fn with_msg(mut self, msg: &str) -> Self {
        self.msg = msg.to_string();
        self
    }

    /// Returns a reference to the kind of the error.
    ///
    /// # Returns
    ///
    /// A reference to the `PalmErrorKind` associated with the error.
    pub fn kind(&self) -> &PalmErrorKind {
        &self.kind
    }

    /// Returns a reference to the error message.
    ///
    /// # Returns
    ///
    /// A reference to the message string associated with the error.
    pub fn msg(&self) -> &String {
        &self.msg
    }
}

impl std::fmt::Display for PalmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for PalmError {}
