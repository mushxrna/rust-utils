use crate::parsing::Molecule;
use std::ops::{Deref, Index, Range};
//
// ENUMS AND STRUCTS
//
pub enum NestedObject<A> {
    Atom(A),
    Molecule(Vec<NestedObject<A>>),
}
pub struct IndexNode {
    index: Option<usize>,             // points to an index in a SPLIT source.
    children: Option<Vec<IndexNode>>, //child nodes.
}
pub struct IndexTree {
    nodes: Vec<IndexNode>,
    pub root: IndexNode,
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

                    dist_to_match += 1;

                    if delims_found == 0 {
                        break;
                    }
                }

                let inc_range = index..(index + dist_to_match); //includes delimiters
                let exc_range = (index + 1)..(index + dist_to_match - 1); //does not
                let evaluated_node = self.nest_into_tree(&source[exc_range]);
                node_pool.push(evaluated_node);
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

    /*

    fn _nest_tree_recursive<CompA>(&self, source: &[CompA], offset: usize) -> IndexTree
    where
        CompA: AsRef<[A::Atom]>,
    {
        let source_vec: Vec<&CompA> = source.iter().collect();

        let mut result = vec![];

        let mut index = 0;

        while index < source.len() {
            let value = &source[index];

            if value.as_ref() == &*self.delimiters.0 {
                let mut possible_range = source[index..].iter();
                let mut delimiters_found = 0;
                let mut distance = 0;

                while let Some(item) = possible_range.next() {
                    if item.as_ref() == &*self.delimiters.0 {
                        delimiters_found += 1;
                    } else if item.as_ref() == &*self.delimiters.1 {
                        delimiters_found -= 1;
                    }

                    if delimiters_found == 0 {
                        break;
                    }

                    distance += 1;
                }

                let (start, end) = (index + 1, index + distance);

                let evaluated_inner = self
                    ._nest_tree_recursive(&source[start..end], offset + start)
                    .release_nodes();

                let parent_index = result.len();
                let children_start = parent_index + 1;
                let children_end = children_start + evaluated_inner.len();

                result.push(IndexNode {
                    range: ((offset + start)..(offset + end + 1)),
                    children: if evaluated_inner.is_empty() {
                        None
                    } else {
                        Some(children_start..children_end)
                    },
                });

                // Adjust child indices in evaluated_inner to account for the offset
                let index_offset = children_start;
                let adjusted_inner: Vec<IndexNode> = evaluated_inner
                    .into_iter()
                    .map(|mut node| {
                        if let Some(x) = node.children {
                            node.children = Some(x.start + index_offset..x.end + index_offset);
                        }
                        node
                    })
                    .collect();

                result.extend(adjusted_inner);
                index = end + 1;
            } else {
                result.push(IndexNode {
                    range: ((offset + index)..(offset + index + 1)),
                    children: None,
                });
                index += 1;
            }
        }

        let l = result.len();

        IndexTree {
            nodes: result,
            root: IndexNode {
                range: 0..0,
                children: Some(0..l),
            },
        }
    }
    */
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
