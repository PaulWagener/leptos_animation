use std::ops::{Add, Mul, Sub};

/// Default linear tween between any time of number
pub fn default<T, I>() -> fn(&T, &T, f64) -> I
where
    T: Copy,
    T: Sub<T, Output = I>,
    I: Mul<f64, Output = I>,
    I: Add<T, Output = I>,
{
    |from, to, progress| (*to - *from) * progress + *from
}
