use serde_json::error::Category;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct InputError {
    description: String,
}

impl From<String> for InputError {
    fn from(description: String) -> InputError {
        InputError { description }
    }
}

impl From<&str> for InputError {
    fn from(description: &str) -> InputError {
        InputError {
            description: description.to_string(),
        }
    }
}

impl Error for InputError {
    fn description(&self) -> &str {
        self.description.as_str()
    }
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum CliError {
    Git(git2::Error),
    IO(std::io::Error),
    Input(InputError),
}

impl From<git2::Error> for CliError {
    fn from(err: git2::Error) -> CliError {
        CliError::Git(err)
    }
}

impl From<serde_json::Error> for CliError {
    fn from(error: serde_json::Error) -> CliError {
        let description = match error.classify() {
            Category::Io => format!("An IO error occurred: {}", error.description()),
            Category::Syntax => format!("A syntax error occurred: {}", error.description()),
            Category::Data => format!("A data error occurred: {}", error.description()),
            Category::Eof => format!("Unexpected EOF. {}", error.description()),
        };
        CliError::Input(InputError::from(description))
    }
}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> CliError {
        CliError::IO(err)
    }
}

impl From<InputError> for CliError {
    fn from(err: InputError) -> CliError {
        CliError::Input(err)
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::Git(ref err) => err.fmt(f),
            CliError::IO(ref err) => err.fmt(f),
            CliError::Input(ref err) => err.fmt(f),
        }
    }
}

impl Error for CliError {
    fn description(&self) -> &str {
        match *self {
            CliError::Git(ref err) => err.description(),
            CliError::IO(ref err) => err.description(),
            CliError::Input(ref err) => err.description(),
        }
    }
}
