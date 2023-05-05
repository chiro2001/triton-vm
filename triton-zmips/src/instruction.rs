use std::collections::HashMap;
use std::fmt::Display;
use std::vec;

use anyhow::bail;
use anyhow::Result;
use strum::EnumCount;
use strum::IntoEnumIterator;
use strum_macros::Display as DisplayMacro;
use strum_macros::EnumCount as EnumCountMacro;
use strum_macros::EnumIter;
use twenty_first::shared_math::b_field_element::BFieldElement;
use twenty_first::shared_math::b_field_element::BFIELD_ZERO;

use crate::ord_n::Regs;
use AnInstruction::*;

/// An `Instruction` has `call` addresses encoded as absolute integers.
pub type Instruction = AnInstruction<BFieldElement, Regs, u32>;

pub const ALL_INSTRUCTIONS: [Instruction; Instruction::COUNT] = all_instructions_without_args();
pub const ALL_INSTRUCTION_NAMES: [&str; Instruction::COUNT] = all_instruction_names();

/// A `LabelledInstruction` has `call` addresses encoded as label names.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LabelledInstruction {
    /// Instructions belong to the ISA:
    ///
    /// <https://triton-vm.org/spec/isa.html>
    Instruction(AnInstruction<String, String, String>),

    /// Labels look like "`<name>:`" and are translated into absolute addresses.
    Label(String),
}

impl Display for LabelledInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LabelledInstruction::Instruction(instr) => write!(f, "{instr}"),
            LabelledInstruction::Label(label_name) => write!(f, "{label_name}:"),
        }
    }
}

#[derive(Debug, DisplayMacro, Clone, Copy, PartialEq, Eq, Hash, EnumCountMacro)]
pub enum DivinationHint {}

/// A Triton VM instruction. See the
/// [Instruction Set Architecture](https://triton-vm.org/spec/isa.html)
/// for more details.
///
/// The type parameter `Dest` describes the type of addresses (absolute or labels).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCountMacro, EnumIter)]
pub enum AnInstruction<Dest: PartialEq + Default, R: PartialEq + Default, I: PartialEq + Default> {
    // Control flow
    BEQ((R, R, Dest)),
    BNE((R, R, Dest)),
    BLT((R, R, Dest)),
    BLE((R, R, Dest)),
    SEQ((R, R, I)),
    SNE((R, R, I)),
    SLT((R, R, I)),
    SLE((R, R, I)),
    J(Dest),
    JR(R),

    // Memory access
    LW((R, I, R)),
    SW((R, I, R)),

    // Base field arithmetic on stack
    ADD((R, R, I)),
    SUB((R, R, I)),
    MULT((R, R, I)),
    DIV((R, R, I)),
    MOD((R, R, I)),
    MOVE((R, I)),
    LA((R, I)),

    // Bitwise arithmetic on stack
    AND((R, R, I)),
    XOR((R, R, I)),
    NOT((R, R, I)),
    SLL((R, R, I)),
    SRL((R, R, I)),

    // Read/write
    PUBREAD(R),
    SECREAD(R),
    PUBSEEK((R, I)),
    SECSEEK((R, I)),
    PRINT(R),
    EXIT(R),
    ANSWER(R),
}

