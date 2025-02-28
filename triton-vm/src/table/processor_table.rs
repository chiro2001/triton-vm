use std::cmp::max;
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Mul;

use itertools::Itertools;
use ndarray::parallel::prelude::*;
use ndarray::s;
use ndarray::Array1;
use ndarray::ArrayView1;
use ndarray::ArrayView2;
use ndarray::ArrayViewMut2;
use ndarray::Axis;
use num_traits::One;
use num_traits::Zero;
use strum::EnumCount;
use triton_opcodes::instruction::AnInstruction::*;
use triton_opcodes::instruction::Instruction;
use triton_opcodes::instruction::ALL_INSTRUCTIONS;
use triton_opcodes::ord_n::Ord8;
use twenty_first::shared_math::b_field_element::BFieldElement;
use twenty_first::shared_math::b_field_element::BFIELD_ONE;
use twenty_first::shared_math::traits::Inverse;
use twenty_first::shared_math::x_field_element::XFieldElement;

use crate::table::challenges::ChallengeId::*;
use crate::table::challenges::Challenges;
use crate::table::constraint_circuit::ConstraintCircuit;
use crate::table::constraint_circuit::ConstraintCircuitBuilder;
use crate::table::constraint_circuit::ConstraintCircuitMonad;
use crate::table::constraint_circuit::DualRowIndicator;
use crate::table::constraint_circuit::InputIndicator;
use crate::table::constraint_circuit::SingleRowIndicator;
use crate::table::cross_table_argument::CrossTableArg;
use crate::table::cross_table_argument::EvalArg;
use crate::table::cross_table_argument::LookupArg;
use crate::table::cross_table_argument::PermArg;
use crate::table::master_table::NUM_BASE_COLUMNS;
use crate::table::master_table::NUM_EXT_COLUMNS;
use crate::table::table_column::MasterBaseTableColumn;
use crate::table::table_column::MasterExtTableColumn;
use crate::table::table_column::ProcessorBaseTableColumn;
use crate::table::table_column::ProcessorBaseTableColumn::*;
use crate::table::table_column::ProcessorExtTableColumn;
use crate::table::table_column::ProcessorExtTableColumn::*;
use crate::vm::AlgebraicExecutionTrace;

pub const BASE_WIDTH: usize = ProcessorBaseTableColumn::COUNT;
pub const EXT_WIDTH: usize = ProcessorExtTableColumn::COUNT;
pub const FULL_WIDTH: usize = BASE_WIDTH + EXT_WIDTH;

#[derive(Debug, Clone)]
pub struct ProcessorTable {}

impl ProcessorTable {
    pub fn fill_trace(
        processor_table: &mut ArrayViewMut2<BFieldElement>,
        aet: &AlgebraicExecutionTrace,
        clk_jump_diffs_op_stack: &[BFieldElement],
        clk_jump_diffs_ram: &[BFieldElement],
        clk_jump_diffs_jump_stack: &[BFieldElement],
    ) {
        // compute the lookup multiplicities of the clock jump differences
        let num_rows = aet.processor_trace.nrows();
        let mut clk_jump_diff_multiplicities = Array1::zeros([num_rows]);
        for clk_jump_diff in clk_jump_diffs_op_stack.iter() {
            let clk = clk_jump_diff.value() as usize;
            match clk < num_rows {
                true => clk_jump_diff_multiplicities[clk] += BFIELD_ONE,
                false => panic!(
                    "Op Stack: clock jump diff {clk} must fit in trace with {num_rows} rows."
                ),
            }
        }
        for clk_jump_diff in clk_jump_diffs_ram.iter() {
            let clk = clk_jump_diff.value() as usize;
            match clk < num_rows {
                true => clk_jump_diff_multiplicities[clk] += BFIELD_ONE,
                false => {
                    panic!("RAM: clock jump diff {clk} must fit in trace with {num_rows} rows.")
                }
            }
        }
        for clk_jump_diff in clk_jump_diffs_jump_stack.iter() {
            let clk = clk_jump_diff.value() as usize;
            match clk < num_rows {
                true => clk_jump_diff_multiplicities[clk] += BFIELD_ONE,
                false => panic!(
                    "Jump Stack: clock jump diff {clk} must fit in trace with {num_rows} rows."
                ),
            }
        }

        // fill the processor table from the AET and the lookup multiplicities
        let mut processor_table_to_fill =
            processor_table.slice_mut(s![0..aet.processor_trace.nrows(), ..]);
        aet.processor_trace
            .clone()
            .move_into(&mut processor_table_to_fill);
        processor_table_to_fill
            .column_mut(ClockJumpDifferenceLookupMultiplicity.base_table_index())
            .assign(&clk_jump_diff_multiplicities);
    }

    pub fn pad_trace(
        processor_table: &mut ArrayViewMut2<BFieldElement>,
        processor_table_len: usize,
    ) {
        assert!(
            processor_table_len > 0,
            "Processor Table must have at least one row."
        );
        let mut padding_template = processor_table.row(processor_table_len - 1).to_owned();
        padding_template[IsPadding.base_table_index()] = BFieldElement::one();
        padding_template[ClockJumpDifferenceLookupMultiplicity.base_table_index()] =
            BFieldElement::zero();
        processor_table
            .slice_mut(s![processor_table_len.., ..])
            .axis_iter_mut(Axis(0))
            .into_par_iter()
            .for_each(|mut row| row.assign(&padding_template));

        let clk_range = processor_table_len..processor_table.nrows();
        let clk_col = Array1::from_iter(clk_range.map(|a| BFieldElement::new(a as u64)));
        clk_col.move_into(
            processor_table.slice_mut(s![processor_table_len.., CLK.base_table_index()]),
        );

        // The 3 memory-like tables do not have a padding indicator. Hence, clock jump differences
        // are being looked up in their padding sections. The clock jump differences in that
        // section are always 1. The lookup multiplicities of clock value 1 must be increased
        // accordingly: one per padding row, times the number of memory-like tables, which is 3.
        let num_padding_rows = 3 * (processor_table.nrows() - processor_table_len);
        let num_pad_rows = BFieldElement::new(num_padding_rows as u64);
        let mut row_1 = processor_table.row_mut(1);
        row_1[ClockJumpDifferenceLookupMultiplicity.base_table_index()] += num_pad_rows;
    }

    pub fn extend(
        base_table: ArrayView2<BFieldElement>,
        mut ext_table: ArrayViewMut2<XFieldElement>,
        challenges: &Challenges,
    ) {
        assert_eq!(BASE_WIDTH, base_table.ncols());
        assert_eq!(EXT_WIDTH, ext_table.ncols());
        assert_eq!(base_table.nrows(), ext_table.nrows());
        let mut input_table_running_evaluation = EvalArg::default_initial();
        let mut output_table_running_evaluation = EvalArg::default_initial();
        let mut instruction_lookup_log_derivative = LookupArg::default_initial();
        let mut op_stack_table_running_product = PermArg::default_initial();
        let mut ram_table_running_product = PermArg::default_initial();
        let mut jump_stack_running_product = PermArg::default_initial();
        let mut hash_input_running_evaluation = EvalArg::default_initial();
        let mut hash_digest_running_evaluation = EvalArg::default_initial();
        let mut sponge_running_evaluation = EvalArg::default_initial();
        let mut u32_table_running_sum_log_derivative = LookupArg::default_initial();
        let mut clock_jump_diff_lookup_op_stack_log_derivative = LookupArg::default_initial();

        let mut previous_row: Option<ArrayView1<BFieldElement>> = None;
        for row_idx in 0..base_table.nrows() {
            let current_row = base_table.row(row_idx);

            // Input table
            if let Some(prev_row) = previous_row {
                if prev_row[CI.base_table_index()] == Instruction::ReadIo.opcode_b() {
                    let input_symbol = current_row[ST0.base_table_index()];
                    input_table_running_evaluation = input_table_running_evaluation
                        * challenges.get_challenge(StandardInputIndeterminate)
                        + input_symbol;
                }
            }

            // Output table
            if current_row[CI.base_table_index()] == Instruction::WriteIo.opcode_b() {
                let output_symbol = current_row[ST0.base_table_index()];
                output_table_running_evaluation = output_table_running_evaluation
                    * challenges.get_challenge(StandardOutputIndeterminate)
                    + output_symbol;
            }

            // Program table
            if current_row[IsPadding.base_table_index()].is_zero() {
                let ip = current_row[IP.base_table_index()];
                let ci = current_row[CI.base_table_index()];
                let nia = current_row[NIA.base_table_index()];
                let compressed_row_for_instruction_lookup = ip
                    * challenges.get_challenge(ProgramAddressWeight)
                    + ci * challenges.get_challenge(ProgramInstructionWeight)
                    + nia * challenges.get_challenge(ProgramNextInstructionWeight);
                instruction_lookup_log_derivative += (challenges
                    .get_challenge(InstructionLookupIndeterminate)
                    - compressed_row_for_instruction_lookup)
                    .inverse();
            }

            // OpStack table
            let clk = current_row[CLK.base_table_index()];
            let ib1 = current_row[IB1.base_table_index()];
            let osp = current_row[OSP.base_table_index()];
            let osv = current_row[OSV.base_table_index()];
            let compressed_row_for_op_stack_table_permutation_argument = clk
                * challenges.get_challenge(OpStackClkWeight)
                + ib1 * challenges.get_challenge(OpStackIb1Weight)
                + osp * challenges.get_challenge(OpStackOspWeight)
                + osv * challenges.get_challenge(OpStackOsvWeight);
            op_stack_table_running_product *= challenges.get_challenge(OpStackIndeterminate)
                - compressed_row_for_op_stack_table_permutation_argument;

            // RAM Table
            let ramv = current_row[RAMV.base_table_index()];
            let ramp = current_row[RAMP.base_table_index()];
            let previous_instruction = current_row[PreviousInstruction.base_table_index()];
            let compressed_row_for_ram_table_permutation_argument = clk
                * challenges.get_challenge(RamClkWeight)
                + ramp * challenges.get_challenge(RamRampWeight)
                + ramv * challenges.get_challenge(RamRamvWeight)
                + previous_instruction * challenges.get_challenge(RamPreviousInstructionWeight);
            ram_table_running_product *= challenges.get_challenge(RamIndeterminate)
                - compressed_row_for_ram_table_permutation_argument;

            // JumpStack Table
            let ci = current_row[CI.base_table_index()];
            let jsp = current_row[JSP.base_table_index()];
            let jso = current_row[JSO.base_table_index()];
            let jsd = current_row[JSD.base_table_index()];
            let compressed_row_for_jump_stack_table = clk
                * challenges.get_challenge(JumpStackClkWeight)
                + ci * challenges.get_challenge(JumpStackCiWeight)
                + jsp * challenges.get_challenge(JumpStackJspWeight)
                + jso * challenges.get_challenge(JumpStackJsoWeight)
                + jsd * challenges.get_challenge(JumpStackJsdWeight);
            jump_stack_running_product *= challenges.get_challenge(JumpStackIndeterminate)
                - compressed_row_for_jump_stack_table;

            // Hash Table – Hash's input from Processor to Hash Coprocessor
            let st_0_through_9 = [ST0, ST1, ST2, ST3, ST4, ST5, ST6, ST7, ST8, ST9]
                .map(|st| current_row[st.base_table_index()]);
            let hash_state_weights = [
                HashStateWeight0,
                HashStateWeight1,
                HashStateWeight2,
                HashStateWeight3,
                HashStateWeight4,
                HashStateWeight5,
                HashStateWeight6,
                HashStateWeight7,
                HashStateWeight8,
                HashStateWeight9,
            ]
            .map(|id| challenges.get_challenge(id));
            let compressed_row_for_hash_input_and_sponge: XFieldElement = st_0_through_9
                .into_iter()
                .zip_eq(hash_state_weights.into_iter())
                .map(|(st, weight)| weight * st)
                .sum();
            let compressed_row_for_hash_digest: XFieldElement = st_0_through_9[5..=9]
                .iter()
                .zip_eq(hash_state_weights[0..=4].iter())
                .map(|(&st, &weight)| weight * st)
                .sum();

            if current_row[CI.base_table_index()] == Instruction::Hash.opcode_b() {
                hash_input_running_evaluation = hash_input_running_evaluation
                    * challenges.get_challenge(HashInputIndeterminate)
                    + compressed_row_for_hash_input_and_sponge;
            }

            // Hash Table – Hash's output from Hash Coprocessor to Processor
            if let Some(prev_row) = previous_row {
                if prev_row[CI.base_table_index()] == Instruction::Hash.opcode_b() {
                    hash_digest_running_evaluation = hash_digest_running_evaluation
                        * challenges.get_challenge(HashDigestIndeterminate)
                        + compressed_row_for_hash_digest;
                }
            }

            // Hash Table – Sponge
            if let Some(prev_row) = previous_row {
                if prev_row[CI.base_table_index()] == Instruction::AbsorbInit.opcode_b()
                    || prev_row[CI.base_table_index()] == Instruction::Absorb.opcode_b()
                    || prev_row[CI.base_table_index()] == Instruction::Squeeze.opcode_b()
                {
                    sponge_running_evaluation = sponge_running_evaluation
                        * challenges.get_challenge(SpongeIndeterminate)
                        + challenges.get_challenge(HashCIWeight) * prev_row[CI.base_table_index()]
                        + compressed_row_for_hash_input_and_sponge;
                }
            }

            // U32 Table
            if let Some(prev_row) = previous_row {
                let previously_current_instruction = prev_row[CI.base_table_index()];
                if previously_current_instruction == Instruction::Split.opcode_b() {
                    let compressed_row = current_row[ST0.base_table_index()]
                        * challenges.get_challenge(U32LhsWeight)
                        + current_row[ST1.base_table_index()]
                            * challenges.get_challenge(U32RhsWeight)
                        + prev_row[CI.base_table_index()] * challenges.get_challenge(U32CiWeight);
                    u32_table_running_sum_log_derivative +=
                        (challenges.get_challenge(U32Indeterminate) - compressed_row).inverse();
                }
                if previously_current_instruction == Instruction::Lt.opcode_b()
                    || previously_current_instruction == Instruction::And.opcode_b()
                    || previously_current_instruction == Instruction::Pow.opcode_b()
                {
                    let compressed_row = prev_row[ST0.base_table_index()]
                        * challenges.get_challenge(U32LhsWeight)
                        + prev_row[ST1.base_table_index()] * challenges.get_challenge(U32RhsWeight)
                        + prev_row[CI.base_table_index()] * challenges.get_challenge(U32CiWeight)
                        + current_row[ST0.base_table_index()]
                            * challenges.get_challenge(U32ResultWeight);
                    u32_table_running_sum_log_derivative +=
                        (challenges.get_challenge(U32Indeterminate) - compressed_row).inverse();
                }
                if previously_current_instruction == Instruction::Xor.opcode_b() {
                    // Triton VM uses the following equality to compute the results of both the
                    // `and` and `xor` instruction using the u32 coprocessor's `and` capability:
                    //     a ^ b = a + b - 2 · (a & b)
                    // <=> a & b = (a + b - a ^ b) / 2
                    let st0_prev = prev_row[ST0.base_table_index()];
                    let st1_prev = prev_row[ST1.base_table_index()];
                    let st0 = current_row[ST0.base_table_index()];
                    let from_xor_in_processor_to_and_in_u32_coprocessor =
                        (st0_prev + st1_prev - st0) / BFieldElement::new(2);
                    let compressed_row = st0_prev * challenges.get_challenge(U32LhsWeight)
                        + st1_prev * challenges.get_challenge(U32RhsWeight)
                        + Instruction::And.opcode_b() * challenges.get_challenge(U32CiWeight)
                        + from_xor_in_processor_to_and_in_u32_coprocessor
                            * challenges.get_challenge(U32ResultWeight);
                    u32_table_running_sum_log_derivative +=
                        (challenges.get_challenge(U32Indeterminate) - compressed_row).inverse();
                }
                if previously_current_instruction == Instruction::Log2Floor.opcode_b()
                    || previously_current_instruction == Instruction::PopCount.opcode_b()
                {
                    let compressed_row = prev_row[ST0.base_table_index()]
                        * challenges.get_challenge(U32LhsWeight)
                        + prev_row[CI.base_table_index()] * challenges.get_challenge(U32CiWeight)
                        + current_row[ST0.base_table_index()]
                            * challenges.get_challenge(U32ResultWeight);
                    u32_table_running_sum_log_derivative +=
                        (challenges.get_challenge(U32Indeterminate) - compressed_row).inverse();
                }
                if previously_current_instruction == Instruction::Div.opcode_b() {
                    let compressed_row_for_lt_check = current_row[ST0.base_table_index()]
                        * challenges.get_challenge(U32LhsWeight)
                        + prev_row[ST1.base_table_index()] * challenges.get_challenge(U32RhsWeight)
                        + Instruction::Lt.opcode_b() * challenges.get_challenge(U32CiWeight)
                        + BFieldElement::one() * challenges.get_challenge(U32ResultWeight);
                    let compressed_row_for_range_check = prev_row[ST0.base_table_index()]
                        * challenges.get_challenge(U32LhsWeight)
                        + current_row[ST1.base_table_index()]
                            * challenges.get_challenge(U32RhsWeight)
                        + Instruction::Split.opcode_b() * challenges.get_challenge(U32CiWeight);
                    u32_table_running_sum_log_derivative +=
                        (challenges.get_challenge(U32Indeterminate) - compressed_row_for_lt_check)
                            .inverse();
                    u32_table_running_sum_log_derivative += (challenges
                        .get_challenge(U32Indeterminate)
                        - compressed_row_for_range_check)
                        .inverse();
                }
            }

            // Lookup Argument for clock jump differences
            let lookup_multiplicity =
                current_row[ClockJumpDifferenceLookupMultiplicity.base_table_index()];
            clock_jump_diff_lookup_op_stack_log_derivative +=
                (challenges.get_challenge(ClockJumpDifferenceLookupIndeterminate) - clk).inverse()
                    * lookup_multiplicity;

            let mut extension_row = ext_table.row_mut(row_idx);
            extension_row[InputTableEvalArg.ext_table_index()] = input_table_running_evaluation;
            extension_row[OutputTableEvalArg.ext_table_index()] = output_table_running_evaluation;
            extension_row[InstructionLookupClientLogDerivative.ext_table_index()] =
                instruction_lookup_log_derivative;
            extension_row[OpStackTablePermArg.ext_table_index()] = op_stack_table_running_product;
            extension_row[RamTablePermArg.ext_table_index()] = ram_table_running_product;
            extension_row[JumpStackTablePermArg.ext_table_index()] = jump_stack_running_product;
            extension_row[HashInputEvalArg.ext_table_index()] = hash_input_running_evaluation;
            extension_row[HashDigestEvalArg.ext_table_index()] = hash_digest_running_evaluation;
            extension_row[SpongeEvalArg.ext_table_index()] = sponge_running_evaluation;
            extension_row[U32LookupClientLogDerivative.ext_table_index()] =
                u32_table_running_sum_log_derivative;
            extension_row[ClockJumpDifferenceLookupServerLogDerivative.ext_table_index()] =
                clock_jump_diff_lookup_op_stack_log_derivative;
            previous_row = Some(current_row);
        }
    }
}

