use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub enum NodeError {
    NotFound,
    IsNull,
}

impl Error for NodeError {}

impl fmt::Display for NodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "entity not found"),
            Self::IsNull => write!(f, "entity is null"),
        }
    }
}
