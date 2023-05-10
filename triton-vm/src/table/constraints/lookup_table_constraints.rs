use ndarray::ArrayView1;
use twenty_first::shared_math::b_field_element::BFieldElement;
use twenty_first::shared_math::mpolynomial::Degree;
use twenty_first::shared_math::x_field_element::XFieldElement;

use crate::table::challenges::ChallengeId::*;
use crate::table::challenges::Challenges;
use crate::table::extension_table::Evaluable;
use crate::table::extension_table::Quotientable;
use crate::table::lookup_table::ExtLookupTable;

// This file has been auto-generated. Any modifications _will_ be lost.
// To re-generate, execute:
// `cargo run --bin constraint-evaluation-generator`
impl Evaluable<BFieldElement> for ExtLookupTable {
    #[inline]
    #[allow(unused_variables)]
    fn evaluate_initial_constraints(
        base_row: ArrayView1<BFieldElement>,
        ext_row: ArrayView1<XFieldElement>,
        challenges: &Challenges,
    ) -> Vec<XFieldElement> {
        let base_constraints = [base_row[135]];
        let ext_constraints = [
            ((ext_row[43])
                * ((challenges.get_challenge(CascadeLookupIndeterminate))
                    - ((base_row[136]) * (challenges.get_challenge(LookupTableOutputWeight)))))
                - (base_row[137]),
            ((ext_row[44]) - (challenges.get_challenge(LookupTablePublicIndeterminate)))
                - (base_row[136]),
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
        let base_constraints = [(base_row[134]) * ((BFieldElement::new(1)) - (base_row[134]))];
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
        let node_11 = (BFieldElement::new(1)) - (next_base_row[134]);
        let node_24 = (next_ext_row[43]) - (current_ext_row[43]);

        let base_constraints = [
            (current_base_row[134]) * (node_11),
            ((next_base_row[134]) * (next_base_row[135]))
                + ((node_11)
                    * (((next_base_row[135]) - (current_base_row[135])) - (BFieldElement::new(1)))),
        ];
        let ext_constraints = [
            ((node_11)
                * (((node_24)
                    * ((challenges.get_challenge(CascadeLookupIndeterminate))
                        - (((next_base_row[135])
                            * (challenges.get_challenge(LookupTableInputWeight)))
                            + ((next_base_row[136])
                                * (challenges.get_challenge(LookupTableOutputWeight))))))
                    - (next_base_row[137])))
                + ((next_base_row[134]) * (node_24)),
            ((node_11)
                * (((next_ext_row[44])
                    - ((current_ext_row[44])
                        * (challenges.get_challenge(LookupTablePublicIndeterminate))))
                    - (next_base_row[136])))
                + ((next_base_row[134]) * ((next_ext_row[44]) - (current_ext_row[44]))),
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
        let base_constraints: [BFieldElement; 0] = [];
        let ext_constraints =
            [(ext_row[44]) - (challenges.get_challenge(LookupTablePublicTerminal))];
        base_constraints
            .into_iter()
            .map(|bfe| bfe.lift())
            .chain(ext_constraints.into_iter())
            .collect()
    }
}

impl Evaluable<XFieldElement> for ExtLookupTable {
    #[inline]
    #[allow(unused_variables)]
    fn evaluate_initial_constraints(
        base_row: ArrayView1<XFieldElement>,
        ext_row: ArrayView1<XFieldElement>,
        challenges: &Challenges,
    ) -> Vec<XFieldElement> {
        let base_constraints = [base_row[135]];
        let ext_constraints = [
            ((ext_row[43])
                * ((challenges.get_challenge(CascadeLookupIndeterminate))
                    - ((base_row[136]) * (challenges.get_challenge(LookupTableOutputWeight)))))
                - (base_row[137]),
            ((ext_row[44]) - (challenges.get_challenge(LookupTablePublicIndeterminate)))
                - (base_row[136]),
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
        let base_constraints = [(base_row[134]) * ((BFieldElement::new(1)) - (base_row[134]))];
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
        let node_11 = (BFieldElement::new(1)) - (next_base_row[134]);
        let node_24 = (next_ext_row[43]) - (current_ext_row[43]);

        let base_constraints = [
            (current_base_row[134]) * (node_11),
            ((next_base_row[134]) * (next_base_row[135]))
                + ((node_11)
                    * (((next_base_row[135]) - (current_base_row[135])) - (BFieldElement::new(1)))),
        ];
        let ext_constraints = [
            ((node_11)
                * (((node_24)
                    * ((challenges.get_challenge(CascadeLookupIndeterminate))
                        - (((next_base_row[135])
                            * (challenges.get_challenge(LookupTableInputWeight)))
                            + ((next_base_row[136])
                                * (challenges.get_challenge(LookupTableOutputWeight))))))
                    - (next_base_row[137])))
                + ((next_base_row[134]) * (node_24)),
            ((node_11)
                * (((next_ext_row[44])
                    - ((current_ext_row[44])
                        * (challenges.get_challenge(LookupTablePublicIndeterminate))))
                    - (next_base_row[136])))
                + ((next_base_row[134]) * ((next_ext_row[44]) - (current_ext_row[44]))),
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
        let base_constraints = [];
        let ext_constraints =
            [(ext_row[44]) - (challenges.get_challenge(LookupTablePublicTerminal))];
        base_constraints
            .into_iter()
            .chain(ext_constraints.into_iter())
            .collect()
    }
}

impl Quotientable for ExtLookupTable {
    fn num_initial_quotients() -> usize {
        3
    }

    fn num_consistency_quotients() -> usize {
        1
    }

    fn num_transition_quotients() -> usize {
        4
    }

    fn num_terminal_quotients() -> usize {
        1
    }

    #[allow(unused_variables)]
    fn initial_quotient_degree_bounds(interpolant_degree: Degree) -> Vec<Degree> {
        let zerofier_degree = 1 as Degree;
        [
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
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
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
        ]
        .to_vec()
    }

    #[allow(unused_variables)]
    fn terminal_quotient_degree_bounds(interpolant_degree: Degree) -> Vec<Degree> {
        let zerofier_degree = 1 as Degree;
        [interpolant_degree * 1 as Degree - zerofier_degree].to_vec()
    }
}