impl ExtProcessorTable {
    /// Instruction-specific transition constraints are combined with deselectors in such a way
    /// that arbitrary sets of mutually exclusive combinations are summed, i.e.,
    ///
    /// ```py
    /// [ deselector_pop * tc_pop_0 + deselector_push * tc_push_0 + ...,
    ///   deselector_pop * tc_pop_1 + deselector_push * tc_push_1 + ...,
    ///   ...,
    ///   deselector_pop * tc_pop_i + deselector_push * tc_push_i + ...,
    ///   deselector_pop * 0        + deselector_push * tc_push_{i+1} + ...,
    ///   ...,
    /// ]
    /// ```
    /// For instructions that have fewer transition constraints than the maximal number of
    /// transition constraints among all instructions, the deselector is multiplied with a zero,
    /// causing no additional terms in the final sets of combined transition constraint polynomials.
    fn combine_instruction_constraints_with_deselectors(
        factory: &mut DualRowConstraints,
        instr_tc_polys_tuples: [(Instruction, Vec<ConstraintCircuitMonad<DualRowIndicator>>);
            Instruction::COUNT],
    ) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        let (all_instructions, all_tc_polys_for_all_instructions): (Vec<_>, Vec<Vec<_>>) =
            instr_tc_polys_tuples.into_iter().unzip();

        let instruction_deselectors = InstructionDeselectors::new(factory);

        let all_instruction_deselectors = all_instructions
            .into_iter()
            .map(|instr| instruction_deselectors.get(instr))
            .collect_vec();

        let max_number_of_constraints = all_tc_polys_for_all_instructions
            .iter()
            .map(|tc_polys_for_instr| tc_polys_for_instr.len())
            .max()
            .unwrap();

        let zero_poly = factory.zero();
        let all_tc_polys_for_all_instructions_transposed = (0..max_number_of_constraints)
            .map(|idx| {
                all_tc_polys_for_all_instructions
                    .iter()
                    .map(|tc_polys_for_instr| tc_polys_for_instr.get(idx).unwrap_or(&zero_poly))
                    .collect_vec()
            })
            .collect_vec();

        all_tc_polys_for_all_instructions_transposed
            .into_iter()
            .map(|row| {
                all_instruction_deselectors
                    .clone()
                    .into_iter()
                    .zip(row)
                    .map(|(deselector, instruction_tc)| deselector * instruction_tc.to_owned())
                    .sum()
            })
            .collect_vec()
    }

    fn combine_transition_constraints_with_padding_constraints(
        factory: &DualRowConstraints,
        instruction_transition_constraints: Vec<ConstraintCircuitMonad<DualRowIndicator>>,
    ) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        let ip_remains = factory.ip_next() - factory.ip();
        let ci_remains = factory.ci_next() - factory.ci();
        let nia_remains = factory.nia_next() - factory.nia();
        let padding_row_transition_constraints = [
            vec![ip_remains, ci_remains, nia_remains],
            factory.keep_jump_stack(),
            factory.keep_stack(),
            factory.keep_ram(),
        ]
        .concat();

        let padding_row_deselector = factory.one() - factory.is_padding_next();
        let padding_row_selector = factory.is_padding_next();

        let max_number_of_constraints = max(
            instruction_transition_constraints.len(),
            padding_row_transition_constraints.len(),
        );

        (0..max_number_of_constraints)
            .map(|idx| {
                let instruction_constraint = instruction_transition_constraints
                    .get(idx)
                    .unwrap_or(&factory.zero())
                    .clone();
                let padding_constraint = padding_row_transition_constraints
                    .get(idx)
                    .unwrap_or(&factory.zero())
                    .clone();

                instruction_constraint * padding_row_deselector.clone()
                    + padding_constraint * padding_row_selector.clone()
            })
            .collect_vec()
    }
}

#[derive(Debug, Clone)]
pub struct ExtProcessorTable {}

impl ExtProcessorTable {
    pub fn ext_initial_constraints_as_circuits() -> Vec<ConstraintCircuit<SingleRowIndicator>> {
        let factory = SingleRowConstraints::default();
        let constant = |c| factory.constant_from_i32(c);
        let constant_x = |x| factory.constant_from_xfe(x);
        let challenge = |c| factory.circuit_builder.challenge(c);

        let clk_is_0 = factory.clk();
        let ip_is_0 = factory.ip();
        let jsp_is_0 = factory.jsp();
        let jso_is_0 = factory.jso();
        let jsd_is_0 = factory.jsd();
        let st0_is_0 = factory.st0();
        let st1_is_0 = factory.st1();
        let st2_is_0 = factory.st2();
        let st3_is_0 = factory.st3();
        let st4_is_0 = factory.st4();
        let st5_is_0 = factory.st5();
        let st6_is_0 = factory.st6();
        let st7_is_0 = factory.st7();
        let st8_is_0 = factory.st8();
        let st9_is_0 = factory.st9();
        let st10_is_0 = factory.st10();
        let st11_is_0 = factory.st11();
        let st12_is_0 = factory.st12();
        let st13_is_0 = factory.st13();
        let st14_is_0 = factory.st14();
        let st15_is_0 = factory.st15();
        let osp_is_16 = factory.osp() - constant(16);
        let osv_is_0 = factory.osv();
        let ramv_is_0 = factory.ramv();
        let ramp_is_0 = factory.ramp();
        let previous_instruction_is_0 = factory.previous_instruction();

        // Permutation and Evaluation Arguments with all tables the Processor Table relates to

        // standard input
        let running_evaluation_for_standard_input_is_initialized_correctly =
            factory.running_evaluation_standard_input() - constant_x(EvalArg::default_initial());

        // program table
        let instruction_lookup_indeterminate = challenge(InstructionLookupIndeterminate);
        let instruction_ci_weight = challenge(ProgramInstructionWeight);
        let instruction_nia_weight = challenge(ProgramNextInstructionWeight);
        let compressed_row_for_instruction_lookup =
            instruction_ci_weight * factory.ci() + instruction_nia_weight * factory.nia();
        let instruction_lookup_log_derivative_is_initialized_correctly = (factory
            .instruction_lookup_log_derivative()
            - constant_x(LookupArg::default_initial()))
            * (instruction_lookup_indeterminate - compressed_row_for_instruction_lookup)
            - factory.one();

        // standard output
        let running_evaluation_for_standard_output_is_initialized_correctly =
            factory.running_evaluation_standard_output() - constant_x(EvalArg::default_initial());

        // op-stack table
        let op_stack_indeterminate = challenge(OpStackIndeterminate);
        let op_stack_ib1_weight = challenge(OpStackIb1Weight);
        let op_stack_osp_weight = challenge(OpStackOspWeight);
        // note: `clk` and `osv` are already constrained to be 0, `osp` to be 16
        let compressed_row_for_op_stack_table =
            op_stack_ib1_weight * factory.ib1() + op_stack_osp_weight * constant(16);
        let running_product_for_op_stack_table_is_initialized_correctly = factory
            .running_product_op_stack_table()
            - constant_x(PermArg::default_initial())
                * (op_stack_indeterminate - compressed_row_for_op_stack_table);

        // ram table
        let ram_indeterminate = challenge(RamIndeterminate);
        // note: `clk`, `ramp`, and `ramv` are already constrained to be 0.
        let compressed_row_for_ram_table = constant(0);
        let running_product_for_ram_table_is_initialized_correctly = factory
            .running_product_ram_table()
            - constant_x(PermArg::default_initial())
                * (ram_indeterminate - compressed_row_for_ram_table);

        // jump-stack table
        let jump_stack_indeterminate = challenge(JumpStackIndeterminate);
        let jump_stack_ci_weight = challenge(JumpStackCiWeight);
        // note: `clk`, `jsp`, `jso`, and `jsd` are already constrained to be 0.
        let compressed_row_for_jump_stack_table = jump_stack_ci_weight * factory.ci();
        let running_product_for_jump_stack_table_is_initialized_correctly = factory
            .running_product_jump_stack_table()
            - constant_x(PermArg::default_initial())
                * (jump_stack_indeterminate - compressed_row_for_jump_stack_table);

        // clock jump difference lookup argument
        // A clock jump difference of 0 is illegal. Hence, the initial is recorded.
        let clock_jump_diff_lookup_log_derivative_is_initialized_correctly = factory
            .clock_jump_difference_lookup_server_log_derivative()
            - constant_x(LookupArg::default_initial());

        // from processor to hash table
        let hash_selector = factory.ci() - constant(Instruction::Hash.opcode() as i32);
        let hash_deselector =
            InstructionDeselectors::instruction_deselector_single_row(&factory, Instruction::Hash);
        let hash_input_indeterminate = challenge(HashInputIndeterminate);
        // the opStack is guaranteed to be initialized to 0 by virtue of other initial constraints
        let compressed_row = constant(0);
        let running_evaluation_hash_input_has_absorbed_first_row = factory
            .running_evaluation_hash_input()
            - hash_input_indeterminate * constant_x(EvalArg::default_initial())
            - compressed_row;
        let running_evaluation_hash_input_is_default_initial =
            factory.running_evaluation_hash_input() - constant_x(EvalArg::default_initial());
        let running_evaluation_hash_input_is_initialized_correctly = hash_selector
            * running_evaluation_hash_input_is_default_initial
            + hash_deselector * running_evaluation_hash_input_has_absorbed_first_row;

        // from hash table to processor
        let running_evaluation_hash_digest_is_initialized_correctly =
            factory.running_evaluation_hash_digest() - constant_x(EvalArg::default_initial());

        // Hash Table – Sponge
        let running_evaluation_sponge_absorb_is_initialized_correctly =
            factory.running_evaluation_sponge() - constant_x(EvalArg::default_initial());

        // u32 table
        let running_sum_log_derivative_for_u32_table_is_initialized_correctly = factory
            .u32_table_running_sum_log_derivative()
            - constant_x(LookupArg::default_initial());

        let mut constraints = [
            clk_is_0,
            ip_is_0,
            jsp_is_0,
            jso_is_0,
            jsd_is_0,
            st0_is_0,
            st1_is_0,
            st2_is_0,
            st3_is_0,
            st4_is_0,
            st5_is_0,
            st6_is_0,
            st7_is_0,
            st8_is_0,
            st9_is_0,
            st10_is_0,
            st11_is_0,
            st12_is_0,
            st13_is_0,
            st14_is_0,
            st15_is_0,
            osp_is_16,
            osv_is_0,
            ramv_is_0,
            ramp_is_0,
            previous_instruction_is_0,
            running_evaluation_for_standard_input_is_initialized_correctly,
            instruction_lookup_log_derivative_is_initialized_correctly,
            running_evaluation_for_standard_output_is_initialized_correctly,
            running_product_for_op_stack_table_is_initialized_correctly,
            running_product_for_ram_table_is_initialized_correctly,
            running_product_for_jump_stack_table_is_initialized_correctly,
            clock_jump_diff_lookup_log_derivative_is_initialized_correctly,
            running_evaluation_hash_input_is_initialized_correctly,
            running_evaluation_hash_digest_is_initialized_correctly,
            running_evaluation_sponge_absorb_is_initialized_correctly,
            running_sum_log_derivative_for_u32_table_is_initialized_correctly,
        ];

        ConstraintCircuitMonad::constant_folding(&mut constraints);
        constraints.map(|circuit| circuit.consume()).to_vec()
    }

    pub fn ext_consistency_constraints_as_circuits() -> Vec<ConstraintCircuit<SingleRowIndicator>> {
        let factory = SingleRowConstraints::default();
        let one = factory.one();
        let constant = |c| factory.constant_from_i32(c);

        // The composition of instruction buckets ib0-ib7 corresponds the current instruction ci.
        let ib_composition = one.clone() * factory.ib0()
            + constant(1 << 1) * factory.ib1()
            + constant(1 << 2) * factory.ib2()
            + constant(1 << 3) * factory.ib3()
            + constant(1 << 4) * factory.ib4()
            + constant(1 << 5) * factory.ib5()
            + constant(1 << 6) * factory.ib6()
            + constant(1 << 7) * factory.ib7();
        let ci_corresponds_to_ib0_thru_ib7 = factory.ci() - ib_composition;

        let ib0_is_bit = factory.ib0() * (factory.ib0() - one.clone());
        let ib1_is_bit = factory.ib1() * (factory.ib1() - one.clone());
        let ib2_is_bit = factory.ib2() * (factory.ib2() - one.clone());
        let ib3_is_bit = factory.ib3() * (factory.ib3() - one.clone());
        let ib4_is_bit = factory.ib4() * (factory.ib4() - one.clone());
        let ib5_is_bit = factory.ib5() * (factory.ib5() - one.clone());
        let ib6_is_bit = factory.ib6() * (factory.ib6() - one.clone());
        let ib7_is_bit = factory.ib7() * (factory.ib7() - one.clone());
        let is_padding_is_bit = factory.is_padding() * (factory.is_padding() - one);

        // In padding rows, the clock jump difference lookup multiplicity is 0. The one row
        // exempt from this rule is the row wth CLK == 1: since the memory-like tables don't have
        // an “awareness” of padding rows, they keep looking up clock jump differences of
        // magnitude 1.
        let clock_jump_diff_lookup_multiplicity_is_0_in_padding_rows = factory.is_padding()
            * (factory.clk() - factory.one())
            * factory.clock_jump_difference_lookup_multiplicity();

        let mut constraints = [
            ib0_is_bit,
            ib1_is_bit,
            ib2_is_bit,
            ib3_is_bit,
            ib4_is_bit,
            ib5_is_bit,
            ib6_is_bit,
            ib7_is_bit,
            is_padding_is_bit,
            ci_corresponds_to_ib0_thru_ib7,
            clock_jump_diff_lookup_multiplicity_is_0_in_padding_rows,
        ];

        ConstraintCircuitMonad::constant_folding(&mut constraints);
        constraints.map(|circuit| circuit.consume()).to_vec()
    }

    pub fn ext_transition_constraints_as_circuits() -> Vec<ConstraintCircuit<DualRowIndicator>> {
        let mut factory = DualRowConstraints::default();

        // instruction-specific constraints
        let all_instruction_transition_constraints: [_; Instruction::COUNT] = [
            (Pop, factory.instruction_pop()),
            (Push(Default::default()), factory.instruction_push()),
            (Divine(Default::default()), factory.instruction_divine()),
            (Dup(Default::default()), factory.instruction_dup()),
            (Swap(Default::default()), factory.instruction_swap()),
            (Nop, factory.instruction_nop()),
            (Skiz, factory.instruction_skiz()),
            (Call(Default::default()), factory.instruction_call()),
            (Return, factory.instruction_return()),
            (Recurse, factory.instruction_recurse()),
            (Assert, factory.instruction_assert()),
            (Halt, factory.instruction_halt()),
            (ReadMem, factory.instruction_read_mem()),
            (WriteMem, factory.instruction_write_mem()),
            (Hash, factory.instruction_hash()),
            (DivineSibling, factory.instruction_divine_sibling()),
            (AssertVector, factory.instruction_assert_vector()),
            (AbsorbInit, factory.instruction_absorb_init()),
            (Absorb, factory.instruction_absorb()),
            (Squeeze, factory.instruction_squeeze()),
            (Add, factory.instruction_add()),
            (Mul, factory.instruction_mul()),
            (Invert, factory.instruction_invert()),
            (Eq, factory.instruction_eq()),
            (Split, factory.instruction_split()),
            (Lt, factory.instruction_lt()),
            (And, factory.instruction_and()),
            (Xor, factory.instruction_xor()),
            (Log2Floor, factory.instruction_log_2_floor()),
            (Pow, factory.instruction_pow()),
            (Div, factory.instruction_div()),
            (PopCount, factory.instruction_pop_count()),
            (XxAdd, factory.instruction_xxadd()),
            (XxMul, factory.instruction_xxmul()),
            (XInvert, factory.instruction_xinv()),
            (XbMul, factory.instruction_xbmul()),
            (ReadIo, factory.instruction_read_io()),
            (WriteIo, factory.instruction_write_io()),
        ];

        let mut transition_constraints = Self::combine_instruction_constraints_with_deselectors(
            &mut factory,
            all_instruction_transition_constraints,
        );

        // if next row is padding row: disable transition constraints, enable padding constraints
        transition_constraints = Self::combine_transition_constraints_with_padding_constraints(
            &factory,
            transition_constraints,
        );

        // constraints common to all instructions
        transition_constraints.insert(0, factory.clk_always_increases_by_one());
        transition_constraints.insert(1, factory.is_padding_is_zero_or_does_not_change());
        transition_constraints.insert(2, factory.previous_instruction_is_copied_correctly());

        // constraints related to clock jump difference Lookup Argument
        let clock_jump_difference_lookup_indeterminate = factory
            .circuit_builder
            .challenge(ClockJumpDifferenceLookupIndeterminate);
        let log_derivative_accumulates_clk_next = (factory
            .clock_jump_difference_lookup_server_log_derivative_next()
            - factory.clock_jump_difference_lookup_server_log_derivative())
            * (clock_jump_difference_lookup_indeterminate - factory.clk_next())
            - factory.clock_jump_difference_lookup_multiplicity_next();

        transition_constraints.push(log_derivative_accumulates_clk_next);

        // constraints related to evaluation and permutation arguments

        transition_constraints
            .push(factory.running_evaluation_for_standard_input_updates_correctly());
        transition_constraints
            .push(factory.log_derivative_for_instruction_lookup_updates_correctly());
        transition_constraints
            .push(factory.running_evaluation_for_standard_output_updates_correctly());
        transition_constraints.push(factory.running_product_for_op_stack_table_updates_correctly());
        transition_constraints.push(factory.running_product_for_ram_table_updates_correctly());
        transition_constraints
            .push(factory.running_product_for_jump_stack_table_updates_correctly());
        transition_constraints.push(factory.running_evaluation_hash_input_updates_correctly());
        transition_constraints.push(factory.running_evaluation_hash_digest_updates_correctly());
        transition_constraints.push(factory.running_evaluation_sponge_updates_correctly());
        transition_constraints.push(factory.log_derivative_with_u32_table_updates_correctly());

        ConstraintCircuitMonad::constant_folding(&mut transition_constraints);
        transition_constraints
            .into_iter()
            .map(|tc_ref| tc_ref.consume())
            .collect_vec()
    }

