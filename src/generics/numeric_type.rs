pub trait NumericType:
    Sized
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + Clone
    + Copy
{
}

impl NumericType for i32 {}
impl NumericType for i64 {}
impl NumericType for f32 {}
impl NumericType for f64 {}
impl NumericType for u32 {}
impl NumericType for u64 {}
