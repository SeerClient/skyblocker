//! Ported from io.github.moulberry.repo.NEURepositoryException (neurepoparser 1.12.0).

/// java: public class NEURepositoryException extends Exception
///
/// The Java exception's message and cause chain; `suppressed` is `Throwable.addSuppressed`.
#[derive(Clone, Debug)]
pub struct NEURepositoryException {
    message: String,
    cause: Option<String>,
    suppressed: Vec<NEURepositoryException>,
}

impl NEURepositoryException {
    /// java: public NEURepositoryException(String path, String message, Throwable cause)
    pub fn new(path: &str, message: &str, cause: Option<String>) -> Self {
        // java: super("NEU Repository | Failure loading " + path + ": " + message, cause);
        NEURepositoryException { message: format!("NEU Repository | Failure loading {}: {}", path, message), cause, suppressed: Vec::new() }
    }

    /// java: Throwable.getMessage()
    pub fn get_message(&self) -> &str {
        &self.message
    }

    /// java: Throwable.getCause() (its message)
    pub fn get_cause(&self) -> Option<&str> {
        self.cause.as_deref()
    }

    /// java: Throwable.addSuppressed(Throwable)
    pub fn add_suppressed(&mut self, exception: NEURepositoryException) {
        self.suppressed.push(exception);
    }

    /// java: Throwable.getSuppressed()
    pub fn get_suppressed(&self) -> &[NEURepositoryException] {
        &self.suppressed
    }
}

impl std::fmt::Display for NEURepositoryException {
    /// java: Throwable.toString() - the class name and the message, then (as a stack trace prints them) the cause
    /// and every suppressed exception.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "io.github.moulberry.repo.NEURepositoryException: {}", self.message)?;
        if let Some(cause) = &self.cause {
            write!(f, "\nCaused by: {}", cause)?;
        }
        for suppressed in &self.suppressed {
            write!(f, "\n\tSuppressed: {}", suppressed)?;
        }
        Ok(())
    }
}

impl std::error::Error for NEURepositoryException {}
