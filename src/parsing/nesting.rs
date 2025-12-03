use std::ops::{Index, Range};
//
// ENUMS AND STRUCTS
//
pub enum NestedObject<A> {
    Atom(A),
    Molecule(Vec<NestedObject<A>>),
}
pub struct IndexNode {
    range: Range<usize>,
    children: Option<Range<usize>>,
}
pub struct IndexTree {
    nodes: Vec<IndexNode>,
    root: IndexNode,
}
pub struct Nester<A> {
    pub delimiters: (A, A),
}
//
// IMPL METHODS
//
impl IndexNode {
    pub fn get_children(&self) -> &Option<Range<usize>> {
        &self.children
    }

    pub fn get_range(&self) -> Range<usize> {
        self.range.clone()
    }

    pub fn get_start(&self) -> usize {
        self.range.start
    }

    pub fn get_end(&self) -> usize {
        self.range.end
    }

    pub fn ref_into<'a, A: Index<usize, Output = B>, B>(&self, s: &'a A) -> &'a B {
        &s[self.get_start()]
    }
}
impl IndexTree {
    pub fn get_nodes(&self) -> &Vec<IndexNode> {
        &self.nodes
    }

    pub fn release_nodes(self) -> Vec<IndexNode> {
        self.nodes
    }

    pub fn ref_child_nodes(&self) -> Vec<&IndexNode> {
        let mut result = vec![];

        for i in &self.nodes {
            if i.get_children().is_none() {
                result.push(i)
            }
        }

        result
    }
}
impl<A: PartialEq> Nester<A> {
    pub fn new(delimiters: (A, A)) -> Nester<A> {
        Nester { delimiters }
    }
    pub fn nest_into_object<'a>(&self, source: &'a [A]) -> NestedObject<&'a A> {
        let source_vec: Vec<&'a A> = source.iter().collect();

        let mut result = vec![];

        let mut index = 0;

        while index < source_vec.len() {
            let value = source_vec[index];

            if value == &self.delimiters.0 {
                let mut possible_range = source_vec[index..].iter();
                let mut delimiters_found = 0;
                let mut distance = 0;

                while let Some(&item) = possible_range.next() {
                    if item == &self.delimiters.0 {
                        delimiters_found += 1;
                    } else if item == &self.delimiters.1 {
                        delimiters_found -= 1;
                    }

                    if delimiters_found == 0 {
                        break;
                    }

                    distance += 1;
                }

                let (start, end) = (index + 1, index + distance);

                let evaluated_inner = self.nest_into_object(&source[start..end]);
                result.push(evaluated_inner);
                index = end;
            } else {
                result.push(NestedObject::Atom(value))
            }

            index += 1;
        }

        NestedObject::Molecule(result)
    }

    pub fn nest_into_tree<CompA: PartialEq<A>>(&self, source: &[CompA]) -> IndexTree {
        self._nest_tree_recursive(source, 0)
    }

    fn _nest_tree_recursive<CompA>(&self, source: &[CompA], offset: usize) -> IndexTree
    where
        CompA: PartialEq<A>,
    {
        let source_vec: Vec<&CompA> = source.iter().collect();

        let mut result = vec![];

        let mut index = 0;

        while index < source.len() {
            let value = &source[index];

            if value == &self.delimiters.0 {
                let mut possible_range = source[index..].iter();
                let mut delimiters_found = 0;
                let mut distance = 0;

                while let Some(item) = possible_range.next() {
                    if item == &self.delimiters.0 {
                        delimiters_found += 1;
                    } else if item == &self.delimiters.1 {
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
                        Some((children_start..children_end))
                    },
                });

                // Adjust child indices in evaluated_inner to account for the offset
                let index_offset = children_start;
                let adjusted_inner: Vec<IndexNode> = evaluated_inner
                    .into_iter()
                    .map(|mut node| {
                        if let Some(x) = node.children {
                            node.children = Some((x.start + index_offset..x.end + index_offset));
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
