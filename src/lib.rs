use std::borrow::Cow;
use std::collections::HashMap;
use std::mem;

pub use crate::id::Id;
pub use crate::clone_with::CloneWith;
pub use crate::cmp_with::{EqWith, PartialEqWith};

mod id;
mod clone_with;
mod cmp_with;

#[derive(Debug)]
pub enum Expr {
    Var(VarExpr),
    Abs(AbsExpr),
    App(AppExpr),
}

impl CloneWith for Expr {
    fn clone_with(&self, ids: &mut HashMap<Id, Id>) -> Self {
        match self {
            Expr::Var(x) => Expr::Var(x.clone_with(ids)),
            Expr::Abs(x) => Expr::Abs(x.clone_with(ids)),
            Expr::App(x) => Expr::App(x.clone_with(ids)),
        }
    }
}

impl Clone for Expr {
    fn clone(&self) -> Self {
        self.clone_with(&mut HashMap::new())
    }
}

impl PartialEqWith for Expr {
    fn eq_with(&self, other: &Expr, ids: &mut HashMap<Id, Id>) -> bool {
        match (self, other) {
            (Expr::Var(x), Expr::Var(other)) => x.eq_with(other, ids),
            (Expr::Abs(x), Expr::Abs(other)) => x.eq_with(other, ids),
            (Expr::App(x), Expr::App(other)) => x.eq_with(other, ids),
            _ => false,
        }
    }
}
impl EqWith for Expr {}

impl Expr {
    pub const DUMMY: Expr = Expr::Var(VarExpr::DUMMY);
}

#[derive(Debug)]
pub struct VarExpr {
    pub id: Id,
}

impl CloneWith for VarExpr {
    fn clone_with(&self, ids: &mut HashMap<Id, Id>) -> Self {
        Self { id: *ids.get(&self.id).unwrap_or(&self.id) }
    }
}

impl Clone for VarExpr {
    fn clone(&self) -> Self {
        self.clone_with(&mut HashMap::new())
    }
}

impl PartialEqWith for VarExpr {
    fn eq_with(&self, other: &VarExpr, ids: &mut HashMap<Id, Id>) -> bool {
        let converted_id = *ids.get(&self.id).unwrap_or(&self.id);
        converted_id == other.id
    }
}
impl EqWith for VarExpr {}

impl VarExpr {
    pub const DUMMY: VarExpr = VarExpr { id: Id::DUMMY };
}

#[derive(Debug)]
pub struct AbsExpr {
    body: Box<Expr>,
}

impl CloneWith for AbsExpr {
    fn clone_with(&self, ids: &mut HashMap<Id, Id>) -> Self {
        let old_id = self.raw_id();
        AbsExpr::new(|new_id| {
            ids.insert(old_id, new_id);
            let ret = self.raw_body().clone_with(ids);
            ids.remove(&old_id);
            ret
        })
    }
}

impl Clone for AbsExpr {
    fn clone(&self) -> Self {
        self.clone_with(&mut HashMap::new())
    }
}

impl PartialEqWith for AbsExpr {
    fn eq_with(&self, other: &AbsExpr, ids: &mut HashMap<Id, Id>) -> bool {
        ids.insert(self.raw_id(), other.raw_id());
        let ret = self.raw_body().eq_with(other.raw_body(), ids);
        ids.remove(&self.raw_id());
        ret
    }
}
impl EqWith for AbsExpr {}

impl AbsExpr {
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce(Id) -> Expr
    {
        let mut ret = AbsExpr {
            body: Box::new(Expr::DUMMY),
        };
        let id = ret.raw_id();
        *ret.body = f(id);
        ret
    }

    pub fn raw_id(&self) -> Id {
        Id::from_boxed::<Expr>(&self.body)
    }
    pub fn raw_body(&self) -> &Expr {
        &self.body
    }
    pub fn raw_body_mut(&mut self) -> &mut Expr {
        &mut self.body
    }
}

#[derive(Debug)]
pub struct AppExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

impl CloneWith for AppExpr {
    fn clone_with(&self, ids: &mut HashMap<Id, Id>) -> Self {
        Self {
            lhs: self.lhs.clone_with(ids),
            rhs: self.rhs.clone_with(ids),
        }
    }
}

impl Clone for AppExpr {
    fn clone(&self) -> Self {
        self.clone_with(&mut HashMap::new())
    }
}

impl PartialEqWith for AppExpr {
    fn eq_with(&self, other: &AppExpr, ids: &mut HashMap<Id, Id>) -> bool {
        self.lhs.eq_with(&other.lhs, ids) && self.rhs.eq_with(&other.rhs, ids)
    }
}

impl Expr {
    pub fn subst<'a>(&'a mut self, id: Id, r: &mut Cow<'a, Expr>) {
        match self {
            Expr::Var(x) if x.id == id => {
                match r {
                    Cow::Borrowed(r) => {
                        *self = r.clone();
                    }
                    Cow::Owned(r_) => {
                        *self = mem::replace(r_, Expr::DUMMY);
                        *r = Cow::Borrowed(&*self);
                    }
                }
            }
            Expr::Var(_) => {}
            Expr::Abs(x) => {
                x.raw_body_mut().subst(id, r);
            }
            Expr::App(x) => {
                x.lhs.subst(id, r);
                x.rhs.subst(id, r);
            }
        }
    }
}