impl<Dest: PartialEq + Default, R: PartialEq + Default, I: PartialEq + Default>
    AnInstruction<Dest, R, I>
{
    // /// Drop the specific argument in favor of a default one.
    // pub fn strip(&self) -> Self {
    //     self.clone()
    // }

    /// Assign a unique positive integer to each `Instruction`.
    pub fn opcode(&self) -> u32 {
        match self {
            BEQ(_) => 0,
            BNE(_) => 1,
            BLT(_) => 2,
            BLE(_) => 3,
            SEQ(_) => 4,
            SNE(_) => 5,
            SLT(_) => 6,
            SLE(_) => 7,
            J(_) => 8,
            JR(_) => 9,
            LW(_) => 10,
            SW(_) => 11,
            ADD(_) => 12,
            SUB(_) => 13,
            MULT(_) => 14,
            DIV(_) => 15,
            MOD(_) => 16,
            MOVE(_) => 17,
            LA(_) => 18,
            AND(_) => 19,
            XOR(_) => 20,
            NOT(_) => 21,
            SLL(_) => 22,
            SRL(_) => 23,
            PUBREAD(_) => 24,
            SECREAD(_) => 25,
            PUBSEEK(_) => 26,
            SECSEEK(_) => 27,
            PRINT(_) => 28,
            EXIT(_) => 29,
            ANSWER(_) => 30,
        }
    }

    const fn name(&self) -> &'static str {
        match self {
            BEQ(_) => "beq",
            BNE(_) => "bne",
            BLT(_) => "blt",
            BLE(_) => "ble",
            SEQ(_) => "seq",
            SNE(_) => "sne",
            SLT(_) => "slt",
            SLE(_) => "sle",
            J(_) => "j",
            JR(_) => "jr",
            LW(_) => "lw",
            SW(_) => "sw",
            ADD(_) => "add",
            SUB(_) => "sub",
            MULT(_) => "mult",
            DIV(_) => "div",
            MOD(_) => "mod",
            MOVE(_) => "move",
            LA(_) => "la",
            AND(_) => "and",
            XOR(_) => "xor",
            NOT(_) => "not",
            SLL(_) => "sll",
            SRL(_) => "srl",
            PUBREAD(_) => "pubread",
            SECREAD(_) => "secread",
            PUBSEEK(_) => "pubseek",
            SECSEEK(_) => "secseek",
            PRINT(_) => "print",
            EXIT(_) => "exit",
            ANSWER(_) => "answer",
        }
    }

    pub fn opcode_b(&self) -> BFieldElement {
        self.opcode().into()
    }

    pub fn size(&self) -> usize {
        // match matches!(self, Push(_) | Dup(_) | Swap(_) | Call(_)) {
        //     true => 2,
        //     false => 1,
        // }
        1
    }

    // /// Get the i'th instruction bit
    // pub fn ib(&self, arg: Ord8) -> BFieldElement {
    //     let opcode = self.opcode();
    //     let bit_number: usize = arg.into();
    //
    //     ((opcode >> bit_number) & 1).into()
    // }
}

impl<Dest: PartialEq + Default> AnInstruction<Dest, String, String> {
    fn map_call_address<F, NewDest: PartialEq + Default>(
        &self,
        f: F,
    ) -> AnInstruction<NewDest, Regs, u32>
    where
        F: Fn(&Dest) -> NewDest,
        Dest: Clone,
    {
        match self {
            BEQ((r1, r2, addr)) => BEQ((r1.into(), r2.into(), f(addr))),
            BNE((r1, r2, addr)) => BNE((r1.into(), r2.into(), f(addr))),
            BLT((r1, r2, addr)) => BLT((r1.into(), r2.into(), f(addr))),
            BLE((r1, r2, addr)) => BLE((r1.into(), r2.into(), f(addr))),
            SEQ((r1, r2, imm)) => SEQ((r1.into(), r2.into(), imm.parse().unwrap())),
            SNE((r1, r2, imm)) => SNE((r1.into(), r2.into(), imm.parse().unwrap())),
            SLT((r1, r2, imm)) => SLT((r1.into(), r2.into(), imm.parse().unwrap())),
            SLE((r1, r2, imm)) => SLE((r1.into(), r2.into(), imm.parse().unwrap())),
            J(label) => J(f(label)),
            JR(r) => JR(r.into()),
            LW((r1, imm, r2)) => LW((r1.into(), imm.parse().unwrap(), r2.into())),
            SW((r1, imm, r2)) => SW((r1.into(), imm.parse().unwrap(), r2.into())),
            ADD((r1, r2, imm)) => ADD((r1.into(), r2.into(), imm.parse().unwrap())),
            SUB((r1, r2, imm)) => SUB((r1.into(), r2.into(), imm.parse().unwrap())),
            MULT((r1, r2, imm)) => MULT((r1.into(), r2.into(), imm.parse().unwrap())),
            DIV((r1, r2, imm)) => DIV((r1.into(), r2.into(), imm.parse().unwrap())),
            MOD((r1, r2, imm)) => MOD((r1.into(), r2.into(), imm.parse().unwrap())),
            MOVE((r, imm)) => MOVE((r.into(), imm.parse().unwrap())),
            LA((r, imm)) => LA((r.into(), imm.parse().unwrap())),
            AND((r1, r2, imm)) => AND((r1.into(), r2.into(), imm.parse().unwrap())),
            XOR((r1, r2, imm)) => XOR((r1.into(), r2.into(), imm.parse().unwrap())),
            NOT((r1, r2, imm)) => NOT((r1.into(), r2.into(), imm.parse().unwrap())),
            SLL((r1, r2, imm)) => SLL((r1.into(), r2.into(), imm.parse().unwrap())),
            SRL((r1, r2, imm)) => SRL((r1.into(), r2.into(), imm.parse().unwrap())),
            PUBREAD(r) => PUBREAD(r.into()),
            SECREAD(r) => SECREAD(r.into()),
            PUBSEEK((r, imm)) => PUBSEEK((r.into(), imm.parse().unwrap())),
            SECSEEK((r, imm)) => SECSEEK((r.into(), imm.parse().unwrap())),
            PRINT(r) => PRINT(r.into()),
            EXIT(r) => EXIT(r.into()),
            ANSWER(r) => ANSWER(r.into()),
        }
    }
}

