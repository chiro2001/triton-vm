use ndarray::ArrayView1;
use twenty_first::shared_math::b_field_element::BFieldElement;
use twenty_first::shared_math::mpolynomial::Degree;
use twenty_first::shared_math::x_field_element::XFieldElement;

use crate::table::cascade_table::ExtCascadeTable;
use crate::table::challenges::ChallengeId::*;
use crate::table::challenges::Challenges;
use crate::table::extension_table::Evaluable;
use crate::table::extension_table::Quotientable;

// This file has been auto-generated. Any modifications _will_ be lost.
// To re-generate, execute:
// `cargo run --bin constraint-evaluation-generator`
impl Evaluable<BFieldElement> for ExtCascadeTable {
    #[inline]
    #[allow(unused_variables)]
    fn evaluate_initial_constraints(
        base_row: ArrayView1<BFieldElement>,
        ext_row: ArrayView1<XFieldElement>,
        challenges: &Challenges,
    ) -> Vec<XFieldElement> {
        let node_29 = (BFieldElement::new(1)) - (base_row[128]);
        let node_35 = ((challenges.get_challenge(LookupTableInputWeight)) * (base_row[130]))
            + ((challenges.get_challenge(LookupTableOutputWeight)) * (base_row[132]));
        let node_38 = ((challenges.get_challenge(LookupTableInputWeight)) * (base_row[129]))
            + ((challenges.get_challenge(LookupTableOutputWeight)) * (base_row[131]));

        let base_constraints: [BFieldElement; 0] = [];
        let ext_constraints = [
            ((node_29)
                * (((ext_row[41])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (((BFieldElement::new(256)) * (base_row[129]))
                                + (base_row[130])))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (((BFieldElement::new(256)) * (base_row[131]))
                                    + (base_row[132]))))))
                    - (base_row[133])))
                + ((base_row[128]) * (ext_row[41])),
            ((node_29)
                * ((((((ext_row[42])
                    * ((challenges.get_challenge(CascadeLookupIndeterminate)) - (node_35)))
                    * ((challenges.get_challenge(CascadeLookupIndeterminate)) - (node_38)))
                    - ((BFieldElement::new(2))
                        * (challenges.get_challenge(CascadeLookupIndeterminate))))
                    + (node_35))
                    + (node_38)))
                + ((base_row[128]) * (ext_row[42])),
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
        let base_constraints = [(base_row[128]) * ((BFieldElement::new(1)) - (base_row[128]))];
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
        let node_20 = (BFieldElement::new(1)) - (next_base_row[128]);
        let node_29 = (next_ext_row[41]) - (current_ext_row[41]);
        let node_42 = (next_ext_row[42]) - (current_ext_row[42]);
        let node_38 = ((challenges.get_challenge(LookupTableInputWeight)) * (next_base_row[130]))
            + ((challenges.get_challenge(LookupTableOutputWeight)) * (next_base_row[132]));
        let node_41 = ((challenges.get_challenge(LookupTableInputWeight)) * (next_base_row[129]))
            + ((challenges.get_challenge(LookupTableOutputWeight)) * (next_base_row[131]));

        let base_constraints = [(current_base_row[128]) * (node_20)];
        let ext_constraints = [
            ((node_20)
                * (((node_29)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (((BFieldElement::new(256)) * (next_base_row[129]))
                                + (next_base_row[130])))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (((BFieldElement::new(256)) * (next_base_row[131]))
                                    + (next_base_row[132]))))))
                    - (next_base_row[133])))
                + ((next_base_row[128]) * (node_29)),
            ((node_20)
                * ((((((node_42)
                    * ((challenges.get_challenge(CascadeLookupIndeterminate)) - (node_38)))
                    * ((challenges.get_challenge(CascadeLookupIndeterminate)) - (node_41)))
                    - ((BFieldElement::new(2))
                        * (challenges.get_challenge(CascadeLookupIndeterminate))))
                    + (node_38))
                    + (node_41)))
                + ((next_base_row[128]) * (node_42)),
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

