use std::ops::Deref;

pub trait Molecule: Deref<Target: for<'a> PartialEq<&'a [Self::Atom]>> {
    type Atom: PartialEq;
}
