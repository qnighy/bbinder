use std::collections::HashMap;
use crate::Id;

pub trait CloneWith {
    fn clone_with(&self, ids: &mut HashMap<Id, Id>) -> Self;
}

impl<T: CloneWith> CloneWith for Box<T> {
    fn clone_with(&self, ids: &mut HashMap<Id, Id>) -> Self {
        Box::new(T::clone_with(&**self, ids))
    }
}

impl<T: CloneWith> CloneWith for Option<T> {
    fn clone_with(&self, ids: &mut HashMap<Id, Id>) -> Self {
        self.as_ref().map(|x| x.clone_with(ids))
    }
}

impl<T: CloneWith> CloneWith for Vec<T> {
    fn clone_with(&self, ids: &mut HashMap<Id, Id>) -> Self {
        self.iter().map(|x| x.clone_with(ids)).collect()
    }
}
