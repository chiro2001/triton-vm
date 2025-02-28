use std::any::Any;
use std::fmt::{Debug, Display};
use std::io::Cursor;

use anyhow::Result;
use itertools::Itertools;
use triton_program::{AbstractInstruction, AbstractLabelledInstruction, AbstractProgram, FromCode};
use twenty_first::shared_math::b_field_element::BFieldElement;
use twenty_first::util_types::algebraic_hasher::Hashable;

use crate::instruction::convert_labels;
use crate::instruction::Instruction;
use crate::instruction::LabelledInstruction;
use crate::parser::parse;
use crate::parser::to_labelled;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stream = self.instructions.iter();
        while let Some(instruction) = stream.next() {
            writeln!(f, "{instruction}")?;

            // Skip duplicate placeholder used for aligning instructions and instruction_pointer in VM.
            for _ in 1..instruction.size() {
                stream.next();
            }
        }
        Ok(())
    }
}

impl Hashable for Program {
    fn to_sequence(&self) -> Vec<BFieldElement> {
        self.to_bwords()
    }
}

/// An `InstructionIter` loops the instructions of a `Program` by skipping duplicate placeholders.
pub struct InstructionIter {
    cursor: Cursor<Vec<Instruction>>,
}

impl Iterator for InstructionIter {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.cursor.position() as usize;
        let instructions = self.cursor.get_ref();
        let instruction = *instructions.get(pos)?;
        self.cursor.set_position((pos + instruction.size()) as u64);

        Some(instruction)
    }
}

impl IntoIterator for Program {
    type Item = Instruction;

    type IntoIter = InstructionIter;

    fn into_iter(self) -> Self::IntoIter {
        let cursor = Cursor::new(self.instructions);
        InstructionIter { cursor }
    }
}

/// A `Program` is a `Vec<Instruction>` that contains duplicate elements for
/// instructions with a size of 2. This means that the index in the vector
/// corresponds to the VM's `instruction_pointer`. These duplicate values
/// should most often be skipped/ignored, e.g. when pretty-printing.
impl Program {
    /// Create a `Program` from a slice of `Instruction`.
    // pub fn new(input: &[Box<dyn AbstractLabelledInstruction>]) -> Self {
    pub fn new(input: &[LabelledInstruction]) -> Self {
        let instructions = convert_labels(input)
            .iter()
            .flat_map(|instr| vec![*instr; instr.size()])
            .collect::<Vec<_>>();

        Program { instructions }
    }

    /// Create a `Program` by parsing source code.
    pub fn from_code(code: &str) -> Result<Self> {
        // parse(code)
        //     .map(|program| {
        //         Program::new(
        //             &to_labelled(&program)
        //                 .into_iter()
        //                 .map(|x| Box::new(x) as Box<dyn AbstractLabelledInstruction>)
        //                 .collect::<Vec<_>>(),
        //         )
        //     })
        //     .map_err(|err| anyhow::anyhow!("{}", err))
        parse(code)
            .map(|program| Program::new(&to_labelled(&program)))
            .map_err(|err| anyhow::anyhow!("{}", err))
    }

    /// Convert a `Program` to a `Vec<BFieldElement>`.
    ///
    /// Every single-word instruction is converted to a single word.
    ///
    /// Every double-word instruction is converted to two words.
    pub fn to_bwords(&self) -> Vec<BFieldElement> {
        self.clone()
            .into_iter()
            .flat_map(|instruction| {
                let opcode = instruction.opcode_b();
                if let Some(arg) = instruction.arg() {
                    vec![opcode, arg]
                } else {
                    vec![opcode]
                }
            })
            .collect()
    }

    /// The total length of the program as `BFieldElement`s. Double-word instructions contribute
    /// two `BFieldElement`s.
    pub fn len_bwords(&self) -> usize {
        self.instructions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }
}

impl AbstractInstruction for Instruction {
    fn clone_(&self) -> Box<dyn AbstractInstruction> {
        Box::new(*self) as Box<dyn AbstractInstruction>
    }
}

impl AbstractProgram for Program {
    fn to_bwords(&self) -> Vec<BFieldElement> {
        self.to_bwords()
    }

    fn len_bwords(&self) -> usize {
        self.len_bwords()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn get_instructions(&self) -> Vec<Box<dyn AbstractInstruction>> {
        self.instructions
            .iter()
            .map(|x| Box::new(*x) as Box<dyn AbstractInstruction>)
            .collect::<Vec<_>>()
    }

    fn to_sequence_(&self) -> Vec<BFieldElement> {
        self.to_sequence()
    }

    fn clone_(&self) -> Box<dyn AbstractProgram> {
        Box::new(self.clone()) as Box<dyn AbstractProgram>
    }

    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl FromCode for Program {
    fn from_code(code: &str) -> Result<Box<dyn AbstractProgram>>
    where
        Self: Sized,
    {
        Program::from_code(code).map(|x| Box::new(x) as Box<dyn AbstractProgram>)
    }

    fn create(input: &[Box<dyn AbstractLabelledInstruction>]) -> Box<dyn AbstractProgram> {
        let input = input
            .iter()
            .map(|x| x.as_any().downcast_ref::<LabelledInstruction>().unwrap())
            .cloned()
            .collect_vec();
        Box::new(Program::new(input.as_slice())) as Box<dyn AbstractProgram>
    }

    // fn create(input: &[LabelledInstruction]) -> Box<dyn AbstractProgram> {
    //     Box::new(Program::new(input)) as Box<dyn AbstractProgram>
    // }
}
