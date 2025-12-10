use std::ops::Deref;

use crate::parsing::rules::TypeConstructor;

pub trait Molecule: Deref<Target: for<'a> PartialEq> {
    type Atom: PartialEq;
    type Of<'a>;
}

impl<A: Molecule> TypeConstructor for A {
    type Of<'a> = A::Of<'a>;
}
