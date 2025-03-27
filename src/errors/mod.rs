use std::fmt;
use std::io;
use std::path::PathBuf;
use std::process::ExitStatus;

/// Represents all possible errors that can occur in Lion CLI
#[derive(Debug)]
#[allow(dead_code)]
pub enum LionError {
    /// Unsupported or unknown file type
    UnsupportedFileType(String),

    /// Error when creating a file or directory
    FileSystemError { path: PathBuf, source: io::Error },

    /// Error when reading from or writing to a file
    FileIOError {
        path: PathBuf,
        operation: String,
        source: io::Error,
    },

    /// Error when parsing configuration
    ConfigError {
        message: String,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Error when executing a command
    CommandError {
        command: String,
        args: Vec<String>,
        exit_status: Option<ExitStatus>,
        source: io::Error,
    },

    /// Error when a required dependency is missing
    MissingDependency { dependency: String, message: String },

    /// Error when a command is not supported for a specific language
    UnsupportedCommand { command: String, language: String },

    /// Invalid argument or parameter provided
    InvalidArgument { argument: String, message: String },

    /// Custom error with a message
    Custom(String),
}
impl fmt::Display for LionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LionError::UnsupportedFileType(ext) => {
                write!(f, "Unsupported file type: {}", ext)
            }
            LionError::FileSystemError { path, source } => {
                write!(
                    f,
                    "File system error for path '{}': {}",
                    path.display(),
                    source
                )
            }
            LionError::FileIOError {
                path,
                operation,
                source,
            } => {
                write!(
                    f,
                    "Error {} file '{}': {}",
                    operation,
                    path.display(),
                    source
                )
            }
            LionError::ConfigError { message, source } => {
                if let Some(err) = source {
                    write!(f, "Configuration error: {}. Cause: {}", message, err)
                } else {
                    write!(f, "Configuration error: {}", message)
                }
            }
            LionError::CommandError {
                command,
                args,
                exit_status,
                source,
            } => {
                write!(
                    f,
                    "Error executing command '{}' with args {:?}: {}",
                    command,
                    args,
                    if let Some(status) = exit_status {
                        format!("exited with status {}", status)
                    } else {
                        source.to_string()
                    }
                )
            }
            LionError::MissingDependency {
                dependency,
                message,
            } => {
                write!(f, "Missing dependency '{}': {}", dependency, message)
            }
            LionError::UnsupportedCommand { command, language } => {
                write!(
                    f,
                    "Command '{}' is not supported for {} files",
                    command, language
                )
            }
            LionError::InvalidArgument { argument, message } => {
                write!(f, "Invalid argument '{}': {}", argument, message)
            }
            LionError::Custom(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl std::error::Error for LionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LionError::FileSystemError { source, .. } => Some(source),
            LionError::FileIOError { source, .. } => Some(source),
            LionError::CommandError { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Common result type for all Lion CLI operations
pub type Result<T> = std::result::Result<T, LionError>;

/// Helper functions for common error conversions
impl From<io::Error> for LionError {
    fn from(err: io::Error) -> Self {
        LionError::Custom(format!("I/O error: {}", err))
    }
}

/// Helper method to construct a file system error
#[allow(dead_code)]
pub fn file_system_error(path: impl Into<PathBuf>, err: io::Error) -> LionError {
    LionError::FileSystemError {
        path: path.into(),
        source: err,
    }
}

/// Helper method to construct a file I/O error
pub fn file_io_error(
    path: impl Into<PathBuf>,
    operation: impl Into<String>,
    err: io::Error,
) -> LionError {
    LionError::FileIOError {
        path: path.into(),
        operation: operation.into(),
        source: err,
    }
}

/// Helper method to construct a command error
pub fn command_error(
    command: impl Into<String>,
    args: Vec<String>,
    exit_status: Option<ExitStatus>,
    err: io::Error,
) -> LionError {
    LionError::CommandError {
        command: command.into(),
        args,
        exit_status,
        source: err,
    }
}
