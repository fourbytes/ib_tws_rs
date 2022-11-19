use std::error;
use std::fmt;

#[derive(Debug)]
pub enum EncodeError {
    VersionLessError(i32),
    NeedExtraAuth,
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EncodeError::VersionLessError(version) => {
                write!(f, "Tws version less than {}", version)
            }
            EncodeError::NeedExtraAuth => write!(f, "NeedExtraAuth"),
        }
    }
}

impl error::Error for EncodeError {
    fn description(&self) -> &str {
        "Tws encode error"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}
