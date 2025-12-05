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
#[derive(Clone)]
pub struct IndexNode {
    index: Option<usize>,             // points to an index in a SPLIT source.
    children: Option<Vec<IndexNode>>, //child nodes.
}
pub struct Nester<A: Molecule> {
    pub delimiters: (A, A),
}
//
// IMPL METHODS
//
impl IndexNode {
    pub fn get_children(&self) -> Option<&[IndexNode]> {
        if let Some(inner) = self.children.as_ref() {
            Some(inner.as_slice())
        } else {
            None
        }
    }

    pub fn get_index(&self) -> Option<usize> {
        self.index
    }

    pub fn ref_into<'a, A>(&self, source: &'a [A]) -> Result<&'a A, String> {
        match self.get_index() {
            Some(i) => Ok(&source[i]),
            None => Err(String::from("Attempted to use parent node as reference.")),
        }
    }

    pub fn get_children_or_panic(&self) -> &[IndexNode] {
        if let Some(inner) = self.children.as_ref() {
            inner.as_slice()
        } else {
            panic!("PANIC! Used get_children_or_panic on IndexNode that does not have children")
        }
    }

    pub fn get_index_or_panic(&self) -> usize {
        self.index
            .expect("PANIC! Used get_index_or_panic on IndexNode that does not have an index.")
    }

    pub fn collapse(self) -> Option<Vec<IndexNode>> {
        self.children
    }
}

impl<A: Molecule> Nester<A> {
    pub fn new(delimiters: (A, A)) -> Nester<A> {
        Nester { delimiters }
    }
    pub fn nest_into_tree(&self, source: &[impl AsRef<[A::Atom]>]) -> IndexNode {
        let mut index = 0;
        let mut node_pool = vec![];

        while index < source.len() {
            let i = source[index].as_ref();

            if i == &*self.delimiters.0 {
                let mut delims_found = 0;
                let mut dist_to_match = 0;

                for j in source[index..].iter().map(|x| x.as_ref()) {
                    if j == &*self.delimiters.0 {
                        delims_found += 1;
                    } else if j == &*self.delimiters.1 {
                        delims_found -= 1;
                    }

                    if delims_found == 0 {
                        break;
                    }

                    dist_to_match += 1;
                }

                let inc_range = index..(index + dist_to_match); //includes delimiters
                let exc_range = (index + 1)..(index + dist_to_match); //does not
                let evaluated_node = self.nest_into_tree(&source[exc_range]);
                node_pool.push(evaluated_node);
                index += dist_to_match;
            } else if i != &*self.delimiters.1 {
                node_pool.push(IndexNode {
                    index: Some(index),
                    children: None,
                });
            }

            index += 1;
        }
        IndexNode {
            index: None,
            children: Some(node_pool),
        }
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
impl std::fmt::Display for IndexNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self.get_children() {
            Some(c) => write!(f, "Parent Node! Owner of {} nodes.", c.len()),
            None => write!(f, "Child Node!"),
        }
    }
}
