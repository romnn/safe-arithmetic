use crate::error::{DivideByZero, Overflow, Underflow};
use num::Zero;
use std::fmt::{self, Debug, Display};

pub trait CheckedDiv<Rhs = Self>
where
    Self: Sized,
{
    type Output;
    type Error;

    /// Checked arithmetic division of self
    ///
    /// # Errors
    /// When the result of the division can not be represented (e.g. due to an overflow).
    fn checked_div(self, scalar: Rhs) -> Result<Self::Output, Self::Error>;
}

macro_rules! impl_unsigned_checked_div {
    ( $T:ty ) => {
        impl CheckedDiv for $T {
            type Output = Self;
            type Error = DivError<Self, Self>;

            fn checked_div(self, rhs: Self) -> Result<Self::Output, Self::Error> {
                // can fail if rhs == 0
                if rhs.is_zero() {
                    Err(DivError(self.divide_by_zero()))
                } else {
                    num::CheckedDiv::checked_div(&self, &rhs)
                        .ok_or(rhs.underflows(self))
                        .map_err(DivError)
                }
            }
        }
    };
}

impl_unsigned_checked_div!(u32);

macro_rules! impl_signed_checked_div {
    ( $T:ty ) => {
        impl CheckedDiv for $T {
            type Output = Self;
            type Error = DivError<Self, Self>;

            fn checked_div(self, rhs: Self) -> Result<Self::Output, Self::Error> {
                // can fail if rhs == 0
                if rhs.is_zero() {
                    Err(DivError(self.divide_by_zero()))
                } else if self.signum() == rhs.signum() {
                    // can also overflow
                    num::CheckedDiv::checked_div(&self, &rhs)
                        .ok_or(rhs.overflows(self))
                        .map_err(DivError)
                } else {
                    // can also underflow?
                    num::CheckedDiv::checked_div(&self, &rhs)
                        .ok_or(rhs.underflows(self))
                        .map_err(DivError)
                }
            }
        }
    };
}

impl_signed_checked_div!(i64);

macro_rules! impl_float_checked_div {
    ( $T:ty ) => {
        impl CheckedDiv for $T {
            type Output = Self;
            type Error = DivError<Self, Self>;

            fn checked_div(self, rhs: Self) -> Result<Self::Output, Self::Error> {
                // can fail if rhs == 0
                if rhs.is_zero() {
                    return Err(DivError(self.divide_by_zero()));
                }
                let result = self / rhs;
                if result.is_nan() && self.signum() == rhs.signum() {
                    // can also overflow
                    Err(DivError(rhs.overflows(self)))
                } else if result.is_nan() {
                    // can also underflow?
                    Err(DivError(rhs.underflows(self)))
                } else {
                    Ok(result)
                }
            }
        }
    };
}

impl_float_checked_div!(f64);

#[derive(PartialEq, Clone, Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct DivError<Lhs, Rhs>(pub crate::error::Operation<Lhs, Rhs>);

impl<Lhs, Rhs> crate::error::Arithmetic for DivError<Lhs, Rhs>
where
    Lhs: crate::Type,
    Rhs: crate::Type,
{
}

impl<Lhs, Rhs> std::error::Error for DivError<Lhs, Rhs>
where
    Lhs: Display + Debug,
    Rhs: Display + Debug,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.cause.as_deref().map(crate::error::AsErr::as_err)
    }
}

impl<Lhs, Rhs> Display for DivError<Lhs, Rhs>
where
    Lhs: Display,
    Rhs: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0.kind {
            Some(kind) => match kind {
                crate::error::Kind::DivideByZero => {
                    write!(f, "dividing {} by {} is undefined", self.0.lhs, self.0.rhs)
                }
                other => {
                    write!(
                        f,
                        "dividing {} by {} would {} {}",
                        self.0.lhs,
                        self.0.rhs,
                        other,
                        std::any::type_name::<Lhs>(),
                    )
                }
            },
            None => write!(f, "cannot divide {} by {}", self.0.lhs, self.0.rhs),
        }
    }
}
