use std::{fmt::Debug, ops::Deref};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum IndexNodeError {
    #[error("Failed to get associated item.")]
    AssociationError,
    #[error("No associated item on node.")]
    NoAssociationError,
    #[error("Could not associate node.")]
    CouldNotAssociateError,
}

pub enum IndexNode<A> {
    Associated(NodeData, A),
    NotAssociated(NodeData),
}

#[derive(Clone)]
pub struct NodeData {
    index: Option<usize>, //only leaf nodes have an index
    children: Option<Vec<NodeData>>,
}

impl NodeData {
    pub fn children(&self) -> Option<&Vec<NodeData>> {
        self.children.as_ref()
    }

    pub fn index(&self) -> Option<usize> {
        self.index
    }
}

impl<A> IndexNode<A> {
    //------------------------------------------------------------------------------------------
    fn data(&self) -> &NodeData {
        match self {
            IndexNode::Associated(x, _) | IndexNode::NotAssociated(x) => x,
        }
    }

    fn into_data(self) -> NodeData {
        match self {
            IndexNode::Associated(x, _) | IndexNode::NotAssociated(x) => x,
        }
    }
    //------------------------------------------------------------------------------------------
    pub fn children(&self) -> Option<Vec<IndexNode<A>>> {
        let c = self
            .data()
            .children()?
            .iter()
            .map(|x| IndexNode::NotAssociated(x.clone()))
            .collect();

        Some(c)
    }

    pub fn index(&self) -> Option<usize> {
        self.data().index()
    }

    pub fn is_associated(&self) -> bool {
        matches!(self, IndexNode::Associated(_, _))
    }
    //------------------------------------------------------------------------------------------
    pub fn new(index: Option<usize>, children: Option<Vec<IndexNode<A>>>) -> IndexNode<A> {
        let c = (children.is_some())
            .then(|| Vec::from_iter(children.unwrap().into_iter().map(|x| x.into_data())));

        IndexNode::NotAssociated(NodeData { index, children: c })
    }

    pub fn associate<B: Clone>(self, source: &[B]) -> Result<IndexNode<B>, IndexNodeError> {
        let i = self.index().ok_or(IndexNodeError::CouldNotAssociateError)?;
        let val = &source[i];

        Ok(IndexNode::Associated(self.into_data(), val.clone()))
    }
    //------------------------------------------------------------------------------------------

    pub fn associated(&self) -> Result<&A, IndexNodeError> {
        match self {
            IndexNode::Associated(data, assoc) => Ok(assoc),
            IndexNode::NotAssociated(d) => Err(IndexNodeError::NoAssociationError),
        }
    }
}

impl<A: Clone + Debug> std::fmt::Debug for IndexNode<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.debug_struct("IndexNode")
            .field("Associated", &self.is_associated())
            .field(
                "Associated Item",
                &(self.is_associated())
                    .then(|| -> String { format!("{:?}", self.associated().unwrap()) })
                    .unwrap_or_else(|| -> String { String::from("N/A") }),
            )
            .field("Children", &self.children().iter().len())
            .finish();

        Ok(())
    }
}
