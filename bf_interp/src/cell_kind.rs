//! Common behavior for a Brainfuck [crate::VM] cell.

use std::cmp::PartialOrd;
use std::ops::{AddAssign, SubAssign};

/// Each [crate::VM] cell is of generic type T, which must implement this [CellKind] trait, otherwise the virtual machine
/// would be meaningless.
pub trait CellKind: AddAssign + SubAssign + PartialOrd + Sized + Clone {
    /// Zero represented as T type.
    fn zero() -> Self;

    /// One represented as T type.
    fn one() -> Self;

    /// Max value which T can represent.
    fn max() -> Self;

    /// Min value which T can represent.
    fn min() -> Self;

    /// Set a given u8 value to a T type variable.
    fn set_value(&mut self, value: u8);

    /// Get the underlying data as u8.
    fn get_value(&self) -> u8;

    /// Increment a T type value by one. The result is wrapped to be less than or equal to T type max value.
    fn increment(&mut self) {
        if self < &mut CellKind::max() {
            *self += CellKind::one();
        } else {
            *self = CellKind::min();
        }
    }

    /// Decrement a T type value by one. The result is wrapped to be greater than or equal to T min value.
    fn decrement(&mut self) {
        if self > &mut CellKind::min() {
            *self -= CellKind::one();
        } else {
            *self = CellKind::max();
        }
    }
}

impl CellKind for u8 {
    fn zero() -> Self {
        0_u8
    }

    fn one() -> Self {
        1_u8
    }

    fn max() -> Self {
        255_u8
    }

    fn min() -> Self {
        0_u8
    }

    fn set_value(&mut self, value: u8) {
        *self = value;
    }

    fn get_value(&self) -> u8 {
        *self
    }
}
