use ndarray::ArrayView1;
use twenty_first::shared_math::b_field_element::BFieldElement;
use twenty_first::shared_math::mpolynomial::Degree;
use twenty_first::shared_math::x_field_element::XFieldElement;

use crate::table::challenges::ChallengeId::*;
use crate::table::challenges::Challenges;
use crate::table::extension_table::Evaluable;
use crate::table::extension_table::Quotientable;
use crate::table::jump_stack_table::ExtJumpStackTable;

// This file has been auto-generated. Any modifications _will_ be lost.
// To re-generate, execute:
// `cargo run --bin constraint-evaluation-generator`
impl Evaluable<BFieldElement> for ExtJumpStackTable {
    #[inline]
    #[allow(unused_variables)]
    fn evaluate_initial_constraints(
        base_row: ArrayView1<BFieldElement>,
        ext_row: ArrayView1<XFieldElement>,
        challenges: &Challenges,
    ) -> Vec<XFieldElement> {
        let base_constraints = [base_row[57], base_row[59], base_row[60], base_row[61]];
        let ext_constraints = [
            (ext_row[20])
                - ((challenges.get_challenge(JumpStackIndeterminate))
                    - ((challenges.get_challenge(JumpStackCiWeight)) * (base_row[58]))),
            ext_row[21],
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
        let node_17 = (next_base_row[59]) - (current_base_row[59]);
        let node_18 = (node_17) - (BFieldElement::new(1));
        let node_20 = (current_base_row[58]) - (BFieldElement::new(24));
        let node_21 = (node_18) * (node_20);
        let node_26 = (next_base_row[57]) - (current_base_row[57]);
        let node_50 = (next_ext_row[21]) - (current_ext_row[21]);

        let base_constraints = [
            (node_18) * (node_17),
            (node_21) * ((next_base_row[60]) - (current_base_row[60])),
            (node_21) * ((next_base_row[61]) - (current_base_row[61])),
            (((node_18) * ((node_26) - (BFieldElement::new(1))))
                * ((current_base_row[58]) - (BFieldElement::new(25))))
                * (node_20),
        ];
        let ext_constraints = [
            (next_ext_row[20])
                - ((current_ext_row[20])
                    * ((challenges.get_challenge(JumpStackIndeterminate))
                        - ((((((challenges.get_challenge(JumpStackClkWeight))
                            * (next_base_row[57]))
                            + ((challenges.get_challenge(JumpStackCiWeight))
                                * (next_base_row[58])))
                            + ((challenges.get_challenge(JumpStackJspWeight))
                                * (next_base_row[59])))
                            + ((challenges.get_challenge(JumpStackJsoWeight))
                                * (next_base_row[60])))
                            + ((challenges.get_challenge(JumpStackJsdWeight))
                                * (next_base_row[61]))))),
            ((node_18)
                * (((node_50)
                    * ((challenges.get_challenge(ClockJumpDifferenceLookupIndeterminate))
                        - (node_26)))
                    - (BFieldElement::new(1))))
                + ((node_17) * (node_50)),
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

impl Evaluable<XFieldElement> for ExtJumpStackTable {
    #[inline]
    #[allow(unused_variables)]
    fn evaluate_initial_constraints(
        base_row: ArrayView1<XFieldElement>,
        ext_row: ArrayView1<XFieldElement>,
        challenges: &Challenges,
    ) -> Vec<XFieldElement> {
        let base_constraints = [base_row[57], base_row[59], base_row[60], base_row[61]];
        let ext_constraints = [
            (ext_row[20])
                - ((challenges.get_challenge(JumpStackIndeterminate))
                    - ((challenges.get_challenge(JumpStackCiWeight)) * (base_row[58]))),
            ext_row[21],
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
        let node_17 = (next_base_row[59]) - (current_base_row[59]);
        let node_18 = (node_17) - (BFieldElement::new(1));
        let node_20 = (current_base_row[58]) - (BFieldElement::new(24));
        let node_21 = (node_18) * (node_20);
        let node_26 = (next_base_row[57]) - (current_base_row[57]);
        let node_50 = (next_ext_row[21]) - (current_ext_row[21]);

        let base_constraints = [
            (node_18) * (node_17),
            (node_21) * ((next_base_row[60]) - (current_base_row[60])),
            (node_21) * ((next_base_row[61]) - (current_base_row[61])),
            (((node_18) * ((node_26) - (BFieldElement::new(1))))
                * ((current_base_row[58]) - (BFieldElement::new(25))))
                * (node_20),
        ];
        let ext_constraints = [
            (next_ext_row[20])
                - ((current_ext_row[20])
                    * ((challenges.get_challenge(JumpStackIndeterminate))
                        - ((((((challenges.get_challenge(JumpStackClkWeight))
                            * (next_base_row[57]))
                            + ((challenges.get_challenge(JumpStackCiWeight))
                                * (next_base_row[58])))
                            + ((challenges.get_challenge(JumpStackJspWeight))
                                * (next_base_row[59])))
                            + ((challenges.get_challenge(JumpStackJsoWeight))
                                * (next_base_row[60])))
                            + ((challenges.get_challenge(JumpStackJsdWeight))
                                * (next_base_row[61]))))),
            ((node_18)
                * (((node_50)
                    * ((challenges.get_challenge(ClockJumpDifferenceLookupIndeterminate))
                        - (node_26)))
                    - (BFieldElement::new(1))))
                + ((node_17) * (node_50)),
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

impl Quotientable for ExtJumpStackTable {
    fn num_initial_quotients() -> usize {
        6
    }

    fn num_consistency_quotients() -> usize {
        0
    }

    fn num_transition_quotients() -> usize {
        6
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
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 4 as Degree - zerofier_degree,
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