impl<
        Dest: Display + PartialEq + Default,
        R: Display + PartialEq + Default + Clone,
        I: Display + PartialEq + Default + Clone,
    > Display for AnInstruction<Dest, R, I>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())?;
        match self {
            BEQ((r1, r2, addr)) => write!(f, " {}, {}, {}", r1, r2, addr),
            BNE((r1, r2, addr)) => write!(f, " {}, {}, {}", r1, r2, addr),
            BLT((r1, r2, addr)) => write!(f, " {}, {}, {}", r1, r2, addr),
            BLE((r1, r2, addr)) => write!(f, " {}, {}, {}", r1, r2, addr),
            J(addr) => write!(f, " {}", addr),
            _ => Ok(()),
        }
    }
}

impl Instruction {
    pub fn args(&self) -> Vec<BFieldElement> {
        match self {
            BEQ((r1, r2, addr)) => vec![r1.into(), r2.into(), *addr],
            BNE((r1, r2, addr)) => vec![r1.into(), r2.into(), *addr],
            BLE((r1, r2, addr)) => vec![r1.into(), r2.into(), *addr],
            J(addr) => vec![*addr],
            _ => vec![],
        }
    }
}

impl TryFrom<u32> for Instruction {
    type Error = anyhow::Error;

    fn try_from(opcode: u32) -> Result<Self> {
        if let Some(instruction) =
            Instruction::iter().find(|instruction| instruction.opcode() == opcode)
        {
            Ok(instruction)
        } else {
            bail!("No instruction with opcode {opcode} exists.")
        }
    }
}

impl TryFrom<u64> for Instruction {
    type Error = anyhow::Error;

    fn try_from(opcode: u64) -> Result<Self> {
        (opcode as u32).try_into()
    }
}

impl TryFrom<usize> for Instruction {
    type Error = anyhow::Error;

    fn try_from(opcode: usize) -> Result<Self> {
        (opcode as u32).try_into()
    }
}

// fn ord16_to_bfe(n: &Ord16) -> BFieldElement {
//     let n: u32 = n.into();
//     n.into()
// }

/// Convert a program with labels to a program with absolute positions
pub fn convert_labels(program: &[LabelledInstruction]) -> Vec<Instruction> {
    let mut label_map = HashMap::<String, usize>::new();
    let mut instruction_pointer: usize = 0;

    // 1. Add all labels to a map
    for labelled_instruction in program.iter() {
        match labelled_instruction {
            LabelledInstruction::Label(label_name) => {
                label_map.insert(label_name.clone(), instruction_pointer);
            }

            LabelledInstruction::Instruction(instr) => {
                instruction_pointer += instr.size();
            }
        }
    }

    // 2. Convert every label to the lookup value of that map
    program
        .iter()
        .flat_map(|labelled_instruction| convert_labels_helper(labelled_instruction, &label_map))
        .collect()
}

