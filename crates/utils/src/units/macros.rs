/**
 * -----------------------------------------------------------------------------
 * This file includes code derived from the project typst/typst by @typst.
 * Project Repository: https://github.com/typst/typst/blob/main/crates/typst/src/util/macros.rs
 *
 * Date of Import: 03 May 2024
 * -----------------------------------------------------------------------------
 * The code included in this file is licensed under the Apache License v2,
 * as per the original project by @typst.
 * For the license text, see: https://github.com/typst/typst/blob/main/LICENSE
 * -----------------------------------------------------------------------------
 */

/// Implement the `Sub` trait based on existing `Neg` and `Add` impls.
#[macro_export]
macro_rules! sub_impl {
    ($a:ident - $b:ident -> $c:ident) => {
        impl std::ops::Sub<$b> for $a {
            type Output = $c;

            fn sub(self, other: $b) -> $c {
                self + -other
            }
        }
    };
}

/// Implement an assign trait based on an existing non-assign trait.
#[macro_export]
macro_rules! assign_impl {
    ($a:ident += $b:ident) => {
        impl std::ops::AddAssign<$b> for $a {
            fn add_assign(&mut self, other: $b) {
                *self = *self + other;
            }
        }
    };

    ($a:ident -= $b:ident) => {
        impl std::ops::SubAssign<$b> for $a {
            fn sub_assign(&mut self, other: $b) {
                *self = *self - other;
            }
        }
    };

    ($a:ident *= $b:ident) => {
        impl std::ops::MulAssign<$b> for $a {
            fn mul_assign(&mut self, other: $b) {
                *self = *self * other;
            }
        }
    };

    ($a:ident /= $b:ident) => {
        impl std::ops::DivAssign<$b> for $a {
            fn div_assign(&mut self, other: $b) {
                *self = *self / other;
            }
        }
    };
}
