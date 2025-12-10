use std::ops::Deref;

pub trait Molecule: Deref<Target: for<'a> PartialEq> {
    type Atom: PartialEq;
    type Of<'a>;
}