    pub fn ext_terminal_constraints_as_circuits() -> Vec<ConstraintCircuit<SingleRowIndicator>> {
        let factory = SingleRowConstraints::default();

        // In the last row, current instruction register ci is 0, corresponding to instruction halt.
        let last_ci_is_halt = factory.ci();

        let mut constraints = [last_ci_is_halt];

        ConstraintCircuitMonad::constant_folding(&mut constraints);
        constraints.map(|circuit| circuit.consume()).to_vec()
    }
}

#[derive(Debug, Clone)]
pub struct SingleRowConstraints {
    base_row_variables: [ConstraintCircuitMonad<SingleRowIndicator>; NUM_BASE_COLUMNS],
    ext_row_variables: [ConstraintCircuitMonad<SingleRowIndicator>; NUM_EXT_COLUMNS],
    circuit_builder: ConstraintCircuitBuilder<SingleRowIndicator>,
    one: ConstraintCircuitMonad<SingleRowIndicator>,
    two: ConstraintCircuitMonad<SingleRowIndicator>,
}

impl Default for SingleRowConstraints {
    fn default() -> Self {
        let circuit_builder = ConstraintCircuitBuilder::new();
        let base_row_variables = (0..NUM_BASE_COLUMNS)
            .map(|i| circuit_builder.input(SingleRowIndicator::BaseRow(i)))
            .collect_vec()
            .try_into()
            .expect("Create variables for single base row constraints");
        let ext_row_variables = (0..NUM_EXT_COLUMNS)
            .map(|i| circuit_builder.input(SingleRowIndicator::ExtRow(i)))
            .collect_vec()
            .try_into()
            .expect("Create variables for single ext row constraints");

        let one = circuit_builder.b_constant(1u32.into());
        let two = circuit_builder.b_constant(2u32.into());

        Self {
            base_row_variables,
            ext_row_variables,
            circuit_builder,
            one,
            two,
        }
    }
}

impl SingleRowConstraints {
    pub fn constant_from_xfe(
        &self,
        constant: XFieldElement,
    ) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.circuit_builder.x_constant(constant)
    }
    pub fn constant_from_i32(&self, constant: i32) -> ConstraintCircuitMonad<SingleRowIndicator> {
        let bfe = if constant < 0 {
            BFieldElement::new(BFieldElement::P - ((-constant) as u64))
        } else {
            BFieldElement::new(constant as u64)
        };
        self.circuit_builder.b_constant(bfe)
    }

    pub fn one(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.one.clone()
    }
    pub fn two(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.two.clone()
    }
    pub fn clk(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[CLK.master_base_table_index()].clone()
    }
    pub fn is_padding(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[IsPadding.master_base_table_index()].clone()
    }
    pub fn ip(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[IP.master_base_table_index()].clone()
    }
    pub fn ci(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[CI.master_base_table_index()].clone()
    }
    pub fn nia(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[NIA.master_base_table_index()].clone()
    }
    pub fn ib0(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[IB0.master_base_table_index()].clone()
    }
    pub fn ib1(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[IB1.master_base_table_index()].clone()
    }
    pub fn ib2(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[IB2.master_base_table_index()].clone()
    }
    pub fn ib3(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[IB3.master_base_table_index()].clone()
    }
    pub fn ib4(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[IB4.master_base_table_index()].clone()
    }
    pub fn ib5(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[IB5.master_base_table_index()].clone()
    }
    pub fn ib6(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[IB6.master_base_table_index()].clone()
    }
    pub fn ib7(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[IB7.master_base_table_index()].clone()
    }
    pub fn jsp(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[JSP.master_base_table_index()].clone()
    }
    pub fn jsd(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[JSD.master_base_table_index()].clone()
    }
    pub fn jso(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[JSO.master_base_table_index()].clone()
    }
    pub fn st0(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[ST0.master_base_table_index()].clone()
    }
    pub fn st1(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[ST1.master_base_table_index()].clone()
    }
    pub fn st2(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[ST2.master_base_table_index()].clone()
    }
    pub fn st3(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[ST3.master_base_table_index()].clone()
    }
    pub fn st4(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[ST4.master_base_table_index()].clone()
    }
    pub fn st5(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[ST5.master_base_table_index()].clone()
    }
    pub fn st6(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[ST6.master_base_table_index()].clone()
    }
    pub fn st7(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[ST7.master_base_table_index()].clone()
    }
    pub fn st8(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[ST8.master_base_table_index()].clone()
    }
    pub fn st9(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[ST9.master_base_table_index()].clone()
    }
    pub fn st10(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[ST10.master_base_table_index()].clone()
    }
    pub fn st11(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[ST11.master_base_table_index()].clone()
    }
    pub fn st12(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[ST12.master_base_table_index()].clone()
    }
    pub fn st13(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[ST13.master_base_table_index()].clone()
    }
    pub fn st14(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[ST14.master_base_table_index()].clone()
    }
    pub fn st15(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[ST15.master_base_table_index()].clone()
    }
    pub fn osp(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[OSP.master_base_table_index()].clone()
    }
    pub fn osv(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[OSV.master_base_table_index()].clone()
    }
    pub fn hv0(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[HV0.master_base_table_index()].clone()
    }
    pub fn hv1(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[HV1.master_base_table_index()].clone()
    }
    pub fn hv2(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[HV2.master_base_table_index()].clone()
    }
    pub fn hv3(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[HV3.master_base_table_index()].clone()
    }
    pub fn ramv(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[RAMV.master_base_table_index()].clone()
    }
    pub fn ramp(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[RAMP.master_base_table_index()].clone()
    }
    pub fn clock_jump_difference_lookup_multiplicity(
        &self,
    ) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[ClockJumpDifferenceLookupMultiplicity.master_base_table_index()]
            .clone()
    }
    pub fn previous_instruction(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.base_row_variables[PreviousInstruction.master_base_table_index()].clone()
    }

    pub fn running_evaluation_standard_input(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.ext_row_variables[InputTableEvalArg.master_ext_table_index()].clone()
    }
    pub fn running_evaluation_standard_output(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.ext_row_variables[OutputTableEvalArg.master_ext_table_index()].clone()
    }
    pub fn instruction_lookup_log_derivative(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.ext_row_variables[InstructionLookupClientLogDerivative.master_ext_table_index()]
            .clone()
    }
    pub fn running_product_op_stack_table(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.ext_row_variables[OpStackTablePermArg.master_ext_table_index()].clone()
    }
    pub fn running_product_ram_table(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.ext_row_variables[RamTablePermArg.master_ext_table_index()].clone()
    }
    pub fn running_product_jump_stack_table(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.ext_row_variables[JumpStackTablePermArg.master_ext_table_index()].clone()
    }
    pub fn clock_jump_difference_lookup_server_log_derivative(
        &self,
    ) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.ext_row_variables
            [ClockJumpDifferenceLookupServerLogDerivative.master_ext_table_index()]
        .clone()
    }
    pub fn running_evaluation_hash_input(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.ext_row_variables[HashInputEvalArg.master_ext_table_index()].clone()
    }
    pub fn running_evaluation_hash_digest(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.ext_row_variables[HashDigestEvalArg.master_ext_table_index()].clone()
    }
    pub fn running_evaluation_sponge(&self) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.ext_row_variables[SpongeEvalArg.master_ext_table_index()].clone()
    }
    pub fn u32_table_running_sum_log_derivative(
        &self,
    ) -> ConstraintCircuitMonad<SingleRowIndicator> {
        self.ext_row_variables[U32LookupClientLogDerivative.master_ext_table_index()].clone()
    }
}

#[derive(Debug, Clone)]
pub struct DualRowConstraints {
    current_base_row_variables: [ConstraintCircuitMonad<DualRowIndicator>; NUM_BASE_COLUMNS],
    current_ext_row_variables: [ConstraintCircuitMonad<DualRowIndicator>; NUM_EXT_COLUMNS],
    next_base_row_variables: [ConstraintCircuitMonad<DualRowIndicator>; NUM_BASE_COLUMNS],
    next_ext_row_variables: [ConstraintCircuitMonad<DualRowIndicator>; NUM_EXT_COLUMNS],
    circuit_builder: ConstraintCircuitBuilder<DualRowIndicator>,
    zero: ConstraintCircuitMonad<DualRowIndicator>,
    one: ConstraintCircuitMonad<DualRowIndicator>,
    two: ConstraintCircuitMonad<DualRowIndicator>,
}

impl Default for DualRowConstraints {
    fn default() -> Self {
        let circuit_builder = ConstraintCircuitBuilder::new();
        let current_base_row_variables = (0..NUM_BASE_COLUMNS)
            .map(|i| circuit_builder.input(DualRowIndicator::CurrentBaseRow(i)))
            .collect_vec()
            .try_into()
            .expect("Create variables for dual rows – current base row");
        let current_ext_row_variables = (0..NUM_EXT_COLUMNS)
            .map(|i| circuit_builder.input(DualRowIndicator::CurrentExtRow(i)))
            .collect_vec()
            .try_into()
            .expect("Create variables for dual rows – current ext row");
        let next_base_row_variables = (0..NUM_BASE_COLUMNS)
            .map(|i| circuit_builder.input(DualRowIndicator::NextBaseRow(i)))
            .collect_vec()
            .try_into()
            .expect("Create variables for dual rows – next base row");
        let next_ext_row_variables = (0..NUM_EXT_COLUMNS)
            .map(|i| circuit_builder.input(DualRowIndicator::NextExtRow(i)))
            .collect_vec()
            .try_into()
            .expect("Create variables for dual rows – next ext row");

        let zero = circuit_builder.b_constant(0u32.into());
        let one = circuit_builder.b_constant(1u32.into());
        let two = circuit_builder.b_constant(2u32.into());

        Self {
            current_base_row_variables,
            current_ext_row_variables,
            next_base_row_variables,
            next_ext_row_variables,
            circuit_builder,
            zero,
            one,
            two,
        }
    }
}

impl DualRowConstraints {
    /// ## The cycle counter (`clk`) always increases by one
    ///
    /// $$
    /// p(..., clk, clk_next, ...) = clk_next - clk - 1
    /// $$
    ///
    /// In general, for all $clk = a$, and $clk_next = a + 1$,
    ///
    /// $$
    /// p(..., a, a+1, ...) = (a+1) - a - 1 = a + 1 - a - 1 = a - a + 1 - 1 = 0
    /// $$
    ///
    /// So the `clk_increase_by_one` base transition constraint polynomial holds exactly
    /// when every `clk` register $a$ is one less than `clk` register $a + 1$.
    pub fn clk_always_increases_by_one(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        let one = self.one();
        let clk = self.clk();
        let clk_next = self.clk_next();

        clk_next - clk - one
    }

