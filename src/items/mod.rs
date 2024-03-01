use std::{
    fmt,
    ops::{AddAssign, SubAssign},
};

use ordered_float::OrderedFloat;

#[derive(Default, Debug, Clone, Copy, PartialEq, Hash)]
pub enum MetalIngot {
    #[default]
    IronIngot,
    SilverIngot,
    GoldIngot,
}

#[derive(Default, Clone, PartialEq, PartialOrd, Hash)]
pub enum Amount {
    #[default]
    None,
    Weight(OrderedFloat<f32>), // Need ordred float
    Quantity(u32),
}

impl fmt::Debug for Amount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Weight(arg0) => {
                write!(f, "{} Kgs", arg0)
            }
            Self::Quantity(arg0) => {
                write!(f, "x{}", arg0)
            }
            _ => {
                write!(f, "None")
            }
        }
    }
}

impl AddAssign for Amount {
    fn add_assign(&mut self, rhs: Self) {
        match self {
            Amount::Weight(weight) => match rhs {
                Amount::Weight(w) => *weight += w,
                _ => {}
            },
            Amount::Quantity(quantity) => match rhs {
                Amount::Quantity(q) => *quantity += q,
                _ => {}
            },
            Amount::None => {}
        }
    }
}

impl SubAssign for Amount {
    fn sub_assign(&mut self, rhs: Self) {
        match self {
            Amount::Weight(weight) => match rhs {
                Amount::Weight(w) => *weight -= w,
                _ => {}
            },
            Amount::Quantity(quantity) => match rhs {
                Amount::Quantity(q) => *quantity -= q,
                _ => {}
            },
            Amount::None => {}
        }
    }
}