fn convert_labels_helper(
    instruction: &LabelledInstruction,
    label_map: &HashMap<String, usize>,
) -> Vec<Instruction> {
    match instruction {
        LabelledInstruction::Label(_) => vec![],

        LabelledInstruction::Instruction(instr) => {
            let unlabelled_instruction: AnInstruction<BFieldElement, Regs, u32> = instr
                .map_call_address(|label_name| {
                    let label_not_found = format!("Label not found: {label_name}");
                    let absolute_address = label_map.get(label_name).expect(&label_not_found);
                    BFieldElement::new(*absolute_address as u64)
                });

            vec![unlabelled_instruction]
        }
    }
}

const DEFAULT_BRANCH_INFO: (Regs, Regs, BFieldElement) = (Regs::Zero, Regs::Zero, BFIELD_ZERO);
const DEFAULT_LOAD_SAVE: (Regs, u32, Regs) = (Regs::Zero, 0, Regs::Zero);
const DEFAULT_INFO3: (Regs, Regs, u32) = (Regs::Zero, Regs::Zero, 0);
const DEFAULT_INFO2: (Regs, u32) = (Regs::Zero, 0);

const fn all_instructions_without_args(
) -> [AnInstruction<BFieldElement, Regs, u32>; Instruction::COUNT] {
    [
        BEQ(DEFAULT_BRANCH_INFO),
        BNE(DEFAULT_BRANCH_INFO),
        BLT(DEFAULT_BRANCH_INFO),
        BLE(DEFAULT_BRANCH_INFO),
        SEQ(DEFAULT_INFO3),
        SNE(DEFAULT_INFO3),
        SLT(DEFAULT_INFO3),
        SLE(DEFAULT_INFO3),
        J(BFIELD_ZERO),
        JR(Regs::Zero),
        LW(DEFAULT_LOAD_SAVE),
        SW(DEFAULT_LOAD_SAVE),
        ADD(DEFAULT_INFO3),
        SUB(DEFAULT_INFO3),
        MULT(DEFAULT_INFO3),
        DIV(DEFAULT_INFO3),
        MOD(DEFAULT_INFO3),
        MOVE(DEFAULT_INFO2),
        LA(DEFAULT_INFO2),
        AND(DEFAULT_INFO3),
        XOR(DEFAULT_INFO3),
        NOT(DEFAULT_INFO3),
        SLL(DEFAULT_INFO3),
        SRL(DEFAULT_INFO3),
        PUBREAD(Regs::Zero),
        SECREAD(Regs::Zero),
        PUBSEEK(DEFAULT_INFO2),
        SECSEEK(DEFAULT_INFO2),
        PRINT(Regs::Zero),
        EXIT(Regs::Zero),
        ANSWER(Regs::Zero),
    ]
}

const fn all_instruction_names() -> [&'static str; Instruction::COUNT] {
    let mut names = [""; Instruction::COUNT];
    let mut i = 0;
    while i < Instruction::COUNT {
        names[i] = ALL_INSTRUCTIONS[i].name();
        i += 1;
    }
    names
}

pub mod sample_programs {
    pub const SPECK128: &str = "
        move $t4, 32
        secread $t2
        secread $t3
        __L1__:
            srl $t5, $t3, 8
            sll $t6, $t3, 56
            or $t6, $t5, $t6
            add $t3, $t6, $t2
            secread $t7

            xor $t3, $t3, $t7
            srl $t5, $t2, 61
            sll $t6, $t2, 3
            or $t6, $t5, $t6
            xor $t2, $t6, $t3

            add $t1, $t1, 1
        bgt $t4, $t1, __L1__
        print $t2
        print $t3
        answer $t3
    ";
}

