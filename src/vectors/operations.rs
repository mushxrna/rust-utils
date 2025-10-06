use crate::{
    NumericCollectionType, NumericType,
    vectors::{Vec2, Vec3, Vec4},
};

macro_rules! impl_add {
    ($VecType:ident { $($field:ident: $idx:expr),+ }) => {
        impl<T: NumericType, U: NumericCollectionType<T>> std::ops::Add<U> for $VecType<T> {
            type Output = Self;

            fn add(self, other: U) -> Self {
                Self {
                    $($field: self.$field + other.get($idx).unwrap()),+
                }
            }
        }
    };
}

macro_rules! impl_sub{
    ($VecType:ident { $($field:ident: $idx:expr),+ }) => {
        impl<T: NumericType, U: NumericCollectionType<T>> std::ops::Sub<U> for $VecType<T> {
            type Output = Self;

            fn sub(self, other: U) -> Self {
                Self {
                    $($field: self.$field - other.get($idx).unwrap()),+
                }
            }
        }
    };
}

macro_rules! impl_mul{
    ($VecType:ident { $($field:ident: $idx:expr),+ }) => {
        impl<T: NumericType, U: NumericCollectionType<T>> std::ops::Mul<U> for $VecType<T> {
            type Output = Self;

            fn mul(self, other: U) -> Self {
                Self {
                    $($field: self.$field * other.get($idx).unwrap()),+
                }
            }
        }
    };
}

macro_rules! impl_div{
    ($VecType:ident { $($field:ident: $idx:expr),+ }) => {
        impl<T: NumericType, U: NumericCollectionType<T>> std::ops::Div<U> for $VecType<T> {
            type Output = Self;

            fn div(self, other: U) -> Self {
                Self {
                    $($field: self.$field / other.get($idx).unwrap()),+
                }
            }
        }
    };
}

macro_rules! impl_length {
    ($t:ident {$($field:ident),+}) => {
        impl<T: NumericType> $t<T> {
            pub fn length(&self) -> f32 {
                (0.0 $(+ (self.$field * self.$field).to_f32())+).sqrt()
            }
        }
    };
}

macro_rules! impl_normalize {
    ($VecType:ident {$($field:ident), +}) => {
        impl<T: NumericType> $VecType<T> {
            pub fn normalize(&self) -> Self {
                let len = T::from_f32(self.length());
                Self{
                    $($field: (self.$field / len)),+
                }
            }
        }
    }
}

impl_add!(Vec2 { x: 0, y: 1 });
impl_add!(Vec3 { x: 0, y: 1, z: 2 });
impl_add!(Vec4 {
    x: 0,
    y: 1,
    z: 2,
    w: 3
});

impl_sub!(Vec2 { x: 0, y: 1 });
impl_sub!(Vec3 { x: 0, y: 1, z: 2 });
impl_sub!(Vec4 {
    x: 0,
    y: 1,
    z: 2,
    w: 3
});

impl_mul!(Vec2 { x: 0, y: 1 });
impl_mul!(Vec3 { x: 0, y: 1, z: 2 });
impl_mul!(Vec4 {
    x: 0,
    y: 1,
    z: 2,
    w: 3
});

impl_div!(Vec2 { x: 0, y: 1 });
impl_div!(Vec3 { x: 0, y: 1, z: 2 });
impl_div!(Vec4 {
    x: 0,
    y: 1,
    z: 2,
    w: 3
});

impl_length!(Vec2 { x, y });
impl_length!(Vec3 { x, y, z });
impl_length!(Vec4 { x, y, z, w });

impl_normalize!(Vec2 { x, y });
impl_normalize!(Vec3 { x, y, z });
impl_normalize!(Vec4 { x, y, z, w });