    pub fn is_padding_is_zero_or_does_not_change(
        &self,
    ) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.is_padding() * (self.is_padding_next() - self.is_padding())
    }

    pub fn previous_instruction_is_copied_correctly(
        &self,
    ) -> ConstraintCircuitMonad<DualRowIndicator> {
        (self.previous_instruction_next() - self.ci()) * (self.one() - self.is_padding_next())
    }

    pub fn indicator_polynomial(&self, i: usize) -> ConstraintCircuitMonad<DualRowIndicator> {
        let hv0 = self.hv0();
        let hv1 = self.hv1();
        let hv2 = self.hv2();
        let hv3 = self.hv3();

        match i {
            0 => (self.one() - hv3) * (self.one() - hv2) * (self.one() - hv1) * (self.one() - hv0),
            1 => (self.one() - hv3) * (self.one() - hv2) * (self.one() - hv1) * hv0,
            2 => (self.one() - hv3) * (self.one() - hv2) * hv1 * (self.one() - hv0),
            3 => (self.one() - hv3) * (self.one() - hv2) * hv1 * hv0,
            4 => (self.one() - hv3) * hv2 * (self.one() - hv1) * (self.one() - hv0),
            5 => (self.one() - hv3) * hv2 * (self.one() - hv1) * hv0,
            6 => (self.one() - hv3) * hv2 * hv1 * (self.one() - hv0),
            7 => (self.one() - hv3) * hv2 * hv1 * hv0,
            8 => hv3 * (self.one() - hv2) * (self.one() - hv1) * (self.one() - hv0),
            9 => hv3 * (self.one() - hv2) * (self.one() - hv1) * hv0,
            10 => hv3 * (self.one() - hv2) * hv1 * (self.one() - hv0),
            11 => hv3 * (self.one() - hv2) * hv1 * hv0,
            12 => hv3 * hv2 * (self.one() - hv1) * (self.one() - hv0),
            13 => hv3 * hv2 * (self.one() - hv1) * hv0,
            14 => hv3 * hv2 * hv1 * (self.one() - hv0),
            15 => hv3 * hv2 * hv1 * hv0,
            _ => panic!("No indicator polynomial with index {i} exists: there are only 16."),
        }
    }

    pub fn instruction_pop(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        [self.step_1(), self.shrink_stack(), self.keep_ram()].concat()
    }

    /// push'es argument should be on the stack after execution
    /// $st0_next == nia  =>  st0_next - nia == 0$
    pub fn instruction_push(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        let specific_constraints = vec![self.st0_next() - self.nia()];
        [
            specific_constraints,
            self.grow_stack(),
            self.step_2(),
            self.keep_ram(),
        ]
        .concat()
    }

    pub fn instruction_divine(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        [self.step_1(), self.grow_stack(), self.keep_ram()].concat()
    }

    pub fn instruction_dup(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        let specific_constraints = vec![
            self.indicator_polynomial(0) * (self.st0_next() - self.st0()),
            self.indicator_polynomial(1) * (self.st0_next() - self.st1()),
            self.indicator_polynomial(2) * (self.st0_next() - self.st2()),
            self.indicator_polynomial(3) * (self.st0_next() - self.st3()),
            self.indicator_polynomial(4) * (self.st0_next() - self.st4()),
            self.indicator_polynomial(5) * (self.st0_next() - self.st5()),
            self.indicator_polynomial(6) * (self.st0_next() - self.st6()),
            self.indicator_polynomial(7) * (self.st0_next() - self.st7()),
            self.indicator_polynomial(8) * (self.st0_next() - self.st8()),
            self.indicator_polynomial(9) * (self.st0_next() - self.st9()),
            self.indicator_polynomial(10) * (self.st0_next() - self.st10()),
            self.indicator_polynomial(11) * (self.st0_next() - self.st11()),
            self.indicator_polynomial(12) * (self.st0_next() - self.st12()),
            self.indicator_polynomial(13) * (self.st0_next() - self.st13()),
            self.indicator_polynomial(14) * (self.st0_next() - self.st14()),
            self.indicator_polynomial(15) * (self.st0_next() - self.st15()),
        ];
        [
            specific_constraints,
            self.decompose_arg(),
            self.step_2(),
            self.grow_stack(),
            self.keep_ram(),
        ]
        .concat()
    }

    pub fn instruction_swap(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        let specific_constraints = vec![
            self.indicator_polynomial(0),
            self.indicator_polynomial(1) * (self.st1_next() - self.st0()),
            self.indicator_polynomial(2) * (self.st2_next() - self.st0()),
            self.indicator_polynomial(3) * (self.st3_next() - self.st0()),
            self.indicator_polynomial(4) * (self.st4_next() - self.st0()),
            self.indicator_polynomial(5) * (self.st5_next() - self.st0()),
            self.indicator_polynomial(6) * (self.st6_next() - self.st0()),
            self.indicator_polynomial(7) * (self.st7_next() - self.st0()),
            self.indicator_polynomial(8) * (self.st8_next() - self.st0()),
            self.indicator_polynomial(9) * (self.st9_next() - self.st0()),
            self.indicator_polynomial(10) * (self.st10_next() - self.st0()),
            self.indicator_polynomial(11) * (self.st11_next() - self.st0()),
            self.indicator_polynomial(12) * (self.st12_next() - self.st0()),
            self.indicator_polynomial(13) * (self.st13_next() - self.st0()),
            self.indicator_polynomial(14) * (self.st14_next() - self.st0()),
            self.indicator_polynomial(15) * (self.st15_next() - self.st0()),
            self.indicator_polynomial(1) * (self.st0_next() - self.st1()),
            self.indicator_polynomial(2) * (self.st0_next() - self.st2()),
            self.indicator_polynomial(3) * (self.st0_next() - self.st3()),
            self.indicator_polynomial(4) * (self.st0_next() - self.st4()),
            self.indicator_polynomial(5) * (self.st0_next() - self.st5()),
            self.indicator_polynomial(6) * (self.st0_next() - self.st6()),
            self.indicator_polynomial(7) * (self.st0_next() - self.st7()),
            self.indicator_polynomial(8) * (self.st0_next() - self.st8()),
            self.indicator_polynomial(9) * (self.st0_next() - self.st9()),
            self.indicator_polynomial(10) * (self.st0_next() - self.st10()),
            self.indicator_polynomial(11) * (self.st0_next() - self.st11()),
            self.indicator_polynomial(12) * (self.st0_next() - self.st12()),
            self.indicator_polynomial(13) * (self.st0_next() - self.st13()),
            self.indicator_polynomial(14) * (self.st0_next() - self.st14()),
            self.indicator_polynomial(15) * (self.st0_next() - self.st15()),
            (self.one() - self.indicator_polynomial(1)) * (self.st1_next() - self.st1()),
            (self.one() - self.indicator_polynomial(2)) * (self.st2_next() - self.st2()),
            (self.one() - self.indicator_polynomial(3)) * (self.st3_next() - self.st3()),
            (self.one() - self.indicator_polynomial(4)) * (self.st4_next() - self.st4()),
            (self.one() - self.indicator_polynomial(5)) * (self.st5_next() - self.st5()),
            (self.one() - self.indicator_polynomial(6)) * (self.st6_next() - self.st6()),
            (self.one() - self.indicator_polynomial(7)) * (self.st7_next() - self.st7()),
            (self.one() - self.indicator_polynomial(8)) * (self.st8_next() - self.st8()),
            (self.one() - self.indicator_polynomial(9)) * (self.st9_next() - self.st9()),
            (self.one() - self.indicator_polynomial(10)) * (self.st10_next() - self.st10()),
            (self.one() - self.indicator_polynomial(11)) * (self.st11_next() - self.st11()),
            (self.one() - self.indicator_polynomial(12)) * (self.st12_next() - self.st12()),
            (self.one() - self.indicator_polynomial(13)) * (self.st13_next() - self.st13()),
            (self.one() - self.indicator_polynomial(14)) * (self.st14_next() - self.st14()),
            (self.one() - self.indicator_polynomial(15)) * (self.st15_next() - self.st15()),
            self.osv_next() - self.osv(),
            self.osp_next() - self.osp(),
        ];
        [
            specific_constraints,
            self.decompose_arg(),
            self.step_2(),
            self.keep_ram(),
        ]
        .concat()
    }

    pub fn instruction_nop(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        [self.step_1(), self.keep_stack(), self.keep_ram()].concat()
    }

    pub fn instruction_skiz(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        // The next instruction nia is decomposed into helper variables hv.
        let nia_decomposes_to_hvs = self.nia() - (self.hv0() + self.two() * self.hv1());

        // The relevant helper variable hv1 is either 0 or 1.
        // Here, hv0 == 1 means that nia takes an argument.
        let hv0_is_0_or_1 = self.hv0() * (self.hv0() - self.one());

        // If `st0` is non-zero, register `ip` is incremented by 1.
        // If `st0` is 0 and `nia` takes no argument, register `ip` is incremented by 2.
        // If `st0` is 0 and `nia` takes an argument, register `ip` is incremented by 3.
        //
        // Written as Disjunctive Normal Form, the last constraint can be expressed as:
        // 6. (Register `st0` is 0 or `ip` is incremented by 1), and
        // (`st0` has a multiplicative inverse or `hv` is 1 or `ip` is incremented by 2), and
        // (`st0` has a multiplicative inverse or `hv0` is 0 or `ip` is incremented by 3).
        let ip_case_1 = (self.ip_next() - self.ip() - self.one()) * self.st0();
        let ip_case_2 = (self.ip_next() - self.ip() - self.two())
            * (self.st0() * self.hv2() - self.one())
            * (self.hv0() - self.one());
        let ip_case_3 = (self.ip_next() - self.ip() - self.constant(3))
            * (self.st0() * self.hv2() - self.one())
            * self.hv0();
        let ip_incr_by_1_or_2_or_3 = ip_case_1 + ip_case_2 + ip_case_3;

        let specific_constraints =
            vec![nia_decomposes_to_hvs, hv0_is_0_or_1, ip_incr_by_1_or_2_or_3];
        [
            specific_constraints,
            self.keep_jump_stack(),
            self.shrink_stack(),
            self.keep_ram(),
        ]
        .concat()
    }

    pub fn instruction_call(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        // The jump stack pointer jsp is incremented by 1.
        let jsp_incr_1 = self.jsp_next() - (self.jsp() + self.one());

        // The jump's origin jso is set to the current instruction pointer ip plus 2.
        let jso_becomes_ip_plus_2 = self.jso_next() - self.ip() - self.two();

        // The jump's destination jsd is set to the instruction's argument.
        let jsd_becomes_nia = self.jsd_next() - self.nia();

        // The instruction pointer ip is set to the instruction's argument.
        let ip_becomes_nia = self.ip_next() - self.nia();

        let specific_constraints = vec![
            jsp_incr_1,
            jso_becomes_ip_plus_2,
            jsd_becomes_nia,
            ip_becomes_nia,
        ];
        [specific_constraints, self.keep_stack(), self.keep_ram()].concat()
    }

    pub fn instruction_return(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        // The jump stack pointer jsp is decremented by 1.
        let jsp_incr_1 = self.jsp_next() - (self.jsp() - self.one());

        // The instruction pointer ip is set to the last call's origin jso.
        let ip_becomes_jso = self.ip_next() - self.jso();

        let specific_constraints = vec![jsp_incr_1, ip_becomes_jso];
        [specific_constraints, self.keep_stack(), self.keep_ram()].concat()
    }

    pub fn instruction_recurse(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        // The instruction pointer ip is set to the last jump's destination jsd.
        let ip_becomes_jsd = self.ip_next() - self.jsd();
        let specific_constraints = vec![ip_becomes_jsd];
        [
            specific_constraints,
            self.keep_jump_stack(),
            self.keep_stack(),
            self.keep_ram(),
        ]
        .concat()
    }

    pub fn instruction_assert(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        // The current top of the stack st0 is 1.
        let st_0_is_1 = self.st0() - self.one();

        let specific_constraints = vec![st_0_is_1];
        [
            specific_constraints,
            self.step_1(),
            self.shrink_stack(),
            self.keep_ram(),
        ]
        .concat()
    }

    pub fn instruction_halt(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        // The instruction executed in the following step is instruction halt.
        let halt_is_followed_by_halt = self.ci_next() - self.ci();

        let specific_constraints = vec![halt_is_followed_by_halt];
        [
            specific_constraints,
            self.step_1(),
            self.keep_stack(),
            self.keep_ram(),
        ]
        .concat()
    }

    pub fn instruction_read_mem(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        // the RAM pointer is overwritten with st0
        let update_ramp = self.ramp_next() - self.st0();

        // The top of the stack is overwritten with the RAM value.
        let st0_becomes_ramv = self.st0_next() - self.ramv_next();

        let specific_constraints = vec![update_ramp, st0_becomes_ramv];
        [specific_constraints, self.step_1(), self.grow_stack()].concat()
    }

    pub fn instruction_write_mem(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        // the RAM pointer is overwritten with st1
        let update_ramp = self.ramp_next() - self.st1();

        // The RAM value is overwritten with the top of the stack.
        let ramv_becomes_st0 = self.ramv_next() - self.st0();

        let specific_constraints = vec![update_ramp, ramv_becomes_st0];
        [specific_constraints, self.step_1(), self.shrink_stack()].concat()
    }

    /// Two Evaluation Arguments with the Hash Table guarantee correct transition.
    pub fn instruction_hash(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        [
            self.step_1(),
            self.stack_remains_and_top_ten_elements_unconstrained(),
            self.keep_ram(),
        ]
        .concat()
    }

    /// Recall that in a Merkle tree, the indices of left (respectively right)
    /// leafs have 0 (respectively 1) as their least significant bit. The first
    /// two polynomials achieve that helper variable hv0 holds the result of
    /// st10 mod 2. The second polynomial sets the new value of st10 to st10 div 2.
    pub fn instruction_divine_sibling(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        // Helper variable hv0 is either 0 or 1.
        let hv0_is_0_or_1 = self.hv0() * (self.hv0() - self.one());

        // The 11th stack register is shifted by 1 bit to the right.
        let st10_is_shifted_1_bit_right = self.st10_next() * self.two() + self.hv0() - self.st10();

        // The second pentuplet either stays where it is, or is moved to the top
        let maybe_move_st5 = (self.one() - self.hv0()) * (self.st5() - self.st0_next())
            + self.hv0() * (self.st5() - self.st5_next());
        let maybe_move_st6 = (self.one() - self.hv0()) * (self.st6() - self.st1_next())
            + self.hv0() * (self.st6() - self.st6_next());
        let maybe_move_st7 = (self.one() - self.hv0()) * (self.st7() - self.st2_next())
            + self.hv0() * (self.st7() - self.st7_next());
        let maybe_move_st8 = (self.one() - self.hv0()) * (self.st8() - self.st3_next())
            + self.hv0() * (self.st8() - self.st8_next());
        let maybe_move_st9 = (self.one() - self.hv0()) * (self.st9() - self.st4_next())
            + self.hv0() * (self.st9() - self.st9_next());

        let specific_constraints = vec![
            hv0_is_0_or_1,
            st10_is_shifted_1_bit_right,
            maybe_move_st5,
            maybe_move_st6,
            maybe_move_st7,
            maybe_move_st8,
            maybe_move_st9,
        ];
        [
            specific_constraints,
            self.stack_remains_and_top_eleven_elements_unconstrained(),
            self.step_1(),
            self.keep_ram(),
        ]
        .concat()
    }

    pub fn instruction_assert_vector(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        let specific_constraints = vec![
            // Register st0 is equal to st5.
            self.st5() - self.st0(),
            // Register st1 is equal to st6.
            self.st6() - self.st1(),
            // and so on
            self.st7() - self.st2(),
            self.st8() - self.st3(),
            self.st9() - self.st4(),
        ];
        [
            specific_constraints,
            self.step_1(),
            self.keep_stack(),
            self.keep_ram(),
        ]
        .concat()
    }

    pub fn instruction_absorb_init(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        // no further constraints
        let specific_constraints = vec![];
        [
            specific_constraints,
            self.step_1(),
            self.keep_stack(),
            self.keep_ram(),
        ]
        .concat()
    }

    pub fn instruction_absorb(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        // no further constraints
        let specific_constraints = vec![];
        [
            specific_constraints,
            self.step_1(),
            self.keep_stack(),
            self.keep_ram(),
        ]
        .concat()
    }

    pub fn instruction_squeeze(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        // no further constraints
        let specific_constraints = vec![];
        [
            specific_constraints,
            self.step_1(),
            self.stack_remains_and_top_ten_elements_unconstrained(),
            self.keep_ram(),
        ]
        .concat()
    }

    /// The sum of the top two stack elements is moved into the top of the stack.
    ///
    /// $st0' - (st0 + st1) = 0$
    pub fn instruction_add(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        let specific_constraints = vec![self.st0_next() - (self.st0() + self.st1())];
        [
            specific_constraints,
            self.step_1(),
            self.binop(),
            self.keep_ram(),
        ]
        .concat()
    }

    /// The product of the top two stack elements is moved into the top of the stack.
    ///
    /// $st0' - (st0 * st1) = 0$
    pub fn instruction_mul(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        let specific_constraints = vec![self.st0_next() - (self.st0() * self.st1())];
        [
            specific_constraints,
            self.step_1(),
            self.binop(),
            self.keep_ram(),
        ]
        .concat()
    }

    /// The top of the stack's inverse is moved into the top of the stack.
    ///
    /// $st0'·st0 - 1 = 0$
    pub fn instruction_invert(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        let specific_constraints = vec![self.st0_next() * self.st0() - self.one()];
        [
            specific_constraints,
            self.step_1(),
            self.unop(),
            self.keep_ram(),
        ]
        .concat()
    }

    pub fn instruction_eq(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        // Helper variable hv0 is the inverse of the difference of the stack's two top-most elements or 0.
        //
        // $ hv0·(hv0·(st1 - st0) - 1) = 0 $
        let hv0_is_inverse_of_diff_or_hv0_is_0 =
            self.hv0() * (self.hv0() * (self.st1() - self.st0()) - self.one());

        // Helper variable hv0 is the inverse of the difference of the stack's two top-most elements or the difference is 0.
        //
        // $ (st1 - st0)·(hv0·(st1 - st0) - 1) = 0 $
        let hv0_is_inverse_of_diff_or_diff_is_0 =
            (self.st1() - self.st0()) * (self.hv0() * (self.st1() - self.st0()) - self.one());

        // The new top of the stack is 1 if the difference between the stack's two top-most elements is not invertible, 0 otherwise.
        //
        // $ st0' - (1 - hv0·(st1 - st0)) = 0 $
        let st0_becomes_1_if_diff_is_not_invertible =
            self.st0_next() - (self.one() - self.hv0() * (self.st1() - self.st0()));

        let specific_constraints = vec![
            hv0_is_inverse_of_diff_or_hv0_is_0,
            hv0_is_inverse_of_diff_or_diff_is_0,
            st0_becomes_1_if_diff_is_not_invertible,
        ];
        [
            specific_constraints,
            self.step_1(),
            self.binop(),
            self.keep_ram(),
        ]
        .concat()
    }

    pub fn instruction_split(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        let two_pow_32 = self.constant_b(BFieldElement::new(1_u64 << 32));

        // The top of the stack is decomposed as 32-bit chunks into the stack's top-most elements.
        //
        // $st0 - (2^32·st0' + st1') = 0$
        let st0_decomposes_to_two_32_bit_chunks =
            self.st0() - (two_pow_32.clone() * self.st1_next() + self.st0_next());

        // Helper variable `hv0` = 0 if either
        // 1. `hv0` is the difference between (2^32 - 1) and the high 32 bits (`st0'`), or
        // 1. the low 32 bits (`st1'`) are 0.
        //
        // st1'·(hv0·(st0' - (2^32 - 1)) - 1)
        //   lo·(hv0·(hi - 0xffff_ffff)) - 1)
        let hv0_holds_inverse_of_chunk_difference_or_low_bits_are_0 = {
            let hv0 = self.hv0();
            let hi = self.st1_next();
            let lo = self.st0_next();
            let ffff_ffff = two_pow_32 - self.one();

            lo * (hv0 * (hi - ffff_ffff) - self.one())
        };

        let specific_constraints = vec![
            st0_decomposes_to_two_32_bit_chunks,
            hv0_holds_inverse_of_chunk_difference_or_low_bits_are_0,
        ];
        [
            specific_constraints,
            self.grow_stack_and_top_two_elements_unconstrained(),
            self.step_1(),
            self.keep_ram(),
        ]
        .concat()
    }

    pub fn instruction_lt(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        // no further constraints
        let specific_constraints = vec![];
        [
            specific_constraints,
            self.step_1(),
            self.binop(),
            self.keep_ram(),
        ]
        .concat()
    }

    pub fn instruction_and(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        // no further constraints
        let specific_constraints = vec![];
        [
            specific_constraints,
            self.step_1(),
            self.binop(),
            self.keep_ram(),
        ]
        .concat()
    }

    pub fn instruction_xor(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        // no further constraints
        let specific_constraints = vec![];
        [
            specific_constraints,
            self.step_1(),
            self.binop(),
            self.keep_ram(),
        ]
        .concat()
    }

    pub fn instruction_log_2_floor(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        // no further constraints
        let specific_constraints = vec![];
        [
            specific_constraints,
            self.step_1(),
            self.unop(),
            self.keep_ram(),
        ]
        .concat()
    }

    pub fn instruction_pow(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        // no further constraints
        let specific_constraints = vec![];
        [
            specific_constraints,
            self.step_1(),
            self.binop(),
            self.keep_ram(),
        ]
        .concat()
    }

    pub fn instruction_div(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        // `n == d·q + r` means `st0 - st1·st1' - st0'`
        let numerator_is_quotient_times_denominator_plus_remainder =
            self.st0() - self.st1() * self.st1_next() - self.st0_next();

        let st2_does_not_change = self.st2_next() - self.st2();

        let specific_constraints = vec![
            numerator_is_quotient_times_denominator_plus_remainder,
            st2_does_not_change,
        ];
        [
            specific_constraints,
            self.step_1(),
            self.stack_remains_and_top_three_elements_unconstrained(),
            self.keep_ram(),
        ]
        .concat()
    }

    pub fn instruction_pop_count(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        // no further constraints
        let specific_constraints = vec![];
        [
            specific_constraints,
            self.step_1(),
            self.unop(),
            self.keep_ram(),
        ]
        .concat()
    }

    pub fn instruction_xxadd(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        // The result of adding st0 to st3 is moved into st0.
        let st0_becomes_st0_plus_st3 = self.st0_next() - (self.st0() + self.st3());

        // The result of adding st1 to st4 is moved into st1.
        let st1_becomes_st1_plus_st4 = self.st1_next() - (self.st1() + self.st4());

        // The result of adding st2 to st5 is moved into st2.
        let st2_becomes_st2_plus_st5 = self.st2_next() - (self.st2() + self.st5());

        let specific_constraints = vec![
            st0_becomes_st0_plus_st3,
            st1_becomes_st1_plus_st4,
            st2_becomes_st2_plus_st5,
        ];
        [
            specific_constraints,
            self.stack_remains_and_top_three_elements_unconstrained(),
            self.step_1(),
            self.keep_ram(),
        ]
        .concat()
    }

    pub fn instruction_xxmul(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        // The coefficient of x^0 of multiplying the two X-Field elements on the stack is moved into st0.
        //
        // $st0' - (st0·st3 - st2·st4 - st1·st5)$
        let st0_becomes_coefficient_0 = self.st0_next()
            - (self.st0() * self.st3() - self.st2() * self.st4() - self.st1() * self.st5());

        // The coefficient of x^1 of multiplying the two X-Field elements on the stack is moved into st1.
        //
        // st1' - (st1·st3 + st0·st4 - st2·st5 + st2·st4 + st1·st5)
        let st1_becomes_coefficient_1 = self.st1_next()
            - (self.st1() * self.st3() + self.st0() * self.st4() - self.st2() * self.st5()
                + self.st2() * self.st4()
                + self.st1() * self.st5());

        // The coefficient of x^2 of multiplying the two X-Field elements on the stack is moved into st2.
        //
        // st2' - (st2·st3 + st1·st4 + st0·st5 + st2·st5)
        let st2_becomes_coefficient_2 = self.st2_next()
            - (self.st2() * self.st3()
                + self.st1() * self.st4()
                + self.st0() * self.st5()
                + self.st2() * self.st5());

        let specific_constraints = vec![
            st0_becomes_coefficient_0,
            st1_becomes_coefficient_1,
            st2_becomes_coefficient_2,
        ];
        [
            specific_constraints,
            self.stack_remains_and_top_three_elements_unconstrained(),
            self.step_1(),
            self.keep_ram(),
        ]
        .concat()
    }

    pub fn instruction_xinv(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        // The coefficient of x^0 of multiplying X-Field element on top of the current stack and on top of the next stack is 1.
        //
        // $st0·st0' - st2·st1' - st1·st2' - 1 = 0$
        let first_coefficient_of_product_of_element_and_inverse_is_1 = self.st0() * self.st0_next()
            - self.st2() * self.st1_next()
            - self.st1() * self.st2_next()
            - self.one();

        // The coefficient of x^1 of multiplying X-Field element on top of the current stack and on top of the next stack is 0.
        //
        // $st1·st0' + st0·st1' - st2·st2' + st2·st1' + st1·st2' = 0$
        let second_coefficient_of_product_of_element_and_inverse_is_0 =
            self.st1() * self.st0_next() + self.st0() * self.st1_next()
                - self.st2() * self.st2_next()
                + self.st2() * self.st1_next()
                + self.st1() * self.st2_next();

        // The coefficient of x^2 of multiplying X-Field element on top of the current stack and on top of the next stack is 0.
        //
        // $st2·st0' + st1·st1' + st0·st2' + st2·st2' = 0$
        let third_coefficient_of_product_of_element_and_inverse_is_0 = self.st2() * self.st0_next()
            + self.st1() * self.st1_next()
            + self.st0() * self.st2_next()
            + self.st2() * self.st2_next();

        let specific_constraints = vec![
            first_coefficient_of_product_of_element_and_inverse_is_1,
            second_coefficient_of_product_of_element_and_inverse_is_0,
            third_coefficient_of_product_of_element_and_inverse_is_0,
        ];
        [
            specific_constraints,
            self.stack_remains_and_top_three_elements_unconstrained(),
            self.step_1(),
            self.keep_ram(),
        ]
        .concat()
    }

    pub fn instruction_xbmul(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        // The result of multiplying the top of the stack with the X-Field element's coefficient for x^0 is moved into st0.
        //
        // st0' - st0·st1
        let first_coeff_scalar_multiplication = self.st0_next() - self.st0() * self.st1();

        // The result of multiplying the top of the stack with the X-Field element's coefficient for x^1 is moved into st1.
        //
        // st1' - st0·st2
        let secnd_coeff_scalar_multiplication = self.st1_next() - self.st0() * self.st2();

        // The result of multiplying the top of the stack with the X-Field element's coefficient for x^2 is moved into st2.
        //
        // st2' - st0·st3
        let third_coeff_scalar_multiplication = self.st2_next() - self.st0() * self.st3();

        let specific_constraints = vec![
            first_coeff_scalar_multiplication,
            secnd_coeff_scalar_multiplication,
            third_coeff_scalar_multiplication,
        ];
        [
            specific_constraints,
            self.stack_shrinks_and_top_three_elements_unconstrained(),
            self.step_1(),
            self.keep_ram(),
        ]
        .concat()
    }

    /// This instruction has no additional transition constraints.
    ///
    /// An Evaluation Argument with the list of input symbols guarantees correct transition.
    pub fn instruction_read_io(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        [self.step_1(), self.grow_stack(), self.keep_ram()].concat()
    }

    /// This instruction has no additional transition constraints.
    ///
    /// An Evaluation Argument with the list of output symbols guarantees correct transition.
    pub fn instruction_write_io(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        [self.step_1(), self.shrink_stack(), self.keep_ram()].concat()
    }

    pub fn zero(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.zero.clone()
    }

    pub fn one(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.one.clone()
    }

    pub fn two(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.two.clone()
    }

    pub fn constant(&self, constant: u32) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.circuit_builder.b_constant(constant.into())
    }

    pub fn constant_b(&self, constant: BFieldElement) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.circuit_builder.b_constant(constant)
    }

    pub fn clk(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[CLK.master_base_table_index()].clone()
    }

    pub fn ip(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[IP.master_base_table_index()].clone()
    }

    pub fn ci(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[CI.master_base_table_index()].clone()
    }

    pub fn nia(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[NIA.master_base_table_index()].clone()
    }

    pub fn ib0(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[IB0.master_base_table_index()].clone()
    }

    pub fn ib1(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[IB1.master_base_table_index()].clone()
    }

    pub fn ib2(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[IB2.master_base_table_index()].clone()
    }

    pub fn ib3(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[IB3.master_base_table_index()].clone()
    }

    pub fn ib4(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[IB4.master_base_table_index()].clone()
    }

    pub fn ib5(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[IB5.master_base_table_index()].clone()
    }

    pub fn ib6(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[IB6.master_base_table_index()].clone()
    }

    pub fn ib7(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[IB7.master_base_table_index()].clone()
    }

    pub fn jsp(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[JSP.master_base_table_index()].clone()
    }

    pub fn jsd(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[JSD.master_base_table_index()].clone()
    }

    pub fn jso(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[JSO.master_base_table_index()].clone()
    }

    pub fn st0(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[ST0.master_base_table_index()].clone()
    }

    pub fn st1(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[ST1.master_base_table_index()].clone()
    }

    pub fn st2(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[ST2.master_base_table_index()].clone()
    }

    pub fn st3(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[ST3.master_base_table_index()].clone()
    }

    pub fn st4(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[ST4.master_base_table_index()].clone()
    }

    pub fn st5(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[ST5.master_base_table_index()].clone()
    }

    pub fn st6(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[ST6.master_base_table_index()].clone()
    }

    pub fn st7(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[ST7.master_base_table_index()].clone()
    }

    pub fn st8(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[ST8.master_base_table_index()].clone()
    }

    pub fn st9(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[ST9.master_base_table_index()].clone()
    }

    pub fn st10(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[ST10.master_base_table_index()].clone()
    }

    pub fn st11(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[ST11.master_base_table_index()].clone()
    }

    pub fn st12(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[ST12.master_base_table_index()].clone()
    }

    pub fn st13(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[ST13.master_base_table_index()].clone()
    }

    pub fn st14(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[ST14.master_base_table_index()].clone()
    }

    pub fn st15(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[ST15.master_base_table_index()].clone()
    }

    pub fn osp(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[OSP.master_base_table_index()].clone()
    }

    pub fn osv(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[OSV.master_base_table_index()].clone()
    }

    pub fn hv0(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[HV0.master_base_table_index()].clone()
    }

    pub fn hv1(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[HV1.master_base_table_index()].clone()
    }

    pub fn hv2(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[HV2.master_base_table_index()].clone()
    }

    pub fn hv3(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[HV3.master_base_table_index()].clone()
    }

    pub fn ramp(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[RAMP.master_base_table_index()].clone()
    }

    pub fn previous_instruction(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[PreviousInstruction.master_base_table_index()].clone()
    }

    pub fn ramv(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[RAMV.master_base_table_index()].clone()
    }

    pub fn is_padding(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_base_row_variables[IsPadding.master_base_table_index()].clone()
    }

    pub fn running_evaluation_standard_input(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_ext_row_variables[InputTableEvalArg.master_ext_table_index()].clone()
    }
    pub fn running_evaluation_standard_output(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_ext_row_variables[OutputTableEvalArg.master_ext_table_index()].clone()
    }
    pub fn instruction_lookup_log_derivative(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_ext_row_variables
            [InstructionLookupClientLogDerivative.master_ext_table_index()]
        .clone()
    }
    pub fn running_product_op_stack_table(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_ext_row_variables[OpStackTablePermArg.master_ext_table_index()].clone()
    }
    pub fn running_product_ram_table(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_ext_row_variables[RamTablePermArg.master_ext_table_index()].clone()
    }
    pub fn running_product_jump_stack_table(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_ext_row_variables[JumpStackTablePermArg.master_ext_table_index()].clone()
    }
    pub fn clock_jump_difference_lookup_server_log_derivative(
        &self,
    ) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_ext_row_variables
            [ClockJumpDifferenceLookupServerLogDerivative.master_ext_table_index()]
        .clone()
    }
    pub fn running_evaluation_hash_input(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_ext_row_variables[HashInputEvalArg.master_ext_table_index()].clone()
    }
    pub fn running_evaluation_hash_digest(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_ext_row_variables[HashDigestEvalArg.master_ext_table_index()].clone()
    }
    pub fn running_evaluation_sponge(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_ext_row_variables[SpongeEvalArg.master_ext_table_index()].clone()
    }
    pub fn u32_table_running_sum_log_derivative(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.current_ext_row_variables[U32LookupClientLogDerivative.master_ext_table_index()]
            .clone()
    }

    // Property: All polynomial variables that contain '_next' have the same
    // variable position / value as the one without '_next', +/- NUM_COLUMNS.
    pub fn clk_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[CLK.master_base_table_index()].clone()
    }

    pub fn ip_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[IP.master_base_table_index()].clone()
    }

    pub fn ci_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[CI.master_base_table_index()].clone()
    }

    pub fn nia_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[NIA.master_base_table_index()].clone()
    }

    pub fn ib0_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[IB0.master_base_table_index()].clone()
    }
    pub fn ib1_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[IB1.master_base_table_index()].clone()
    }
    pub fn ib2_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[IB2.master_base_table_index()].clone()
    }
    pub fn ib3_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[IB3.master_base_table_index()].clone()
    }
    pub fn ib4_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[IB4.master_base_table_index()].clone()
    }
    pub fn ib5_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[IB5.master_base_table_index()].clone()
    }
    pub fn ib6_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[IB6.master_base_table_index()].clone()
    }
    pub fn ib7_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[IB7.master_base_table_index()].clone()
    }

    pub fn jsp_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[JSP.master_base_table_index()].clone()
    }

    pub fn jsd_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[JSD.master_base_table_index()].clone()
    }

    pub fn jso_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[JSO.master_base_table_index()].clone()
    }

    pub fn st0_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[ST0.master_base_table_index()].clone()
    }

    pub fn st1_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[ST1.master_base_table_index()].clone()
    }

    pub fn st2_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[ST2.master_base_table_index()].clone()
    }

    pub fn st3_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[ST3.master_base_table_index()].clone()
    }

    pub fn st4_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[ST4.master_base_table_index()].clone()
    }

    pub fn st5_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[ST5.master_base_table_index()].clone()
    }

    pub fn st6_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[ST6.master_base_table_index()].clone()
    }

    pub fn st7_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[ST7.master_base_table_index()].clone()
    }

    pub fn st8_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[ST8.master_base_table_index()].clone()
    }

    pub fn st9_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[ST9.master_base_table_index()].clone()
    }

    pub fn st10_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[ST10.master_base_table_index()].clone()
    }

    pub fn st11_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[ST11.master_base_table_index()].clone()
    }

    pub fn st12_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[ST12.master_base_table_index()].clone()
    }

    pub fn st13_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[ST13.master_base_table_index()].clone()
    }

    pub fn st14_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[ST14.master_base_table_index()].clone()
    }

    pub fn st15_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[ST15.master_base_table_index()].clone()
    }

    pub fn osp_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[OSP.master_base_table_index()].clone()
    }

    pub fn osv_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[OSV.master_base_table_index()].clone()
    }

    pub fn ramp_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[RAMP.master_base_table_index()].clone()
    }

    pub fn previous_instruction_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[PreviousInstruction.master_base_table_index()].clone()
    }

    pub fn ramv_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[RAMV.master_base_table_index()].clone()
    }

    pub fn clock_jump_difference_lookup_multiplicity_next(
        &self,
    ) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables
            [ClockJumpDifferenceLookupMultiplicity.master_base_table_index()]
        .clone()
    }

    pub fn is_padding_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_base_row_variables[IsPadding.master_base_table_index()].clone()
    }

    pub fn running_evaluation_standard_input_next(
        &self,
    ) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_ext_row_variables[InputTableEvalArg.master_ext_table_index()].clone()
    }
    pub fn running_evaluation_standard_output_next(
        &self,
    ) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_ext_row_variables[OutputTableEvalArg.master_ext_table_index()].clone()
    }
    pub fn instruction_lookup_log_derivative_next(
        &self,
    ) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_ext_row_variables[InstructionLookupClientLogDerivative.master_ext_table_index()]
            .clone()
    }
    pub fn running_product_op_stack_table_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_ext_row_variables[OpStackTablePermArg.master_ext_table_index()].clone()
    }
    pub fn running_product_ram_table_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_ext_row_variables[RamTablePermArg.master_ext_table_index()].clone()
    }
    pub fn running_product_jump_stack_table_next(
        &self,
    ) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_ext_row_variables[JumpStackTablePermArg.master_ext_table_index()].clone()
    }
    pub fn clock_jump_difference_lookup_server_log_derivative_next(
        &self,
    ) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_ext_row_variables
            [ClockJumpDifferenceLookupServerLogDerivative.master_ext_table_index()]
        .clone()
    }
    pub fn running_evaluation_hash_input_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_ext_row_variables[HashInputEvalArg.master_ext_table_index()].clone()
    }
    pub fn running_evaluation_hash_digest_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_ext_row_variables[HashDigestEvalArg.master_ext_table_index()].clone()
    }
    pub fn running_evaluation_sponge_next(&self) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_ext_row_variables[SpongeEvalArg.master_ext_table_index()].clone()
    }
    pub fn u32_table_running_sum_log_derivative_next(
        &self,
    ) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.next_ext_row_variables[U32LookupClientLogDerivative.master_ext_table_index()].clone()
    }

    pub fn decompose_arg(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        let hv0_is_a_bit = self.hv0() * (self.hv0() - self.one());
        let hv1_is_a_bit = self.hv1() * (self.hv1() - self.one());
        let hv2_is_a_bit = self.hv2() * (self.hv2() - self.one());
        let hv3_is_a_bit = self.hv3() * (self.hv3() - self.one());
        let helper_variables_are_binary_decomposition_of_nia = self.nia()
            - self.constant(8) * self.hv3()
            - self.constant(4) * self.hv2()
            - self.constant(2) * self.hv1()
            - self.hv0();
        vec![
            hv0_is_a_bit,
            hv1_is_a_bit,
            hv2_is_a_bit,
            hv3_is_a_bit,
            helper_variables_are_binary_decomposition_of_nia,
        ]
    }

    pub fn keep_jump_stack(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        let jsp_does_not_change = self.jsp_next() - self.jsp();
        let jso_does_not_change = self.jso_next() - self.jso();
        let jsd_does_not_change = self.jsd_next() - self.jsd();
        vec![
            jsp_does_not_change,
            jso_does_not_change,
            jsd_does_not_change,
        ]
    }

    pub fn step_1(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        let instruction_pointer_increases_by_one = self.ip_next() - self.ip() - self.one();
        let specific_constraints = vec![instruction_pointer_increases_by_one];
        [specific_constraints, self.keep_jump_stack()].concat()
    }

    pub fn step_2(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        let instruction_pointer_increases_by_two = self.ip_next() - self.ip() - self.two();
        let specific_constraints = vec![instruction_pointer_increases_by_two];
        [specific_constraints, self.keep_jump_stack()].concat()
    }

    pub fn grow_stack_and_top_two_elements_unconstrained(
        &self,
    ) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        vec![
            // The stack element in st1 is moved into st2.
            self.st2_next() - self.st1(),
            // And so on...
            self.st3_next() - self.st2(),
            self.st4_next() - self.st3(),
            self.st5_next() - self.st4(),
            self.st6_next() - self.st5(),
            self.st7_next() - self.st6(),
            self.st8_next() - self.st7(),
            self.st9_next() - self.st8(),
            self.st10_next() - self.st9(),
            self.st11_next() - self.st10(),
            self.st12_next() - self.st11(),
            self.st13_next() - self.st12(),
            self.st14_next() - self.st13(),
            self.st15_next() - self.st14(),
            // The stack element in st15 is moved to the top of OpStack underflow, i.e., osv.
            self.osv_next() - self.st15(),
            // The OpStack pointer is incremented by 1.
            self.osp_next() - (self.osp() + self.one()),
        ]
    }

    pub fn grow_stack(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        let specific_constraints = vec![
            // The stack element in st0 is moved into st1.
            self.st1_next() - self.st0(),
        ];
        [
            specific_constraints,
            self.grow_stack_and_top_two_elements_unconstrained(),
        ]
        .concat()
    }

    pub fn stack_shrinks_and_top_three_elements_unconstrained(
        &self,
    ) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        vec![
            // The stack element in st4 is moved into st3.
            self.st3_next() - self.st4(),
            // The stack element in st5 is moved into st4.
            self.st4_next() - self.st5(),
            // And so on...
            self.st5_next() - self.st6(),
            self.st6_next() - self.st7(),
            self.st7_next() - self.st8(),
            self.st8_next() - self.st9(),
            self.st9_next() - self.st10(),
            self.st10_next() - self.st11(),
            self.st11_next() - self.st12(),
            self.st12_next() - self.st13(),
            self.st13_next() - self.st14(),
            self.st14_next() - self.st15(),
            // The stack element at the top of OpStack underflow, i.e., osv, is moved into st15.
            self.st15_next() - self.osv(),
            // The OpStack pointer, osp, is decremented by 1.
            self.osp_next() - (self.osp() - self.one()),
            // The helper variable register hv3 holds the inverse of (osp - 16).
            (self.osp() - self.constant(16)) * self.hv3() - self.one(),
        ]
    }

    pub fn binop(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        let specific_constraints = vec![
            // The stack element in st2 is moved into st1.
            self.st1_next() - self.st2(),
            // The stack element in st3 is moved into st2.
            self.st2_next() - self.st3(),
        ];
        [
            specific_constraints,
            self.stack_shrinks_and_top_three_elements_unconstrained(),
        ]
        .concat()
    }

    pub fn shrink_stack(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        let specific_constrants = vec![self.st0_next() - self.st1()];
        [specific_constrants, self.binop()].concat()
    }

    pub fn stack_remains_and_top_eleven_elements_unconstrained(
        &self,
    ) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        vec![
            self.st11_next() - self.st11(),
            self.st12_next() - self.st12(),
            self.st13_next() - self.st13(),
            self.st14_next() - self.st14(),
            self.st15_next() - self.st15(),
            // The top of the OpStack underflow, i.e., osv, does not change.
            self.osv_next() - self.osv(),
            // The OpStack pointer, osp, does not change.
            self.osp_next() - self.osp(),
        ]
    }

    pub fn stack_remains_and_top_ten_elements_unconstrained(
        &self,
    ) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        let specific_constraints = vec![self.st10_next() - self.st10()];
        [
            specific_constraints,
            self.stack_remains_and_top_eleven_elements_unconstrained(),
        ]
        .concat()
    }

    pub fn stack_remains_and_top_three_elements_unconstrained(
        &self,
    ) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        let specific_constraints = vec![
            self.st3_next() - self.st3(),
            self.st4_next() - self.st4(),
            self.st5_next() - self.st5(),
            self.st6_next() - self.st6(),
            self.st7_next() - self.st7(),
            self.st8_next() - self.st8(),
            self.st9_next() - self.st9(),
        ];
        [
            specific_constraints,
            self.stack_remains_and_top_ten_elements_unconstrained(),
        ]
        .concat()
    }

    pub fn unop(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        let specific_constraints = vec![
            // The stack element in st1 does not change.
            self.st1_next() - self.st1(),
            // The stack element in st2 does not change.
            self.st2_next() - self.st2(),
        ];
        [
            specific_constraints,
            self.stack_remains_and_top_three_elements_unconstrained(),
        ]
        .concat()
    }

    pub fn keep_stack(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        let specific_constraints = vec![self.st0_next() - self.st0()];
        [specific_constraints, self.unop()].concat()
    }

    pub fn keep_ram(&self) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        vec![
            self.ramv_next() - self.ramv(),
            self.ramp_next() - self.ramp(),
        ]
    }

    pub fn running_evaluation_for_standard_input_updates_correctly(
        &self,
    ) -> ConstraintCircuitMonad<DualRowIndicator> {
        let indeterminate = self.circuit_builder.challenge(StandardInputIndeterminate);
        let read_io_deselector =
            InstructionDeselectors::instruction_deselector(self, Instruction::ReadIo);
        let read_io_selector = self.ci() - self.constant_b(Instruction::ReadIo.opcode_b());
        let input_symbol = self.st0_next();
        let running_evaluation_updates = self.running_evaluation_standard_input_next()
            - indeterminate * self.running_evaluation_standard_input()
            - input_symbol;
        let running_evaluation_remains = self.running_evaluation_standard_input_next()
            - self.running_evaluation_standard_input();

        read_io_selector * running_evaluation_remains
            + read_io_deselector * running_evaluation_updates
    }

    pub fn log_derivative_for_instruction_lookup_updates_correctly(
        &self,
    ) -> ConstraintCircuitMonad<DualRowIndicator> {
        let indeterminate = self
            .circuit_builder
            .challenge(InstructionLookupIndeterminate);
        let ip_weight = self.circuit_builder.challenge(ProgramAddressWeight);
        let ci_weight = self.circuit_builder.challenge(ProgramInstructionWeight);
        let nia_weight = self.circuit_builder.challenge(ProgramNextInstructionWeight);
        let compressed_row =
            ip_weight * self.ip_next() + ci_weight * self.ci_next() + nia_weight * self.nia_next();
        let log_derivative_updates = (self.instruction_lookup_log_derivative_next()
            - self.instruction_lookup_log_derivative())
            * (indeterminate - compressed_row)
            - self.one();
        let log_derivative_remains = self.instruction_lookup_log_derivative_next()
            - self.instruction_lookup_log_derivative();

        (self.one() - self.is_padding_next()) * log_derivative_updates
            + self.is_padding_next() * log_derivative_remains
    }

    pub fn running_evaluation_for_standard_output_updates_correctly(
        &self,
    ) -> ConstraintCircuitMonad<DualRowIndicator> {
        let indeterminate = self.circuit_builder.challenge(StandardOutputIndeterminate);
        let write_io_deselector =
            InstructionDeselectors::instruction_deselector_next(self, Instruction::WriteIo);
        let write_io_selector = self.ci_next() - self.constant_b(Instruction::WriteIo.opcode_b());
        let output_symbol = self.st0_next();
        let running_evaluation_updates = self.running_evaluation_standard_output_next()
            - indeterminate * self.running_evaluation_standard_output()
            - output_symbol;
        let running_evaluation_remains = self.running_evaluation_standard_output_next()
            - self.running_evaluation_standard_output();

        write_io_selector * running_evaluation_remains
            + write_io_deselector * running_evaluation_updates
    }

    pub fn running_product_for_op_stack_table_updates_correctly(
        &self,
    ) -> ConstraintCircuitMonad<DualRowIndicator> {
        let indeterminate = self.circuit_builder.challenge(OpStackIndeterminate);
        let clk_weight = self.circuit_builder.challenge(OpStackClkWeight);
        let ib1_weight = self.circuit_builder.challenge(OpStackIb1Weight);
        let osp_weight = self.circuit_builder.challenge(OpStackOspWeight);
        let osv_weight = self.circuit_builder.challenge(OpStackOsvWeight);
        let compressed_row = clk_weight * self.clk_next()
            + ib1_weight * self.ib1_next()
            + osp_weight * self.osp_next()
            + osv_weight * self.osv_next();

        self.running_product_op_stack_table_next()
            - self.running_product_op_stack_table() * (indeterminate - compressed_row)
    }

    pub fn running_product_for_ram_table_updates_correctly(
        &self,
    ) -> ConstraintCircuitMonad<DualRowIndicator> {
        let indeterminate = self.circuit_builder.challenge(RamIndeterminate);
        let clk_weight = self.circuit_builder.challenge(RamClkWeight);
        let ramp_weight = self.circuit_builder.challenge(RamRampWeight);
        let ramv_weight = self.circuit_builder.challenge(RamRamvWeight);
        let previous_instruction_weight =
            self.circuit_builder.challenge(RamPreviousInstructionWeight);
        let compressed_row = clk_weight * self.clk_next()
            + ramp_weight * self.ramp_next()
            + ramv_weight * self.ramv_next()
            + previous_instruction_weight * self.previous_instruction_next();

        self.running_product_ram_table_next()
            - self.running_product_ram_table() * (indeterminate - compressed_row)
    }

    pub fn running_product_for_jump_stack_table_updates_correctly(
        &self,
    ) -> ConstraintCircuitMonad<DualRowIndicator> {
        let indeterminate = self.circuit_builder.challenge(JumpStackIndeterminate);
        let clk_weight = self.circuit_builder.challenge(JumpStackClkWeight);
        let ci_weight = self.circuit_builder.challenge(JumpStackCiWeight);
        let jsp_weight = self.circuit_builder.challenge(JumpStackJspWeight);
        let jso_weight = self.circuit_builder.challenge(JumpStackJsoWeight);
        let jsd_weight = self.circuit_builder.challenge(JumpStackJsdWeight);
        let compressed_row = clk_weight * self.clk_next()
            + ci_weight * self.ci_next()
            + jsp_weight * self.jsp_next()
            + jso_weight * self.jso_next()
            + jsd_weight * self.jsd_next();

        self.running_product_jump_stack_table_next()
            - self.running_product_jump_stack_table() * (indeterminate - compressed_row)
    }

    pub fn running_evaluation_hash_input_updates_correctly(
        &self,
    ) -> ConstraintCircuitMonad<DualRowIndicator> {
        let hash_deselector =
            InstructionDeselectors::instruction_deselector_next(self, Instruction::Hash);
        let hash_selector = self.ci_next() - self.constant_b(Instruction::Hash.opcode_b());

        let indeterminate = self.circuit_builder.challenge(HashInputIndeterminate);

        let weights = [
            HashStateWeight0,
            HashStateWeight1,
            HashStateWeight2,
            HashStateWeight3,
            HashStateWeight4,
            HashStateWeight5,
            HashStateWeight6,
            HashStateWeight7,
            HashStateWeight8,
            HashStateWeight9,
        ]
        .map(|w| self.circuit_builder.challenge(w));
        let state = [
            self.st0_next(),
            self.st1_next(),
            self.st2_next(),
            self.st3_next(),
            self.st4_next(),
            self.st5_next(),
            self.st6_next(),
            self.st7_next(),
            self.st8_next(),
            self.st9_next(),
        ];
        let compressed_row = weights
            .into_iter()
            .zip_eq(state.into_iter())
            .map(|(weight, state)| weight * state)
            .sum();
        let running_evaluation_updates = self.running_evaluation_hash_input_next()
            - indeterminate * self.running_evaluation_hash_input()
            - compressed_row;
        let running_evaluation_remains =
            self.running_evaluation_hash_input_next() - self.running_evaluation_hash_input();

        hash_selector * running_evaluation_remains + hash_deselector * running_evaluation_updates
    }

    pub fn running_evaluation_hash_digest_updates_correctly(
        &self,
    ) -> ConstraintCircuitMonad<DualRowIndicator> {
        let hash_deselector =
            InstructionDeselectors::instruction_deselector(self, Instruction::Hash);
        let hash_selector = self.ci() - self.constant_b(Instruction::Hash.opcode_b());

        let indeterminate = self.circuit_builder.challenge(HashDigestIndeterminate);

        let weights = [
            HashStateWeight0,
            HashStateWeight1,
            HashStateWeight2,
            HashStateWeight3,
            HashStateWeight4,
        ]
        .map(|w| self.circuit_builder.challenge(w));
        let state = [
            self.st5_next(),
            self.st6_next(),
            self.st7_next(),
            self.st8_next(),
            self.st9_next(),
        ];
        let compressed_row = weights
            .into_iter()
            .zip_eq(state.into_iter())
            .map(|(weight, state)| weight * state)
            .sum();
        let running_evaluation_updates = self.running_evaluation_hash_digest_next()
            - indeterminate * self.running_evaluation_hash_digest()
            - compressed_row;
        let running_evaluation_remains =
            self.running_evaluation_hash_digest_next() - self.running_evaluation_hash_digest();

        hash_selector * running_evaluation_remains + hash_deselector * running_evaluation_updates
    }

    pub fn running_evaluation_sponge_updates_correctly(
        &self,
    ) -> ConstraintCircuitMonad<DualRowIndicator> {
        let absorb_init_deselector =
            InstructionDeselectors::instruction_deselector(self, Instruction::AbsorbInit);
        let absorb_deselector =
            InstructionDeselectors::instruction_deselector(self, Instruction::Absorb);
        let squeeze_deselector =
            InstructionDeselectors::instruction_deselector(self, Instruction::Squeeze);

        let opcode_absorb_init = self.constant_b(Instruction::AbsorbInit.opcode_b());
        let opcode_absorb = self.constant_b(Instruction::Absorb.opcode_b());
        let opcode_squeeze = self.constant_b(Instruction::Squeeze.opcode_b());
        let sponge_instruction_selector = (self.ci() - opcode_absorb_init)
            * (self.ci() - opcode_absorb)
            * (self.ci() - opcode_squeeze);

        let weights = [
            HashStateWeight0,
            HashStateWeight1,
            HashStateWeight2,
            HashStateWeight3,
            HashStateWeight4,
            HashStateWeight5,
            HashStateWeight6,
            HashStateWeight7,
            HashStateWeight8,
            HashStateWeight9,
        ]
        .map(|w| self.circuit_builder.challenge(w));
        let state_next = [
            self.st0_next(),
            self.st1_next(),
            self.st2_next(),
            self.st3_next(),
            self.st4_next(),
            self.st5_next(),
            self.st6_next(),
            self.st7_next(),
            self.st8_next(),
            self.st9_next(),
        ];
        let compressed_row_next = weights
            .into_iter()
            .zip_eq(state_next.into_iter())
            .map(|(weight, st_next)| weight * st_next)
            .sum();

        let indeterminate = self.circuit_builder.challenge(SpongeIndeterminate);
        let ci_weight = self.circuit_builder.challenge(HashCIWeight);
        let running_evaluation_updates = self.running_evaluation_sponge_next()
            - indeterminate * self.running_evaluation_sponge()
            - ci_weight * self.ci()
            - compressed_row_next;
        let running_evaluation_remains =
            self.running_evaluation_sponge_next() - self.running_evaluation_sponge();

        sponge_instruction_selector * running_evaluation_remains
            + absorb_init_deselector * running_evaluation_updates.clone()
            + absorb_deselector * running_evaluation_updates.clone()
            + squeeze_deselector * running_evaluation_updates
    }

    pub fn log_derivative_with_u32_table_updates_correctly(
        &self,
    ) -> ConstraintCircuitMonad<DualRowIndicator> {
        let opcode_of_and = self.circuit_builder.b_constant(Instruction::And.opcode_b());
        let two_inverse = self
            .circuit_builder
            .b_constant(BFieldElement::new(2).inverse());

        let indeterminate = self.circuit_builder.challenge(U32Indeterminate);
        let lhs_weight = self.circuit_builder.challenge(U32LhsWeight);
        let rhs_weight = self.circuit_builder.challenge(U32RhsWeight);
        let ci_weight = self.circuit_builder.challenge(U32CiWeight);
        let result_weight = self.circuit_builder.challenge(U32ResultWeight);

        let split_deselector =
            InstructionDeselectors::instruction_deselector(self, Instruction::Split);
        let lt_deselector = InstructionDeselectors::instruction_deselector(self, Instruction::Lt);
        let and_deselector = InstructionDeselectors::instruction_deselector(self, Instruction::And);
        let xor_deselector = InstructionDeselectors::instruction_deselector(self, Instruction::Xor);
        let pow_deselector = InstructionDeselectors::instruction_deselector(self, Instruction::Pow);
        let log_2_floor_deselector =
            InstructionDeselectors::instruction_deselector(self, Instruction::Log2Floor);
        let div_deselector = InstructionDeselectors::instruction_deselector(self, Instruction::Div);
        let pop_count_deselector =
            InstructionDeselectors::instruction_deselector(self, Instruction::PopCount);

        let running_sum = self.u32_table_running_sum_log_derivative();
        let running_sum_next = self.u32_table_running_sum_log_derivative_next();

        let split_factor = indeterminate.clone()
            - lhs_weight.clone() * self.st0_next()
            - rhs_weight.clone() * self.st1_next()
            - ci_weight.clone() * self.ci();
        let binop_factor = indeterminate.clone()
            - lhs_weight.clone() * self.st0()
            - rhs_weight.clone() * self.st1()
            - ci_weight.clone() * self.ci()
            - result_weight.clone() * self.st0_next();
        let xor_factor = indeterminate.clone()
            - lhs_weight.clone() * self.st0()
            - rhs_weight.clone() * self.st1()
            - ci_weight.clone() * opcode_of_and
            - result_weight.clone() * (self.st0() + self.st1() - self.st0_next()) * two_inverse;
        let unop_factor = indeterminate.clone()
            - lhs_weight.clone() * self.st0()
            - ci_weight.clone() * self.ci()
            - result_weight.clone() * self.st0_next();
        let div_factor_for_lt = indeterminate.clone()
            - lhs_weight.clone() * self.st0_next()
            - rhs_weight.clone() * self.st1()
            - ci_weight.clone() * self.constant_b(Instruction::Lt.opcode_b())
            - result_weight;
        let div_factor_for_range_check = indeterminate
            - lhs_weight * self.st0()
            - rhs_weight * self.st1_next()
            - ci_weight * self.constant_b(Instruction::Split.opcode_b());

        let split_summand = split_deselector
            * ((running_sum_next.clone() - running_sum.clone()) * split_factor - self.one());
        let lt_summand = lt_deselector
            * ((running_sum_next.clone() - running_sum.clone()) * binop_factor.clone()
                - self.one());
        let and_summand = and_deselector
            * ((running_sum_next.clone() - running_sum.clone()) * binop_factor.clone()
                - self.one());
        let xor_summand = xor_deselector
            * ((running_sum_next.clone() - running_sum.clone()) * xor_factor - self.one());
        let pow_summand = pow_deselector
            * ((running_sum_next.clone() - running_sum.clone()) * binop_factor - self.one());
        let log_2_floor_summand = log_2_floor_deselector
            * ((running_sum_next.clone() - running_sum.clone()) * unop_factor.clone() - self.one());
        let div_summand = div_deselector
            * ((running_sum_next.clone() - running_sum.clone())
                * div_factor_for_lt.clone()
                * div_factor_for_range_check.clone()
                - div_factor_for_lt
                - div_factor_for_range_check);
        let pop_count_summand = pop_count_deselector
            * ((running_sum_next.clone() - running_sum.clone()) * unop_factor - self.one());
        let no_update_summand = (self.one() - self.ib2()) * (running_sum_next - running_sum);

        split_summand
            + lt_summand
            + and_summand
            + xor_summand
            + pow_summand
            + log_2_floor_summand
            + div_summand
            + pop_count_summand
            + no_update_summand
    }
}

