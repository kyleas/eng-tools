use std::fmt;

#[derive(Debug, Clone)]
pub enum UnitError {
    ParseError(String),
    UnknownUnit { unit: String, quantity: String },
    AmbiguousUnit { unit: String, reason: String },
    OutOfRange { value: f64, reason: String },
    UnsupportedDimension { dimension: String },
}

impl fmt::Display for UnitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(msg) => write!(f, "Parse error: {msg}"),
            Self::UnknownUnit { unit, quantity } => {
                write!(f, "Unknown unit '{unit}' for {quantity}")
            }
            Self::AmbiguousUnit { unit, reason } => {
                write!(f, "Ambiguous unit '{unit}': {reason}")
            }
            Self::OutOfRange { value, reason } => {
                write!(f, "Value {value} out of range: {reason}")
            }
            Self::UnsupportedDimension { dimension } => {
                write!(f, "Unsupported dimension '{dimension}'")
            }
        }
    }
}

impl std::error::Error for UnitError {}