#[cfg(test)]
mod instruction_tests {
    // use itertools::Itertools;
    // use num_traits::One;
    // use num_traits::Zero;
    // use strum::EnumCount;
    // use strum::IntoEnumIterator;
    // use twenty_first::shared_math::b_field_element::BFieldElement;

    use crate::instruction::ALL_INSTRUCTIONS;
    use crate::program::Program;

    // use super::AnInstruction;
    // use super::AnInstruction::*;

    // #[test]
    // fn opcode_test() {
    //     // test for duplicates
    //     let mut opcodes = vec![];
    //     for instruction in AnInstruction::<BFieldElement, String>::iter() {
    //         assert!(
    //             !opcodes.contains(&instruction.opcode()),
    //             "Have different instructions with same opcode."
    //         );
    //         opcodes.push(instruction.opcode());
    //     }
    //
    //     for opc in opcodes.iter() {
    //         println!(
    //             "opcode {} exists: {}",
    //             opc,
    //             AnInstruction::<BFieldElement, String>::try_from(*opc).unwrap()
    //         );
    //     }
    //
    //     // assert size of list corresponds to number of opcodes
    //     assert_eq!(
    //         AnInstruction::<BFieldElement, String>::COUNT,
    //         opcodes.len(),
    //         "Mismatch in number of instructions!"
    //     );
    //
    //     // test for width
    //     let max_opcode: u32 = AnInstruction::<BFieldElement, String>::iter()
    //         .map(|inst| inst.opcode())
    //         .max()
    //         .unwrap();
    //     let mut num_bits = 0;
    //     while (1 << num_bits) < max_opcode {
    //         num_bits += 1;
    //     }
    //     assert!(
    //         num_bits <= Ord8::COUNT,
    //         "Biggest instruction needs more than {} bits :(",
    //         Ord8::COUNT
    //     );
    //
    //     // assert consistency
    //     for instruction in AnInstruction::<BFieldElement, String>::iter() {
    //         assert!(
    //             instruction == instruction.opcode().try_into().unwrap(),
    //             "instruction to opcode map must be consistent"
    //         );
    //     }
    // }

    // #[test]
    // fn parse_push_pop_test() {
    //     let code = "
    //         push 1
    //         push 1
    //         add
    //         pop
    //     ";
    //     let program = Program::from_code(code).unwrap();
    //     let instructions = program.into_iter().collect_vec();
    //     let expected = vec![
    //         Push(BFieldElement::one()),
    //         Push(BFieldElement::one()),
    //         ADD,
    //         Pop,
    //     ];
    //
    //     assert_eq!(expected, instructions);
    // }

    #[test]
    fn fail_on_duplicate_labels_test() {
        let code = "
            push 2
            call foo
            bar: push 2
            foo: push 3
            foo: push 4
            halt
        ";
        let program = Program::from_code(code);
        assert!(
            program.is_err(),
            "Duplicate labels should result in a parse error"
        );
    }

    // #[test]
    // fn ib_registers_are_binary_test() {
    //     use Ord8::*;
    //
    //     for instruction in ALL_INSTRUCTIONS {
    //         let all_ibs: [Ord8; Ord8::COUNT] = [IB0, IB1, IB2, IB3, IB4, IB5, IB6, IB7];
    //         for ib in all_ibs {
    //             let ib_value = instruction.ib(ib);
    //             assert!(
    //                 ib_value.is_zero() || ib_value.is_one(),
    //                 "IB{ib} for instruction {instruction} is 0 or 1 ({ib_value})",
    //             );
    //         }
    //     }
    // }

    #[test]
    fn instruction_to_opcode_to_instruction_is_consistent_test() {
        for instr in ALL_INSTRUCTIONS {
            assert_eq!(instr, instr.opcode().try_into().unwrap());
        }
    }

    #[test]
    fn print_all_instructions_and_opcodes() {
        for instr in ALL_INSTRUCTIONS {
            println!("{:>3} {: <10}", instr.opcode(), format!("{}", instr.name()));
        }
    }
}
