use ndarray::ArrayView1;
use twenty_first::shared_math::b_field_element::BFieldElement;
use twenty_first::shared_math::mpolynomial::Degree;
use twenty_first::shared_math::x_field_element::XFieldElement;

use crate::table::challenges::ChallengeId::*;
use crate::table::challenges::Challenges;
use crate::table::extension_table::Evaluable;
use crate::table::extension_table::Quotientable;
use crate::table::op_stack_table::ExtOpStackTable;

// This file has been auto-generated. Any modifications _will_ be lost.
// To re-generate, execute:
// `cargo run --bin constraint-evaluation-generator`
impl Evaluable<BFieldElement> for ExtOpStackTable {
    #[inline]
    #[allow(unused_variables)]
    fn evaluate_initial_constraints(
        base_row: ArrayView1<BFieldElement>,
        ext_row: ArrayView1<XFieldElement>,
        challenges: &Challenges,
    ) -> Vec<XFieldElement> {
        let base_constraints = [
            base_row[46],
            base_row[49],
            (base_row[48]) - (BFieldElement::new(16)),
        ];
        let ext_constraints = [
            (ext_row[12])
                - ((challenges.get_challenge(OpStackIndeterminate))
                    - (((challenges.get_challenge(OpStackIb1Weight)) * (base_row[47]))
                        + ((challenges.get_challenge(OpStackOspWeight))
                            * (BFieldElement::new(16))))),
            ext_row[13],
        ];
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
        vec![]
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
        let node_13 = (next_base_row[48]) - (current_base_row[48]);
        let node_14 = (node_13) - (BFieldElement::new(1));
        let node_35 = (next_ext_row[13]) - (current_ext_row[13]);

        let base_constraints = [
            (node_14) * (node_13),
            ((node_14) * ((next_base_row[49]) - (current_base_row[49])))
                * ((BFieldElement::new(1)) - (current_base_row[47])),
        ];
        let ext_constraints = [
            (next_ext_row[12])
                - ((current_ext_row[12])
                    * ((challenges.get_challenge(OpStackIndeterminate))
                        - (((((challenges.get_challenge(OpStackClkWeight))
                            * (next_base_row[46]))
                            + ((challenges.get_challenge(OpStackIb1Weight))
                                * (next_base_row[47])))
                            + ((challenges.get_challenge(OpStackOspWeight))
                                * (next_base_row[48])))
                            + ((challenges.get_challenge(OpStackOsvWeight))
                                * (next_base_row[49]))))),
            ((node_14)
                * (((node_35)
                    * ((challenges.get_challenge(ClockJumpDifferenceLookupIndeterminate))
                        - ((next_base_row[46]) - (current_base_row[46]))))
                    - (BFieldElement::new(1))))
                + ((node_13) * (node_35)),
        ];
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

impl Evaluable<XFieldElement> for ExtOpStackTable {
    #[inline]
    #[allow(unused_variables)]
    fn evaluate_initial_constraints(
        base_row: ArrayView1<XFieldElement>,
        ext_row: ArrayView1<XFieldElement>,
        challenges: &Challenges,
    ) -> Vec<XFieldElement> {
        let base_constraints = [
            base_row[46],
            base_row[49],
            (base_row[48]) - (BFieldElement::new(16)),
        ];
        let ext_constraints = [
            (ext_row[12])
                - ((challenges.get_challenge(OpStackIndeterminate))
                    - (((challenges.get_challenge(OpStackIb1Weight)) * (base_row[47]))
                        + ((challenges.get_challenge(OpStackOspWeight))
                            * (BFieldElement::new(16))))),
            ext_row[13],
        ];
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
        vec![]
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
        let node_13 = (next_base_row[48]) - (current_base_row[48]);
        let node_14 = (node_13) - (BFieldElement::new(1));
        let node_35 = (next_ext_row[13]) - (current_ext_row[13]);

        let base_constraints = [
            (node_14) * (node_13),
            ((node_14) * ((next_base_row[49]) - (current_base_row[49])))
                * ((BFieldElement::new(1)) - (current_base_row[47])),
        ];
        let ext_constraints = [
            (next_ext_row[12])
                - ((current_ext_row[12])
                    * ((challenges.get_challenge(OpStackIndeterminate))
                        - (((((challenges.get_challenge(OpStackClkWeight))
                            * (next_base_row[46]))
                            + ((challenges.get_challenge(OpStackIb1Weight))
                                * (next_base_row[47])))
                            + ((challenges.get_challenge(OpStackOspWeight))
                                * (next_base_row[48])))
                            + ((challenges.get_challenge(OpStackOsvWeight))
                                * (next_base_row[49]))))),
            ((node_14)
                * (((node_35)
                    * ((challenges.get_challenge(ClockJumpDifferenceLookupIndeterminate))
                        - ((next_base_row[46]) - (current_base_row[46]))))
                    - (BFieldElement::new(1))))
                + ((node_13) * (node_35)),
        ];
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

impl Quotientable for ExtOpStackTable {
    fn num_initial_quotients() -> usize {
        5
    }

    fn num_consistency_quotients() -> usize {
        0
    }

    fn num_transition_quotients() -> usize {
        4
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
            interpolant_degree * 1 as Degree - zerofier_degree,
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
        [].to_vec()
    }

    #[allow(unused_variables)]
    fn transition_quotient_degree_bounds(
        interpolant_degree: Degree,
        padded_height: usize,
    ) -> Vec<Degree> {
        let zerofier_degree = padded_height as Degree - 1;
        [
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
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
