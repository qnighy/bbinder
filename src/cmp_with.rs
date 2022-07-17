use std::collections::HashMap;
use crate::Id;

pub trait PartialEqWith<Rhs = Self>
where
    Rhs: ?Sized,
{
    fn eq_with(&self, other: &Rhs, ids: &mut HashMap<Id, Id>) -> bool;
    fn ne_with(&self, other: &Rhs, ids: &mut HashMap<Id, Id>) -> bool {
        !Self::eq_with(self, other, ids)
    }
}

pub trait EqWith : PartialEqWith<Self> {}

impl<T: PartialEqWith<T> + ?Sized> PartialEqWith<Box<T>> for Box<T> {
    fn eq_with(&self, other: &Box<T>, ids: &mut HashMap<Id, Id>) -> bool {
        T::eq_with(&*self, &*other, ids)
    }

    fn ne_with(&self, other: &Box<T>, ids: &mut HashMap<Id, Id>) -> bool {
        T::ne_with(&*self, &*other, ids)
    }
}
impl<T: EqWith + ?Sized> EqWith for Box<T> {}

impl<T: PartialEqWith> PartialEqWith for Option<T> {
    fn eq_with(&self, other: &Option<T>, ids: &mut HashMap<Id, Id>) -> bool {
        match (self, other) {
            (Some(x), Some(other)) => T::eq_with(x, other, ids),
            (None, None) => true,
            _ => false
        }
    }
}
impl<T: EqWith> EqWith for Option<T> {}

impl<T: PartialEqWith> PartialEqWith for Vec<T> {
    fn eq_with(&self, other: &Vec<T>, ids: &mut HashMap<Id, Id>) -> bool {
        self.len() == other.len() && self.iter().zip(other).all(|(x, other)| T::eq_with(x, other, ids))
    }
}
impl<T: EqWith> EqWith for Vec<T> {}


