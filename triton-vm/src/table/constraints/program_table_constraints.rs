use ndarray::ArrayView1;
use twenty_first::shared_math::b_field_element::BFieldElement;
use twenty_first::shared_math::mpolynomial::Degree;
use twenty_first::shared_math::x_field_element::XFieldElement;

use crate::table::challenges::ChallengeId::*;
use crate::table::challenges::Challenges;
use crate::table::extension_table::Evaluable;
use crate::table::extension_table::Quotientable;
use crate::table::program_table::ExtProgramTable;

// This file has been auto-generated. Any modifications _will_ be lost.
// To re-generate, execute:
// `cargo run --bin constraint-evaluation-generator`
impl Evaluable<BFieldElement> for ExtProgramTable {
    #[inline]
    #[allow(unused_variables)]
    fn evaluate_initial_constraints(
        base_row: ArrayView1<BFieldElement>,
        ext_row: ArrayView1<XFieldElement>,
        challenges: &Challenges,
    ) -> Vec<XFieldElement> {
        let base_constraints = [base_row[0]];
        let ext_constraints = [ext_row[0]];
        base_constraints
            .into_iter()
            .map(|bfe| bfe.lift())
            .chain(ext_constraints.into_iter())
            .collect()
    }

    #[inline]
    #[allow(unused_variables)]
    fn evaluate_consistency_constraints(
        base_row: ArrayView1<BFieldElement>,
        ext_row: ArrayView1<XFieldElement>,
        challenges: &Challenges,
    ) -> Vec<XFieldElement> {
        let base_constraints = [(base_row[3]) * ((base_row[3]) - (BFieldElement::new(1)))];
        let ext_constraints = [];
        base_constraints
            .into_iter()
            .map(|bfe| bfe.lift())
            .chain(ext_constraints.into_iter())
            .collect()
    }

    #[inline]
    #[allow(unused_variables)]
    fn evaluate_transition_constraints(
        current_base_row: ArrayView1<BFieldElement>,
        current_ext_row: ArrayView1<XFieldElement>,
        next_base_row: ArrayView1<BFieldElement>,
        next_ext_row: ArrayView1<XFieldElement>,
        challenges: &Challenges,
    ) -> Vec<XFieldElement> {
        let node_14 = (next_ext_row[0]) - (current_ext_row[0]);

        let base_constraints = [
            (next_base_row[0]) - ((current_base_row[0]) + (BFieldElement::new(1))),
            (current_base_row[3]) * ((next_base_row[3]) - (current_base_row[3])),
        ];
        let ext_constraints = [(((BFieldElement::new(1)) - (current_base_row[3]))
            * (((node_14)
                * ((challenges.get_challenge(InstructionLookupIndeterminate))
                    - ((((challenges.get_challenge(ProgramAddressWeight))
                        * (current_base_row[0]))
                        + ((challenges.get_challenge(ProgramInstructionWeight))
                            * (current_base_row[1])))
                        + ((challenges.get_challenge(ProgramNextInstructionWeight))
                            * (next_base_row[1])))))
                - (current_base_row[2])))
            + ((current_base_row[3]) * (node_14))];
        base_constraints
            .into_iter()
            .map(|bfe| bfe.lift())
            .chain(ext_constraints.into_iter())
            .collect()
    }

    #[inline]
    #[allow(unused_variables)]
    fn evaluate_terminal_constraints(
        base_row: ArrayView1<BFieldElement>,
        ext_row: ArrayView1<XFieldElement>,
        challenges: &Challenges,
    ) -> Vec<XFieldElement> {
        vec![]
    }
}

impl Evaluable<XFieldElement> for ExtProgramTable {
    #[inline]
    #[allow(unused_variables)]
    fn evaluate_initial_constraints(
        base_row: ArrayView1<XFieldElement>,
        ext_row: ArrayView1<XFieldElement>,
        challenges: &Challenges,
    ) -> Vec<XFieldElement> {
        let base_constraints = [base_row[0]];
        let ext_constraints = [ext_row[0]];
        base_constraints
            .into_iter()
            .chain(ext_constraints.into_iter())
            .collect()
    }

    #[inline]
    #[allow(unused_variables)]
    fn evaluate_consistency_constraints(
        base_row: ArrayView1<XFieldElement>,
        ext_row: ArrayView1<XFieldElement>,
        challenges: &Challenges,
    ) -> Vec<XFieldElement> {
        let base_constraints = [(base_row[3]) * ((base_row[3]) - (BFieldElement::new(1)))];
        let ext_constraints = [];
        base_constraints
            .into_iter()
            .chain(ext_constraints.into_iter())
            .collect()
    }

    #[inline]
    #[allow(unused_variables)]
    fn evaluate_transition_constraints(
        current_base_row: ArrayView1<XFieldElement>,
        current_ext_row: ArrayView1<XFieldElement>,
        next_base_row: ArrayView1<XFieldElement>,
        next_ext_row: ArrayView1<XFieldElement>,
        challenges: &Challenges,
    ) -> Vec<XFieldElement> {
        let node_14 = (next_ext_row[0]) - (current_ext_row[0]);

        let base_constraints = [
            (next_base_row[0]) - ((current_base_row[0]) + (BFieldElement::new(1))),
            (current_base_row[3]) * ((next_base_row[3]) - (current_base_row[3])),
        ];
        let ext_constraints = [(((BFieldElement::new(1)) - (current_base_row[3]))
            * (((node_14)
                * ((challenges.get_challenge(InstructionLookupIndeterminate))
                    - ((((challenges.get_challenge(ProgramAddressWeight))
                        * (current_base_row[0]))
                        + ((challenges.get_challenge(ProgramInstructionWeight))
                            * (current_base_row[1])))
                        + ((challenges.get_challenge(ProgramNextInstructionWeight))
                            * (next_base_row[1])))))
                - (current_base_row[2])))
            + ((current_base_row[3]) * (node_14))];
        base_constraints
            .into_iter()
            .chain(ext_constraints.into_iter())
            .collect()
    }

    #[inline]
    #[allow(unused_variables)]
    fn evaluate_terminal_constraints(
        base_row: ArrayView1<XFieldElement>,
        ext_row: ArrayView1<XFieldElement>,
        challenges: &Challenges,
    ) -> Vec<XFieldElement> {
        vec![]
    }
}

impl Quotientable for ExtProgramTable {
    fn num_initial_quotients() -> usize {
        2
    }

    fn num_consistency_quotients() -> usize {
        1
    }

    fn num_transition_quotients() -> usize {
        3
    }

    fn num_terminal_quotients() -> usize {
        0
    }

    #[allow(unused_variables)]
    fn initial_quotient_degree_bounds(interpolant_degree: Degree) -> Vec<Degree> {
        let zerofier_degree = 1 as Degree;
        [
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
        ]
        .to_vec()
    }

    #[allow(unused_variables)]
    fn consistency_quotient_degree_bounds(
        interpolant_degree: Degree,
        padded_height: usize,
    ) -> Vec<Degree> {
        let zerofier_degree = padded_height as Degree;
        [interpolant_degree * 2 as Degree - zerofier_degree].to_vec()
    }

    #[allow(unused_variables)]
    fn transition_quotient_degree_bounds(
        interpolant_degree: Degree,
        padded_height: usize,
    ) -> Vec<Degree> {
        let zerofier_degree = padded_height as Degree - 1;
        [
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
        ]
        .to_vec()
    }

    #[allow(unused_variables)]
    fn terminal_quotient_degree_bounds(interpolant_degree: Degree) -> Vec<Degree> {
        let zerofier_degree = 1 as Degree;
        [].to_vec()
    }
}
