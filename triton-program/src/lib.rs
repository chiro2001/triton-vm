use anyhow::*;
use std::any::Any;
use std::fmt::{Debug, Formatter};
use twenty_first::shared_math::b_field_element::BFieldElement;
use twenty_first::util_types::algebraic_hasher::Hashable;

pub trait AbstractInstruction {
    fn clone_(&self) -> Box<dyn AbstractInstruction>;
}
pub trait AbstractLabelledInstruction {
    fn as_any(&self) -> &dyn Any;
}

pub trait AbstractProgram {
    fn to_bwords(&self) -> Vec<BFieldElement>;
    fn len_bwords(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn get_instructions(&self) -> Vec<Box<dyn AbstractInstruction>>;
    fn to_sequence_(&self) -> Vec<BFieldElement>;
    fn clone_(&self) -> Box<dyn AbstractProgram>;
    fn as_any(&self) -> &dyn Any;
}

pub trait FromCode {
    fn from_code(code: &str) -> Result<Box<dyn AbstractProgram>>
    where
        Self: Sized;
    fn create(input: &[Box<dyn AbstractLabelledInstruction>]) -> Box<dyn AbstractProgram>;
}

impl Clone for Box<dyn AbstractProgram> {
    fn clone(&self) -> Self {
        self.clone_()
    }
}

impl Debug for dyn AbstractProgram {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Program {{ instructions: [")?;
        for instruction in self.get_instructions() {
            write!(f, "{:?}, ", instruction)?;
        }
        write!(f, "] }}")
    }
}

impl Hashable for Box<dyn AbstractProgram> {
    fn to_sequence(&self) -> Vec<BFieldElement> {
        self.to_sequence_()
    }
}

impl Debug for dyn AbstractInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Instruction {{ }}")
    }
}

impl Clone for Box<dyn AbstractInstruction> {
    fn clone(&self) -> Self {
        self.clone_()
    }
}

impl PartialEq for dyn AbstractInstruction {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}
