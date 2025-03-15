#![warn(missing_docs)]

pub mod cast;
pub mod clamp;
pub mod error;
pub mod ops;
pub mod round;

pub use cast::{Cast, CastError};
pub use clamp::{Clamp, ClampMin};
pub use error::Error;
pub use round::{Ceil, Floor, Round, RoundingMode};
use std::fmt::{Debug, Display};

pub trait Type: Sized + Display + Debug + Clone + PartialEq + Send + Sync + 'static {}

impl<T> Type for T where T: num::Num + Debug + Display + Clone + PartialEq + Send + Sync + 'static {}
