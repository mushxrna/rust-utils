use std::fmt::{Display, Formatter, Result};

use crate::wgpu_helpers::*;

#[derive(Debug)]
pub enum PipelineError {
    WrongType(ActivePipeline),
    PassError(Box<dyn std::error::Error>),
    Specific(String),
}

impl Display for PipelineError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            PipelineError::WrongType(attempted_call) => match attempted_call {
                ActivePipeline::Render(_) => {
                    write!(f, "Tried to get Compute pipeline from a Render Pipeline!")
                }
                ActivePipeline::Compute(_) => {
                    write!(f, "Tried to get Render pipeline from a Compute Pipeline!")
                }
            },
            PipelineError::PassError(err) => {
                write!(f, "Error encountered during the pass! Error: {}", err)
            }
            PipelineError::Specific(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for PipelineError {}