#[derive(Debug, Clone)]
pub struct InstructionDeselectors {
    deselectors: HashMap<Instruction, ConstraintCircuitMonad<DualRowIndicator>>,
}

impl InstructionDeselectors {
    fn new(factory: &mut DualRowConstraints) -> Self {
        let deselectors = Self::create(factory);

        Self { deselectors }
    }

    /// A polynomial that has solutions when `ci` is not `instruction`.
    ///
    /// This is naively achieved by constructing a polynomial that has
    /// a solution when `ci` is any other instruction. This deselector
    /// can be replaced with an efficient one based on `ib` registers.
    pub fn get(&self, instruction: Instruction) -> ConstraintCircuitMonad<DualRowIndicator> {
        self.deselectors
            .get(&instruction)
            .unwrap_or_else(|| panic!("The instruction {instruction} does not exist!"))
            .clone()
    }

    /// internal helper function to de-duplicate functionality common between the similar (but
    /// different on a type level) functions for construction deselectors
    fn instruction_deselector_common_functionality<II: InputIndicator>(
        circuit_builder: &ConstraintCircuitBuilder<II>,
        instruction: Instruction,
        instruction_bucket_polynomials: [ConstraintCircuitMonad<II>; Ord8::COUNT],
    ) -> ConstraintCircuitMonad<II> {
        let one = circuit_builder.b_constant(1u32.into());

        let selector_bits: [_; Ord8::COUNT] = [
            instruction.ib(Ord8::IB0),
            instruction.ib(Ord8::IB1),
            instruction.ib(Ord8::IB2),
            instruction.ib(Ord8::IB3),
            instruction.ib(Ord8::IB4),
            instruction.ib(Ord8::IB5),
            instruction.ib(Ord8::IB6),
            instruction.ib(Ord8::IB7),
        ];
        let deselector_polynomials =
            selector_bits.map(|b| one.clone() - circuit_builder.b_constant(b));

        instruction_bucket_polynomials
            .into_iter()
            .zip_eq(deselector_polynomials.into_iter())
            .map(|(bucket_poly, deselector_poly)| bucket_poly - deselector_poly)
            .fold(one, ConstraintCircuitMonad::mul)
    }

