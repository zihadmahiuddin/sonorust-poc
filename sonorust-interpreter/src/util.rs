use num_traits::FromPrimitive;

pub(crate) fn int_from_f64_checked<T>(n: f64) -> Option<T>
where
    T: FromPrimitive,
{
    if n.fract() != 0.0 {
        return None;
    }
    T::from_f64(n)
}
