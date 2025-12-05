use std::ops::Deref;

pub trait Molecule: Deref<Target = [Self::Atom]> {
    type Atom: PartialEq;
}