    /// A polynomial that has no solutions when ci is 'instruction'
    pub fn instruction_deselector(
        factory: &DualRowConstraints,
        instruction: Instruction,
    ) -> ConstraintCircuitMonad<DualRowIndicator> {
        let instruction_bucket_polynomials = [
            factory.ib0(),
            factory.ib1(),
            factory.ib2(),
            factory.ib3(),
            factory.ib4(),
            factory.ib5(),
            factory.ib6(),
            factory.ib7(),
        ];

        Self::instruction_deselector_common_functionality(
            &factory.circuit_builder,
            instruction,
            instruction_bucket_polynomials,
        )
    }

    /// A polynomial that has no solutions when ci is 'instruction'
    pub fn instruction_deselector_single_row(
        factory: &SingleRowConstraints,
        instruction: Instruction,
    ) -> ConstraintCircuitMonad<SingleRowIndicator> {
        let instruction_bucket_polynomials = [
            factory.ib0(),
            factory.ib1(),
            factory.ib2(),
            factory.ib3(),
            factory.ib4(),
            factory.ib5(),
            factory.ib6(),
            factory.ib7(),
        ];

        Self::instruction_deselector_common_functionality(
            &factory.circuit_builder,
            instruction,
            instruction_bucket_polynomials,
        )
    }

