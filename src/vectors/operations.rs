use crate::{
    NumericType,
    vectors::{Vec2, Vec3, Vec4},
};

macro_rules! impl_add {
    ($VecType:ident { $($field:ident),+ }) => {
        impl<T: NumericType> std::ops::Add for $VecType<T> {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self {
                    $($field: self.$field + other.$field),+
                }
            }
        }
    };
}

macro_rules! impl_sub{
    ($VecType:ident { $($field:ident),+ }) => {
        impl<T: NumericType> std::ops::Sub for $VecType<T> {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self {
                    $($field: self.$field - other.$field),+
                }
            }
        }
    };
}

macro_rules! impl_mul{
    ($VecType:ident { $($field:ident),+ }) => {
        impl<T: NumericType> std::ops::Mul for $VecType<T> {
            type Output = Self;

            fn mul(self, other: Self) -> Self {
                Self {
                    $($field: self.$field * other.$field),+
                }
            }
        }
    };
}

macro_rules! impl_div{
    ($VecType:ident { $($field:ident),+ }) => {
        impl<T: NumericType> std::ops::Div for $VecType<T> {
            type Output = Self;

            fn div(self, other: Self) -> Self {
                Self {
                    $($field: self.$field / other.$field),+
                }
            }
        }
    };
}

impl_add!(Vec2 { x, y });
impl_add!(Vec3 { x, y, z });
impl_add!(Vec4 { x, y, z, w });

impl_sub!(Vec2 { x, y });
impl_sub!(Vec3 { x, y, z });
impl_sub!(Vec4 { x, y, z, w });

impl_mul!(Vec2 { x, y });
impl_mul!(Vec3 { x, y, z });
impl_mul!(Vec4 { x, y, z, w });

impl_div!(Vec2 { x, y });
impl_div!(Vec3 { x, y, z });
impl_div!(Vec4 { x, y, z, w });
