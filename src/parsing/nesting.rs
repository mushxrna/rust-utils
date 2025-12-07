use crate::datastructures::IndexNode;
use crate::parsing::Molecule;
use std::{
    clone,
    ops::{Deref, Index, Range},
};
//
// ENUMS AND STRUCTS
//
pub enum NestedObject<A> {
    Atom(A),
    Molecule(Vec<NestedObject<A>>),
}
pub struct Nester<A: Molecule> {
    pub delimiters: (A, A),
}
//
// IMPL METHODS
//

impl<A: Molecule + Clone> Nester<A> {
    pub fn new(delimiters: (A, A)) -> Nester<A> {
        Nester { delimiters }
    }
    pub fn nest_into_tree(&self, source: &[impl AsRef<[A::Atom]>], base: usize) -> IndexNode<A> {
        let mut index = 0;
        let mut node_pool = vec![];

        while index < source.len() {
            let i = source[index].as_ref();

            if i == &*self.delimiters.0 {
                let mut delims_found = 0;
                let mut dist_to_match = 0;

                for j in source[index..].iter().map(|x| x.as_ref()) {
                    (j == &*self.delimiters.0).then(|| delims_found += 1);
                    (j == &*self.delimiters.1).then(|| delims_found -= 1);

                    if delims_found == 0 {
                        break;
                    }

                    dist_to_match += 1;
                }

                let exc_range = (index + 1)..(index + dist_to_match);
                let evaluated_node =
                    self.nest_into_tree(&source[exc_range.clone()], base + exc_range.start);
                node_pool.push(evaluated_node);
                index += dist_to_match;
            } else if i != &*self.delimiters.1 {
                node_pool.push(IndexNode::new(Some(base + index), None));
            }

            index += 1;
        }
        IndexNode::new(None, Some(node_pool))
    }
}
//
// IMPL DISPLAY
//
impl<T: std::fmt::Display> std::fmt::Display for NestedObject<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            NestedObject::Atom(value) => write!(f, "Atom: {value}"),
            NestedObject::Molecule(vec) => {
                let mut s = String::new();
                for i in vec {
                    s.push_str(&format!("{i} "))
                }
                write!(f, "Molecule: ( {s} )")
            }
        }
    }
}
impl<A: Clone> std::fmt::Display for IndexNode<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self.children() {
            Some(c) => write!(f, "Parent Node! Owner of {} nodes.", c.len()),
            None => write!(f, "Child Node!"),
        }
    }
}
