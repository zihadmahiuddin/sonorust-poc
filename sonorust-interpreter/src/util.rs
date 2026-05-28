use num_traits::{Bounded, FromPrimitive, PrimInt, ToPrimitive};

pub(crate) fn f64_to_int_checked<T>(n: f64) -> Option<T>
where
    T: PrimInt + Bounded + ToPrimitive + FromPrimitive,
{
    if n.fract() != 0.0 {
        return None;
    }

    let min = T::min_value().to_f64()?;
    let max = T::max_value().to_f64()?;

    if n < min || n > max {
        return None;
    }

    T::from_f64(n)
}
