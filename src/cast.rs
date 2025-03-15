use std::marker::PhantomData;

pub trait Cast
where
    Self: Sized + num::ToPrimitive + Copy,
{
    /// Checked cast of self to `Target`.
    ///
    /// # Errors
    /// When `self` can not be casted to `Target`.
    fn cast<Target>(self) -> Result<Target, CastError<Self, Target>>
    where
        Target: num::NumCast;
}

impl<Src> Cast for Src
where
    Self: Sized + num::ToPrimitive + Copy,
{
    fn cast<Target>(self) -> Result<Target, CastError<Self, Target>>
    where
        Target: num::NumCast,
    {
        num::NumCast::from(self).ok_or(CastError {
            src: self,
            target: PhantomData,
            cause: None,
        })
    }
}

#[derive(PartialEq, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct CastError<Src, Target> {
    pub src: Src,
    pub target: PhantomData<Target>,
    pub cause: Option<crate::error::Error>,
}

impl<Src, Target> crate::error::Arithmetic for CastError<Src, Target>
where
    Src: crate::Type,
    Target: crate::Type,
{
}

impl<Src, Target> std::error::Error for CastError<Src, Target>
where
    Src: std::fmt::Debug + std::fmt::Display,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.cause.as_deref().map(crate::error::AsErr::as_err)
    }
}

impl<Src, Target> std::fmt::Debug for CastError<Src, Target>
where
    Src: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CastError")
            .field("src", &self.src)
            .field("target", &std::any::type_name::<Target>())
            .field("cause", &self.cause)
            .finish()
    }
}

impl<Src, Target> std::fmt::Display for CastError<Src, Target>
where
    Src: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "cannot cast {} of type {} to {}",
            self.src,
            std::any::type_name::<Src>(),
            std::any::type_name::<Target>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use similar_asserts::assert_eq as sim_assert_eq;

    #[test]
    fn invalid_num_cast() {
        sim_assert_eq!(
            &42_000f64.cast::<i8>().err().unwrap().to_string(),
            "cannot cast 42000 of type f64 to i8"
        );
        sim_assert_eq!(
            &(-42f64).cast::<u32>().err().unwrap().to_string(),
            "cannot cast -42 of type f64 to u32"
        );
        sim_assert_eq!(
            &(-42i64).cast::<u32>().err().unwrap().to_string(),
            "cannot cast -42 of type i64 to u32"
        );
        let value = i64::MAX;
        sim_assert_eq!(
            &value.cast::<u32>().err().unwrap().to_string(),
            &format!("cannot cast {} of type i64 to u32", &value)
        );
        let value = i64::MIN;
        sim_assert_eq!(
            &value.cast::<u64>().err().unwrap().to_string(),
            &format!("cannot cast {} of type i64 to u64", &value)
        );
    }

    #[test]
    fn valid_num_cast() {
        sim_assert_eq!(42f64.cast::<f32>().ok(), Some(42f32));
        sim_assert_eq!(42f32.cast::<f64>().ok(), Some(42f64));
        sim_assert_eq!(42u64.cast::<f32>().ok(), Some(42f32));
        sim_assert_eq!(42i64.cast::<f32>().ok(), Some(42f32));
        sim_assert_eq!(42.1f64.cast::<i8>().ok(), Some(42i8));
        sim_assert_eq!(42.6f64.cast::<i8>().ok(), Some(42i8));
        assert!(u32::MAX.cast::<i64>().is_ok());
        assert!(i64::MAX.cast::<u64>().is_ok());
        assert!(i128::MAX.cast::<f64>().is_ok());
        assert!(u128::MAX.cast::<f64>().is_ok());
        sim_assert_eq!(f32::MAX.cast::<u32>().ok(), None);

        assert_abs_diff_eq!(
            u32::MAX.cast::<f32>().unwrap(),
            2f32.powi(32),
            epsilon = 2.0
        );
        assert_abs_diff_eq!(
            u32::MAX.cast::<f64>().unwrap(),
            2f64.powi(32),
            epsilon = 2.0
        );
    }
}
