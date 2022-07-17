#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id {
    addr: usize,
}

impl Id {
    pub const DUMMY: Id = Id { addr: 0 };

    pub fn from_boxed<T>(b: &Box<T>) -> Self {
        Self {
            addr: &**b as *const T as usize,
        }
    }
}