impl Evaluable<XFieldElement> for ExtCascadeTable {
    #[inline]
    #[allow(unused_variables)]
    fn evaluate_initial_constraints(
        base_row: ArrayView1<XFieldElement>,
        ext_row: ArrayView1<XFieldElement>,
        challenges: &Challenges,
    ) -> Vec<XFieldElement> {
        let node_29 = (BFieldElement::new(1)) - (base_row[128]);
        let node_35 = ((challenges.get_challenge(LookupTableInputWeight)) * (base_row[130]))
            + ((challenges.get_challenge(LookupTableOutputWeight)) * (base_row[132]));
        let node_38 = ((challenges.get_challenge(LookupTableInputWeight)) * (base_row[129]))
            + ((challenges.get_challenge(LookupTableOutputWeight)) * (base_row[131]));

        let base_constraints = [];
        let ext_constraints = [
            ((node_29)
                * (((ext_row[41])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (((BFieldElement::new(256)) * (base_row[129]))
                                + (base_row[130])))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (((BFieldElement::new(256)) * (base_row[131]))
                                    + (base_row[132]))))))
                    - (base_row[133])))
                + ((base_row[128]) * (ext_row[41])),
            ((node_29)
                * ((((((ext_row[42])
                    * ((challenges.get_challenge(CascadeLookupIndeterminate)) - (node_35)))
                    * ((challenges.get_challenge(CascadeLookupIndeterminate)) - (node_38)))
                    - ((BFieldElement::new(2))
                        * (challenges.get_challenge(CascadeLookupIndeterminate))))
                    + (node_35))
                    + (node_38)))
                + ((base_row[128]) * (ext_row[42])),
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
        let base_constraints = [(base_row[128]) * ((BFieldElement::new(1)) - (base_row[128]))];
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
        let node_20 = (BFieldElement::new(1)) - (next_base_row[128]);
        let node_29 = (next_ext_row[41]) - (current_ext_row[41]);
        let node_42 = (next_ext_row[42]) - (current_ext_row[42]);
        let node_38 = ((challenges.get_challenge(LookupTableInputWeight)) * (next_base_row[130]))
            + ((challenges.get_challenge(LookupTableOutputWeight)) * (next_base_row[132]));
        let node_41 = ((challenges.get_challenge(LookupTableInputWeight)) * (next_base_row[129]))
            + ((challenges.get_challenge(LookupTableOutputWeight)) * (next_base_row[131]));

        let base_constraints = [(current_base_row[128]) * (node_20)];
        let ext_constraints = [
            ((node_20)
                * (((node_29)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (((BFieldElement::new(256)) * (next_base_row[129]))
                                + (next_base_row[130])))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (((BFieldElement::new(256)) * (next_base_row[131]))
                                    + (next_base_row[132]))))))
                    - (next_base_row[133])))
                + ((next_base_row[128]) * (node_29)),
            ((node_20)
                * ((((((node_42)
                    * ((challenges.get_challenge(CascadeLookupIndeterminate)) - (node_38)))
                    * ((challenges.get_challenge(CascadeLookupIndeterminate)) - (node_41)))
                    - ((BFieldElement::new(2))
                        * (challenges.get_challenge(CascadeLookupIndeterminate))))
                    + (node_38))
                    + (node_41)))
                + ((next_base_row[128]) * (node_42)),
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

impl Quotientable for ExtCascadeTable {
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
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 4 as Degree - zerofier_degree,
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
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 4 as Degree - zerofier_degree,
        ]
        .to_vec()
    }

    #[allow(unused_variables)]
    fn terminal_quotient_degree_bounds(interpolant_degree: Degree) -> Vec<Degree> {
        let zerofier_degree = 1 as Degree;
        [].to_vec()
    }
}
