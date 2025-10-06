pub trait NumericType:
    Sized
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + Clone
    + Copy
{
    fn to_f32(self) -> f32;
    fn from_f32(value: f32) -> Self;
}

impl NumericType for i32 {
    fn to_f32(self) -> f32 {
        self as f32
    }

    fn from_f32(value: f32) -> Self {
        value as Self
    }
}
impl NumericType for i64 {
    fn to_f32(self) -> f32 {
        self as f32
    }
    fn from_f32(value: f32) -> Self {
        value as Self
    }
}

impl NumericType for f32 {
    fn to_f32(self) -> f32 {
        self as f32
    }
    fn from_f32(value: f32) -> Self {
        value as Self
    }
}
impl NumericType for f64 {
    fn to_f32(self) -> f32 {
        self as f32
    }
    fn from_f32(value: f32) -> Self {
        value as Self
    }
}
impl NumericType for u32 {
    fn to_f32(self) -> f32 {
        self as f32
    }
    fn from_f32(value: f32) -> Self {
        value as Self
    }
}
impl NumericType for u64 {
    fn to_f32(self) -> f32 {
        self as f32
    }
    fn from_f32(value: f32) -> Self {
        value as Self
    }
}
