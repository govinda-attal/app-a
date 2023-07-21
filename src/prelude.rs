pub use crate::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

// Generic Wrapper Tuple Struct for new type pattern
// to implement external traits on external types
pub struct W<T>(pub T);

pub use std::format as f;