    /// A polynomial that has no solutions when ci_next is 'instruction'
    pub fn instruction_deselector_next(
        factory: &DualRowConstraints,
        instruction: Instruction,
    ) -> ConstraintCircuitMonad<DualRowIndicator> {
        let instruction_bucket_polynomials = [
            factory.ib0_next(),
            factory.ib1_next(),
            factory.ib2_next(),
            factory.ib3_next(),
            factory.ib4_next(),
            factory.ib5_next(),
            factory.ib6_next(),
            factory.ib7_next(),
        ];

        Self::instruction_deselector_common_functionality(
            &factory.circuit_builder,
            instruction,
            instruction_bucket_polynomials,
        )
    }

    pub fn create(
        factory: &mut DualRowConstraints,
    ) -> HashMap<Instruction, ConstraintCircuitMonad<DualRowIndicator>> {
        ALL_INSTRUCTIONS
            .iter()
            .map(|&instr| (instr, Self::instruction_deselector(factory, instr)))
            .collect()
    }
}

pub struct ProcessorTraceRow<'a> {
    pub row: ArrayView1<'a, BFieldElement>,
}

impl<'a> Display for ProcessorTraceRow<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn row(f: &mut std::fmt::Formatter<'_>, s: String) -> std::fmt::Result {
            writeln!(f, "│ {s: <103} │")
        }

        fn row_blank(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            row(f, "".into())
        }

        let instruction = self.row[CI.base_table_index()].value().try_into().unwrap();
        let instruction_with_arg = match instruction {
            Push(_) => Push(self.row[NIA.base_table_index()]),
            Call(_) => Call(self.row[NIA.base_table_index()]),
            Dup(_) => Dup((self.row[NIA.base_table_index()].value() as u32)
                .try_into()
                .unwrap()),
            Swap(_) => Swap(
                (self.row[NIA.base_table_index()].value() as u32)
                    .try_into()
                    .unwrap(),
            ),
            _ => instruction,
        };

        writeln!(f, " ╭───────────────────────────╮")?;
        writeln!(f, " │ {: <25} │", format!("{instruction_with_arg}"))?;
        writeln!(
            f,
            "╭┴───────────────────────────┴────────────────────────────────────\
            ────────────────────┬───────────────────╮"
        )?;

        let width = 20;
        row(
            f,
            format!(
                "ip:   {:>width$} ╷ ci:   {:>width$} ╷ nia: {:>width$} │ {:>17}",
                self.row[IP.base_table_index()].value(),
                self.row[CI.base_table_index()].value(),
                self.row[NIA.base_table_index()].value(),
                self.row[CLK.base_table_index()].value(),
            ),
        )?;

        writeln!(
            f,
            "│ jsp:  {:>width$} │ jso:  {:>width$} │ jsd: {:>width$} ╰───────────────────┤",
            self.row[JSP.base_table_index()].value(),
            self.row[JSO.base_table_index()].value(),
            self.row[JSD.base_table_index()].value(),
        )?;
        row(
            f,
            format!(
                "ramp: {:>width$} │ ramv: {:>width$} │",
                self.row[RAMP.base_table_index()].value(),
                self.row[RAMV.base_table_index()].value(),
            ),
        )?;
        row(
            f,
            format!(
                "osp:  {:>width$} │ osv:  {:>width$} ╵",
                self.row[OSP.base_table_index()].value(),
                self.row[OSV.base_table_index()].value(),
            ),
        )?;

        row_blank(f)?;

        row(
            f,
            format!(
                "st0-3:    [ {:>width$} | {:>width$} | {:>width$} | {:>width$} ]",
                self.row[ST0.base_table_index()].value(),
                self.row[ST1.base_table_index()].value(),
                self.row[ST2.base_table_index()].value(),
                self.row[ST3.base_table_index()].value(),
            ),
        )?;
        row(
            f,
            format!(
                "st4-7:    [ {:>width$} | {:>width$} | {:>width$} | {:>width$} ]",
                self.row[ST4.base_table_index()].value(),
                self.row[ST5.base_table_index()].value(),
                self.row[ST6.base_table_index()].value(),
                self.row[ST7.base_table_index()].value(),
            ),
        )?;
        row(
            f,
            format!(
                "st8-11:   [ {:>width$} | {:>width$} | {:>width$} | {:>width$} ]",
                self.row[ST8.base_table_index()].value(),
                self.row[ST9.base_table_index()].value(),
                self.row[ST10.base_table_index()].value(),
                self.row[ST11.base_table_index()].value(),
            ),
        )?;
        row(
            f,
            format!(
                "st12-15:  [ {:>width$} | {:>width$} | {:>width$} | {:>width$} ]",
                self.row[ST12.base_table_index()].value(),
                self.row[ST13.base_table_index()].value(),
                self.row[ST14.base_table_index()].value(),
                self.row[ST15.base_table_index()].value(),
            ),
        )?;

        row_blank(f)?;

        let w = 2;
        row(
            f,
            format!(
                "hv0-3:    [ {:>w$} | {:>w$} | {:>w$} | {:>w$} ]",
                self.row[HV0.base_table_index()].value(),
                self.row[HV1.base_table_index()].value(),
                self.row[HV2.base_table_index()].value(),
                self.row[HV3.base_table_index()].value(),
            ),
        )?;
        row(
            f,
            format!(
                "ib0-7:    \
                [ {:>w$} | {:>w$} | {:>w$} | {:>w$} | {:>w$} | {:>w$} | {:>w$} | {:>w$} ]",
                self.row[IB0.base_table_index()].value(),
                self.row[IB1.base_table_index()].value(),
                self.row[IB2.base_table_index()].value(),
                self.row[IB3.base_table_index()].value(),
                self.row[IB4.base_table_index()].value(),
                self.row[IB5.base_table_index()].value(),
                self.row[IB6.base_table_index()].value(),
                self.row[IB7.base_table_index()].value(),
            ),
        )?;
        write!(
            f,
            "╰─────────────────────────────────────────────────────────────────\
            ────────────────────────────────────────╯"
        )
    }
}

pub struct ExtProcessorTraceRow<'a> {
    pub row: ArrayView1<'a, XFieldElement>,
}

impl<'a> Display for ExtProcessorTraceRow<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let row = |form: &mut std::fmt::Formatter<'_>,
                   desc: &str,
                   col: ProcessorExtTableColumn|
         -> std::fmt::Result {
            // without the extra `format!()`, alignment in `writeln!()` fails
            let formatted_col_elem = format!("{}", self.row[col.ext_table_index()]);
            writeln!(form, "     │ {desc: <18}  {formatted_col_elem:>73} │")
        };
        writeln!(
            f,
            "     ╭───────────────────────────────────────────────────────\
            ────────────────────────────────────────╮"
        )?;
        row(f, "input_table_ea", InputTableEvalArg)?;
        row(f, "output_table_ea", OutputTableEvalArg)?;
        row(f, "instr_lookup_ld", InstructionLookupClientLogDerivative)?;
        row(f, "opstack_table_pa", OpStackTablePermArg)?;
        row(f, "ram_table_pa", RamTablePermArg)?;
        row(f, "jumpstack_table_pa", JumpStackTablePermArg)?;
        row(f, "hash_input_ea", HashInputEvalArg)?;
        row(f, "hash_digest_ea", HashDigestEvalArg)?;
        row(f, "sponge_absorb_ea", SpongeEvalArg)?;
        row(f, "u32_table_pa", U32LookupClientLogDerivative)?;
        write!(
            f,
            "     ╰───────────────────────────────────────────────────────\
            ────────────────────────────────────────╯"
        )
    }
}

#[cfg(test)]
mod constraint_polynomial_tests {
    use ndarray::Array2;
    use triton_opcodes::ord_n::Ord16;
    use triton_opcodes::program::Program;

    use crate::error::InstructionError;
    use crate::error::InstructionError::DivisionByZero;
    use crate::shared_tests::SourceCodeAndInput;
    use crate::stark::triton_stark_tests::parse_simulate_pad;
    use crate::table::master_table::MasterTable;
    use crate::table::processor_table::ProcessorTraceRow;
    use crate::vm::simulate;

    use super::*;

    #[test]
    /// helps identifying whether the printing causes an infinite loop
    fn print_simple_processor_table_row_test() {
        let code = "push 2 push -1 add assert halt";
        let program = Program::from_code(code).unwrap();
        let (aet, _, _) = simulate(Box::new(program), vec![], vec![]);
        println!();
        for row in aet.processor_trace.rows() {
            println!("{}", ProcessorTraceRow { row });
        }
    }

    fn get_test_row_from_source_code(source_code: &str, row_num: usize) -> Array2<BFieldElement> {
        let (_, _, unpadded_master_base_table, _) = parse_simulate_pad(source_code, vec![], vec![]);
        unpadded_master_base_table
            .trace_table()
            .slice(s![row_num..=row_num + 1, ..])
            .to_owned()
    }

    fn get_transition_constraints_for_instruction(
        instruction: Instruction,
    ) -> Vec<ConstraintCircuitMonad<DualRowIndicator>> {
        let tc = DualRowConstraints::default();
        match instruction {
            Pop => tc.instruction_pop(),
            Push(_) => tc.instruction_push(),
            Divine(_) => tc.instruction_divine(),
            Dup(_) => tc.instruction_dup(),
            Swap(_) => tc.instruction_swap(),
            Nop => tc.instruction_nop(),
            Skiz => tc.instruction_skiz(),
            Call(_) => tc.instruction_call(),
            Return => tc.instruction_return(),
            Recurse => tc.instruction_recurse(),
            Assert => tc.instruction_assert(),
            Halt => tc.instruction_halt(),
            ReadMem => tc.instruction_read_mem(),
            WriteMem => tc.instruction_write_mem(),
            Hash => tc.instruction_hash(),
            DivineSibling => tc.instruction_divine_sibling(),
            AssertVector => tc.instruction_assert_vector(),
            AbsorbInit => tc.instruction_absorb_init(),
            Absorb => tc.instruction_absorb(),
            Squeeze => tc.instruction_squeeze(),
            Add => tc.instruction_add(),
            Mul => tc.instruction_mul(),
            Invert => tc.instruction_invert(),
            Eq => tc.instruction_eq(),
            Split => tc.instruction_split(),
            Lt => tc.instruction_lt(),
            And => tc.instruction_and(),
            Xor => tc.instruction_xor(),
            Log2Floor => tc.instruction_log_2_floor(),
            Pow => tc.instruction_pow(),
            Div => tc.instruction_div(),
            PopCount => tc.instruction_pop_count(),
            XxAdd => tc.instruction_xxadd(),
            XxMul => tc.instruction_xxmul(),
            XInvert => tc.instruction_xinv(),
            XbMul => tc.instruction_xbmul(),
            ReadIo => tc.instruction_read_io(),
            WriteIo => tc.instruction_write_io(),
        }
    }

    fn test_constraints_for_rows_with_debug_info(
        instruction: Instruction,
        master_base_tables: &[Array2<BFieldElement>],
        debug_cols_curr_row: &[ProcessorBaseTableColumn],
        debug_cols_next_row: &[ProcessorBaseTableColumn],
    ) {
        let challenges = Challenges::placeholder(&[], &[]);
        let fake_ext_table = Array2::zeros([2, NUM_EXT_COLUMNS]);
        for (case_idx, test_rows) in master_base_tables.iter().enumerate() {
            let curr_row = test_rows.slice(s![0, ..]);
            let next_row = test_rows.slice(s![1, ..]);

            // Print debug information
            println!(
                "Testing all constraints of {instruction} for test row with index {case_idx}…"
            );
            for &c in debug_cols_curr_row {
                print!("{} = {}, ", c, curr_row[c.master_base_table_index()]);
            }
            for &c in debug_cols_next_row {
                print!("{}' = {}, ", c, next_row[c.master_base_table_index()]);
            }
            println!();

            assert_eq!(
                instruction.opcode_b(),
                curr_row[CI.master_base_table_index()],
                "The test is trying to check the wrong transition constraint polynomials."
            );
            for (constraint_idx, constraint_circuit) in
                get_transition_constraints_for_instruction(instruction)
                    .into_iter()
                    .enumerate()
            {
                let evaluation_result = constraint_circuit.consume().evaluate(
                    test_rows.view(),
                    fake_ext_table.view(),
                    &challenges,
                );
                assert_eq!(
                    XFieldElement::zero(),
                    evaluation_result,
                    "For case {case_idx}, transition constraint polynomial with \
                    index {constraint_idx} must evaluate to zero. Got {evaluation_result} instead.",
                );
            }
        }
    }

    #[test]
    fn transition_constraints_for_instruction_pop_test() {
        let test_rows = [get_test_row_from_source_code("push 1 pop halt", 1)];
        test_constraints_for_rows_with_debug_info(
            Pop,
            &test_rows,
            &[ST0, ST1, ST2],
            &[ST0, ST1, ST2],
        );
    }

    #[test]
    fn transition_constraints_for_instruction_push_test() {
        let test_rows = [get_test_row_from_source_code("push 1 halt", 0)];
        test_constraints_for_rows_with_debug_info(
            Push(BFieldElement::one()),
            &test_rows,
            &[ST0, ST1, ST2],
            &[ST0, ST1, ST2],
        );
    }

    #[test]
    fn transition_constraints_for_instruction_dup_test() {
        let test_rows = [get_test_row_from_source_code("push 1 dup 0 halt", 1)];
        test_constraints_for_rows_with_debug_info(
            Dup(Ord16::ST0),
            &test_rows,
            &[ST0, ST1, ST2],
            &[ST0, ST1, ST2],
        );
    }

    #[test]
    fn transition_constraints_for_instruction_swap_test() {
        let test_rows = [get_test_row_from_source_code(
            "push 1 push 2 swap 1 halt",
            2,
        )];
        test_constraints_for_rows_with_debug_info(
            Swap(Ord16::ST0),
            &test_rows,
            &[ST0, ST1, ST2],
            &[ST0, ST1, ST2],
        );
    }

    #[test]
    fn transition_constraints_for_instruction_skiz_test() {
        // Case 0: ST0 is non-zero
        // Case 1: ST0 is zero, nia is instruction of size 1
        // Case 2: ST0 is zero, nia is instruction of size 2
        let test_rows = [
            get_test_row_from_source_code("push 1 skiz halt", 1),
            get_test_row_from_source_code("push 0 skiz assert halt", 1),
            get_test_row_from_source_code("push 0 skiz push 1 halt", 1),
        ];
        test_constraints_for_rows_with_debug_info(Skiz, &test_rows, &[IP, ST0, HV0, HV1], &[IP]);
    }

    #[test]
    fn transition_constraints_for_instruction_call_test() {
        let test_rows = [get_test_row_from_source_code("call label label: halt", 0)];
        test_constraints_for_rows_with_debug_info(
            Call(Default::default()),
            &test_rows,
            &[IP, CI, NIA, JSP, JSO, JSD],
            &[IP, CI, NIA, JSP, JSO, JSD],
        );
    }

    #[test]
    fn transition_constraints_for_instruction_return_test() {
        let test_rows = [get_test_row_from_source_code(
            "call label halt label: return",
            1,
        )];
        test_constraints_for_rows_with_debug_info(
            Return,
            &test_rows,
            &[IP, JSP, JSO, JSD],
            &[IP, JSP, JSO, JSD],
        );
    }

    #[test]
    fn transition_constraints_for_instruction_recurse_test() {
        let test_rows = [get_test_row_from_source_code(
            "push 2 call label halt label: push -1 add dup 0 skiz recurse return ",
            6,
        )];
        test_constraints_for_rows_with_debug_info(
            Recurse,
            &test_rows,
            &[IP, JSP, JSO, JSD],
            &[IP, JSP, JSO, JSD],
        );
    }

    #[test]
    fn transition_constraints_for_instruction_read_mem_test() {
        let test_rows = [get_test_row_from_source_code(
            "push 5 push 3 write_mem read_mem halt",
            3,
        )];
        test_constraints_for_rows_with_debug_info(
            ReadMem,
            &test_rows,
            &[ST0, ST1, RAMP, RAMV],
            &[ST0, ST1, RAMP, RAMV],
        );
    }

    #[test]
    fn transition_constraints_for_instruction_write_mem_test() {
        let test_rows = [get_test_row_from_source_code(
            "push 5 push 3 write_mem read_mem halt",
            2,
        )];
        test_constraints_for_rows_with_debug_info(
            WriteMem,
            &test_rows,
            &[ST0, ST1, RAMP, RAMV],
            &[ST0, ST1, RAMP, RAMV],
        );
    }

    #[test]
    fn transition_constraints_for_instruction_eq_test() {
        let test_rows = [
            get_test_row_from_source_code("push 3 push 3 eq assert halt", 2),
            get_test_row_from_source_code("push 3 push 2 eq push 0 eq assert halt", 2),
        ];
        test_constraints_for_rows_with_debug_info(Eq, &test_rows, &[ST0, ST1, HV0], &[ST0]);
    }

    #[test]
    fn transition_constraints_for_instruction_split_test() {
        let test_rows = [
            get_test_row_from_source_code("push -1 split halt", 1),
            get_test_row_from_source_code("push  0 split halt", 1),
            get_test_row_from_source_code("push  1 split halt", 1),
            get_test_row_from_source_code("push  2 split halt", 1),
            get_test_row_from_source_code("push  3 split halt", 1),
            // test pushing push 2^32 +- 1
            get_test_row_from_source_code("push 4294967295 split halt", 1),
            get_test_row_from_source_code("push 4294967296 split halt", 1),
            get_test_row_from_source_code("push 4294967297 split halt", 1),
        ];
        test_constraints_for_rows_with_debug_info(
            Split,
            &test_rows,
            &[ST0, ST1, HV0],
            &[ST0, ST1, HV0],
        );
    }

    #[test]
    fn transition_constraints_for_instruction_lt_test() {
        let test_rows = [
            get_test_row_from_source_code("push 3 push 3 lt push 0 eq assert halt", 2),
            get_test_row_from_source_code("push 3 push 2 lt push 1 eq assert halt", 2),
            get_test_row_from_source_code("push 2 push 3 lt push 0 eq assert halt", 2),
            get_test_row_from_source_code("push 512 push 513 lt push 0 eq assert halt", 2),
        ];
        test_constraints_for_rows_with_debug_info(Lt, &test_rows, &[ST0, ST1], &[ST0]);
    }

    #[test]
    fn transition_constraints_for_instruction_and_test() {
        let test_rows = [get_test_row_from_source_code(
            "push 5 push 12 and push 4 eq assert halt",
            2,
        )];
        test_constraints_for_rows_with_debug_info(And, &test_rows, &[ST0, ST1], &[ST0]);
    }

    #[test]
    fn transition_constraints_for_instruction_xor_test() {
        let test_rows = [get_test_row_from_source_code(
            "push 5 push 12 xor push 9 eq assert halt",
            2,
        )];
        test_constraints_for_rows_with_debug_info(Xor, &test_rows, &[ST0, ST1], &[ST0]);
    }

    #[test]
    fn transition_constraints_for_instruction_log2floor_test() {
        let test_rows = [
            get_test_row_from_source_code("push  1 log_2_floor push  0 eq assert halt", 1),
            get_test_row_from_source_code("push  2 log_2_floor push  1 eq assert halt", 1),
            get_test_row_from_source_code("push  3 log_2_floor push  1 eq assert halt", 1),
            get_test_row_from_source_code("push  4 log_2_floor push  2 eq assert halt", 1),
            get_test_row_from_source_code("push  5 log_2_floor push  2 eq assert halt", 1),
            get_test_row_from_source_code("push  6 log_2_floor push  2 eq assert halt", 1),
            get_test_row_from_source_code("push  7 log_2_floor push  2 eq assert halt", 1),
            get_test_row_from_source_code("push  8 log_2_floor push  3 eq assert halt", 1),
            get_test_row_from_source_code("push  9 log_2_floor push  3 eq assert halt", 1),
            get_test_row_from_source_code("push 10 log_2_floor push  3 eq assert halt", 1),
            get_test_row_from_source_code("push 11 log_2_floor push  3 eq assert halt", 1),
            get_test_row_from_source_code("push 12 log_2_floor push  3 eq assert halt", 1),
            get_test_row_from_source_code("push 13 log_2_floor push  3 eq assert halt", 1),
            get_test_row_from_source_code("push 14 log_2_floor push  3 eq assert halt", 1),
            get_test_row_from_source_code("push 15 log_2_floor push  3 eq assert halt", 1),
            get_test_row_from_source_code("push 16 log_2_floor push  4 eq assert halt", 1),
            get_test_row_from_source_code("push 17 log_2_floor push  4 eq assert halt", 1),
        ];
        test_constraints_for_rows_with_debug_info(Log2Floor, &test_rows, &[ST0, ST1], &[ST0]);
    }

    #[test]
    fn transition_constraints_for_instruction_pow_test() {
        let test_rows = [
            get_test_row_from_source_code("push 0 push  0 pow push   1 eq assert halt", 2),
            get_test_row_from_source_code("push 1 push  0 pow push   0 eq assert halt", 2),
            get_test_row_from_source_code("push 2 push  0 pow push   0 eq assert halt", 2),
            get_test_row_from_source_code("push 0 push  1 pow push   1 eq assert halt", 2),
            get_test_row_from_source_code("push 1 push  1 pow push   1 eq assert halt", 2),
            get_test_row_from_source_code("push 2 push  1 pow push   1 eq assert halt", 2),
            get_test_row_from_source_code("push 0 push  2 pow push   1 eq assert halt", 2),
            get_test_row_from_source_code("push 1 push  2 pow push   2 eq assert halt", 2),
            get_test_row_from_source_code("push 2 push  2 pow push   4 eq assert halt", 2),
            get_test_row_from_source_code("push 3 push  2 pow push   8 eq assert halt", 2),
            get_test_row_from_source_code("push 4 push  2 pow push  16 eq assert halt", 2),
            get_test_row_from_source_code("push 5 push  2 pow push  32 eq assert halt", 2),
            get_test_row_from_source_code("push 0 push  3 pow push   1 eq assert halt", 2),
            get_test_row_from_source_code("push 1 push  3 pow push   3 eq assert halt", 2),
            get_test_row_from_source_code("push 2 push  3 pow push   9 eq assert halt", 2),
            get_test_row_from_source_code("push 3 push  3 pow push  27 eq assert halt", 2),
            get_test_row_from_source_code("push 4 push  3 pow push  81 eq assert halt", 2),
            get_test_row_from_source_code("push 0 push 17 pow push   1 eq assert halt", 2),
            get_test_row_from_source_code("push 1 push 17 pow push  17 eq assert halt", 2),
            get_test_row_from_source_code("push 2 push 17 pow push 289 eq assert halt", 2),
        ];
        test_constraints_for_rows_with_debug_info(Pow, &test_rows, &[ST0, ST1], &[ST0]);
    }

    #[test]
    fn transition_constraints_for_instruction_div_test() {
        let test_rows = [
            get_test_row_from_source_code(
                "push 2 push 3 div push 1 eq assert push 1 eq assert halt",
                2,
            ),
            get_test_row_from_source_code(
                "push 3 push 7 div push 1 eq assert push 2 eq assert halt",
                2,
            ),
            get_test_row_from_source_code(
                "push 4 push 7 div push 3 eq assert push 1 eq assert halt",
                2,
            ),
        ];
        test_constraints_for_rows_with_debug_info(Div, &test_rows, &[ST0, ST1], &[ST0, ST1]);
    }

    #[test]
    fn division_by_zero_is_impossible_test() {
        let (_aet, _out, err) = SourceCodeAndInput::without_input("div").simulate();
        let Some(err) = err else {
            panic!("Dividing by 0 must fail.");
        };
        let Ok(err) = err.downcast::<InstructionError>() else {
            panic!("Dividing by 0 must fail with InstructionError.");
        };
        let DivisionByZero = err else {
            panic!("Dividing by 0 must fail with DivisionByZero.");
        };
    }

    #[test]
    fn transition_constraints_for_instruction_xxadd_test() {
        let test_rows = [
            get_test_row_from_source_code(
                "push 5 push 6 push 7 push 8 push 9 push 10 xxadd halt",
                6,
            ),
            get_test_row_from_source_code(
                "push 2 push 3 push 4 push -2 push -3 push -4 xxadd halt",
                6,
            ),
        ];
        test_constraints_for_rows_with_debug_info(
            XxAdd,
            &test_rows,
            &[ST0, ST1, ST2, ST3, ST4, ST5],
            &[ST0, ST1, ST2, ST3, ST4, ST5],
        );
    }

    #[test]
    fn transition_constraints_for_instruction_xxmul_test() {
        let test_rows = [
            get_test_row_from_source_code(
                "push 5 push 6 push 7 push 8 push 9 push 10 xxmul halt",
                6,
            ),
            get_test_row_from_source_code(
                "push 2 push 3 push 4 push -2 push -3 push -4 xxmul halt",
                6,
            ),
        ];
        test_constraints_for_rows_with_debug_info(
            XxMul,
            &test_rows,
            &[ST0, ST1, ST2, ST3, ST4, ST5],
            &[ST0, ST1, ST2, ST3, ST4, ST5],
        );
    }

    #[test]
    fn transition_constraints_for_instruction_xinvert_test() {
        let test_rows = [
            get_test_row_from_source_code("push 5 push 6 push 7 xinvert halt", 3),
            get_test_row_from_source_code("push -2 push -3 push -4 xinvert halt", 3),
        ];
        test_constraints_for_rows_with_debug_info(
            XInvert,
            &test_rows,
            &[ST0, ST1, ST2],
            &[ST0, ST1, ST2],
        );
    }

    #[test]
    fn transition_constraints_for_instruction_xbmul_test() {
        let test_rows = [
            get_test_row_from_source_code("push 5 push 6 push 7 push 2 xbmul halt", 4),
            get_test_row_from_source_code("push 2 push 3 push 4 push -2 xbmul halt", 4),
        ];
        test_constraints_for_rows_with_debug_info(
            XbMul,
            &test_rows,
            &[ST0, ST1, ST2, ST3, OSP, HV3],
            &[ST0, ST1, ST2, ST3, OSP, HV3],
        );
    }

    #[test]
    fn instruction_deselector_gives_0_for_all_other_instructions_test() {
        let mut factory = DualRowConstraints::default();
        let deselectors = InstructionDeselectors::new(&mut factory);

        let mut master_base_table = Array2::zeros([2, NUM_BASE_COLUMNS]);
        let master_ext_table = Array2::zeros([2, NUM_EXT_COLUMNS]);

        // We need dummy challenges to evaluate.
        let dummy_challenges = Challenges::placeholder(&[], &[]);
        for instruction in ALL_INSTRUCTIONS {
            use ProcessorBaseTableColumn::*;
            let deselector = deselectors.get(instruction);

            println!("\n\nThe Deselector for instruction {instruction} is:\n{deselector}",);

            // Negative tests
            for other_instruction in ALL_INSTRUCTIONS
                .into_iter()
                .filter(|other_instruction| *other_instruction != instruction)
            {
                let mut curr_row = master_base_table.slice_mut(s![0, ..]);
                curr_row[IB0.master_base_table_index()] = other_instruction.ib(Ord8::IB0);
                curr_row[IB1.master_base_table_index()] = other_instruction.ib(Ord8::IB1);
                curr_row[IB2.master_base_table_index()] = other_instruction.ib(Ord8::IB2);
                curr_row[IB3.master_base_table_index()] = other_instruction.ib(Ord8::IB3);
                curr_row[IB4.master_base_table_index()] = other_instruction.ib(Ord8::IB4);
                curr_row[IB5.master_base_table_index()] = other_instruction.ib(Ord8::IB5);
                curr_row[IB6.master_base_table_index()] = other_instruction.ib(Ord8::IB6);
                curr_row[IB7.master_base_table_index()] = other_instruction.ib(Ord8::IB7);
                let result = deselector.clone().consume().evaluate(
                    master_base_table.view(),
                    master_ext_table.view(),
                    &dummy_challenges,
                );

                assert!(
                    result.is_zero(),
                    "Deselector for {instruction} should return 0 for all other instructions, \
                    including {other_instruction} whose opcode is {}",
                    other_instruction.opcode()
                )
            }

            // Positive tests
            let mut curr_row = master_base_table.slice_mut(s![0, ..]);
            curr_row[IB0.master_base_table_index()] = instruction.ib(Ord8::IB0);
            curr_row[IB1.master_base_table_index()] = instruction.ib(Ord8::IB1);
            curr_row[IB2.master_base_table_index()] = instruction.ib(Ord8::IB2);
            curr_row[IB3.master_base_table_index()] = instruction.ib(Ord8::IB3);
            curr_row[IB4.master_base_table_index()] = instruction.ib(Ord8::IB4);
            curr_row[IB5.master_base_table_index()] = instruction.ib(Ord8::IB5);
            curr_row[IB6.master_base_table_index()] = instruction.ib(Ord8::IB6);
            curr_row[IB7.master_base_table_index()] = instruction.ib(Ord8::IB7);
            let result = deselector.consume().evaluate(
                master_base_table.view(),
                master_ext_table.view(),
                &dummy_challenges,
            );
            assert!(
                !result.is_zero(),
                "Deselector for {instruction} should be non-zero when CI is {}",
                instruction.opcode()
            )
        }
    }

    #[test]
    fn print_number_and_degrees_of_transition_constraints_for_all_instructions() {
        let factory = DualRowConstraints::default();
        let all_instructions_and_their_transition_constraints: [(Instruction, _);
            Instruction::COUNT] = [
            (Pop, factory.instruction_pop()),
            (Push(Default::default()), factory.instruction_push()),
            (Divine(Default::default()), factory.instruction_divine()),
            (Dup(Default::default()), factory.instruction_dup()),
            (Swap(Default::default()), factory.instruction_swap()),
            (Nop, factory.instruction_nop()),
            (Skiz, factory.instruction_skiz()),
            (Call(Default::default()), factory.instruction_call()),
            (Return, factory.instruction_return()),
            (Recurse, factory.instruction_recurse()),
            (Assert, factory.instruction_assert()),
            (Halt, factory.instruction_halt()),
            (ReadMem, factory.instruction_read_mem()),
            (WriteMem, factory.instruction_write_mem()),
            (Hash, factory.instruction_hash()),
            (DivineSibling, factory.instruction_divine_sibling()),
            (AssertVector, factory.instruction_assert_vector()),
            (AbsorbInit, factory.instruction_absorb_init()),
            (Absorb, factory.instruction_absorb()),
            (Squeeze, factory.instruction_squeeze()),
            (Add, factory.instruction_add()),
            (Mul, factory.instruction_mul()),
            (Invert, factory.instruction_invert()),
            (Eq, factory.instruction_eq()),
            (Split, factory.instruction_split()),
            (Lt, factory.instruction_lt()),
            (And, factory.instruction_and()),
            (Xor, factory.instruction_xor()),
            (Log2Floor, factory.instruction_log_2_floor()),
            (Pow, factory.instruction_pow()),
            (Div, factory.instruction_div()),
            (PopCount, factory.instruction_pop_count()),
            (XxAdd, factory.instruction_xxadd()),
            (XxMul, factory.instruction_xxmul()),
            (XInvert, factory.instruction_xinv()),
            (XbMul, factory.instruction_xbmul()),
            (ReadIo, factory.instruction_read_io()),
            (WriteIo, factory.instruction_write_io()),
        ];

        println!();
        println!("| Instruction     | #polys | max deg | Degrees");
        println!("|:----------------|-------:|--------:|:------------");
        for (instruction, constraints) in all_instructions_and_their_transition_constraints {
            let degrees = constraints
                .iter()
                .map(|circuit| circuit.clone().consume().degree())
                .collect_vec();
            let max_degree = degrees.iter().max().unwrap_or(&0);
            let degrees_str = degrees.iter().map(|d| format!("{d}")).join(", ");
            println!(
                "| {:<15} | {:>6} | {:>7} | [{}]",
                format!("{instruction}"),
                constraints.len(),
                max_degree,
                degrees_str,
            );
        }
    }
}
