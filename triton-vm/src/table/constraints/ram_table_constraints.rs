use ndarray::ArrayView1;
use twenty_first::shared_math::b_field_element::BFieldElement;
use twenty_first::shared_math::mpolynomial::Degree;
use twenty_first::shared_math::x_field_element::XFieldElement;

use crate::table::challenges::ChallengeId::*;
use crate::table::challenges::Challenges;
use crate::table::extension_table::Evaluable;
use crate::table::extension_table::Quotientable;
use crate::table::ram_table::ExtRamTable;

// This file has been auto-generated. Any modifications _will_ be lost.
// To re-generate, execute:
// `cargo run --bin constraint-evaluation-generator`
impl Evaluable<BFieldElement> for ExtRamTable {
    #[inline]
    #[allow(unused_variables)]
    fn evaluate_initial_constraints(
        base_row: ArrayView1<BFieldElement>,
        ext_row: ArrayView1<XFieldElement>,
        challenges: &Challenges,
    ) -> Vec<XFieldElement> {
        let base_constraints = [
            (base_row[53]) * ((BFieldElement::new(26)) - (base_row[51])),
            base_row[55],
        ];
        let ext_constraints = [
            ext_row[16],
            (ext_row[17]) - (base_row[56]),
            (ext_row[14])
                - ((challenges.get_challenge(RamTableBezoutRelationIndeterminate))
                    - (base_row[52])),
            (ext_row[15]) - (BFieldElement::new(1)),
            (ext_row[18])
                - ((challenges.get_challenge(RamIndeterminate))
                    - (((((base_row[50]) * (challenges.get_challenge(RamClkWeight)))
                        + ((base_row[52]) * (challenges.get_challenge(RamRampWeight))))
                        + ((base_row[53]) * (challenges.get_challenge(RamRamvWeight))))
                        + ((base_row[51])
                            * (challenges.get_challenge(RamPreviousInstructionWeight))))),
            ext_row[19],
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
        let node_31 = (next_base_row[52]) - (current_base_row[52]);
        let node_32 = (node_31) * (current_base_row[54]);
        let node_37 = (BFieldElement::new(1)) - (node_32);
        let node_33 = (node_32) - (BFieldElement::new(1));
        let node_38 = (BFieldElement::new(26)) - (next_base_row[51]);
        let node_48 =
            (challenges.get_challenge(RamTableBezoutRelationIndeterminate)) - (next_base_row[52]);
        let node_86 = (next_ext_row[19]) - (current_ext_row[19]);

        let base_constraints = [
            (current_base_row[54]) * (node_33),
            (node_31) * (node_33),
            ((node_37) * (node_38)) * ((next_base_row[53]) - (current_base_row[53])),
            ((node_31) * (node_38)) * (next_base_row[53]),
            (node_37) * ((next_base_row[55]) - (current_base_row[55])),
            (node_37) * ((next_base_row[56]) - (current_base_row[56])),
        ];
        let ext_constraints = [
            ((node_31) * ((next_ext_row[14]) - ((current_ext_row[14]) * (node_48))))
                + ((node_37) * ((next_ext_row[14]) - (current_ext_row[14]))),
            ((node_31)
                * (((next_ext_row[15]) - (current_ext_row[14]))
                    - ((node_48) * (current_ext_row[15]))))
                + ((node_37) * ((next_ext_row[15]) - (current_ext_row[15]))),
            ((node_31)
                * (((next_ext_row[16])
                    - ((challenges.get_challenge(RamTableBezoutRelationIndeterminate))
                        * (current_ext_row[16])))
                    - (next_base_row[55])))
                + ((node_37) * ((next_ext_row[16]) - (current_ext_row[16]))),
            ((node_31)
                * (((next_ext_row[17])
                    - ((challenges.get_challenge(RamTableBezoutRelationIndeterminate))
                        * (current_ext_row[17])))
                    - (next_base_row[56])))
                + ((node_37) * ((next_ext_row[17]) - (current_ext_row[17]))),
            (next_ext_row[18])
                - ((current_ext_row[18])
                    * ((challenges.get_challenge(RamIndeterminate))
                        - (((((next_base_row[50]) * (challenges.get_challenge(RamClkWeight)))
                            + ((next_base_row[52])
                                * (challenges.get_challenge(RamRampWeight))))
                            + ((next_base_row[53]) * (challenges.get_challenge(RamRamvWeight))))
                            + ((next_base_row[51])
                                * (challenges.get_challenge(RamPreviousInstructionWeight)))))),
            ((node_37)
                * (((node_86)
                    * ((challenges.get_challenge(ClockJumpDifferenceLookupIndeterminate))
                        - ((next_base_row[50]) - (current_base_row[50]))))
                    - (BFieldElement::new(1))))
                + ((node_31) * (node_86)),
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
        let ext_constraints = [
            (((ext_row[16]) * (ext_row[14])) + ((ext_row[17]) * (ext_row[15])))
                - (BFieldElement::new(1)),
        ];
        base_constraints
            .into_iter()
            .map(|bfe| bfe.lift())
            .chain(ext_constraints.into_iter())
            .collect()
    }
}

impl Evaluable<XFieldElement> for ExtRamTable {
    #[inline]
    #[allow(unused_variables)]
    fn evaluate_initial_constraints(
        base_row: ArrayView1<XFieldElement>,
        ext_row: ArrayView1<XFieldElement>,
        challenges: &Challenges,
    ) -> Vec<XFieldElement> {
        let base_constraints = [
            (base_row[53]) * ((BFieldElement::new(26)) - (base_row[51])),
            base_row[55],
        ];
        let ext_constraints = [
            ext_row[16],
            (ext_row[17]) - (base_row[56]),
            (ext_row[14])
                - ((challenges.get_challenge(RamTableBezoutRelationIndeterminate))
                    - (base_row[52])),
            (ext_row[15]) - (BFieldElement::new(1)),
            (ext_row[18])
                - ((challenges.get_challenge(RamIndeterminate))
                    - (((((base_row[50]) * (challenges.get_challenge(RamClkWeight)))
                        + ((base_row[52]) * (challenges.get_challenge(RamRampWeight))))
                        + ((base_row[53]) * (challenges.get_challenge(RamRamvWeight))))
                        + ((base_row[51])
                            * (challenges.get_challenge(RamPreviousInstructionWeight))))),
            ext_row[19],
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
        let node_31 = (next_base_row[52]) - (current_base_row[52]);
        let node_32 = (node_31) * (current_base_row[54]);
        let node_37 = (BFieldElement::new(1)) - (node_32);
        let node_33 = (node_32) - (BFieldElement::new(1));
        let node_38 = (BFieldElement::new(26)) - (next_base_row[51]);
        let node_48 =
            (challenges.get_challenge(RamTableBezoutRelationIndeterminate)) - (next_base_row[52]);
        let node_86 = (next_ext_row[19]) - (current_ext_row[19]);

        let base_constraints = [
            (current_base_row[54]) * (node_33),
            (node_31) * (node_33),
            ((node_37) * (node_38)) * ((next_base_row[53]) - (current_base_row[53])),
            ((node_31) * (node_38)) * (next_base_row[53]),
            (node_37) * ((next_base_row[55]) - (current_base_row[55])),
            (node_37) * ((next_base_row[56]) - (current_base_row[56])),
        ];
        let ext_constraints = [
            ((node_31) * ((next_ext_row[14]) - ((current_ext_row[14]) * (node_48))))
                + ((node_37) * ((next_ext_row[14]) - (current_ext_row[14]))),
            ((node_31)
                * (((next_ext_row[15]) - (current_ext_row[14]))
                    - ((node_48) * (current_ext_row[15]))))
                + ((node_37) * ((next_ext_row[15]) - (current_ext_row[15]))),
            ((node_31)
                * (((next_ext_row[16])
                    - ((challenges.get_challenge(RamTableBezoutRelationIndeterminate))
                        * (current_ext_row[16])))
                    - (next_base_row[55])))
                + ((node_37) * ((next_ext_row[16]) - (current_ext_row[16]))),
            ((node_31)
                * (((next_ext_row[17])
                    - ((challenges.get_challenge(RamTableBezoutRelationIndeterminate))
                        * (current_ext_row[17])))
                    - (next_base_row[56])))
                + ((node_37) * ((next_ext_row[17]) - (current_ext_row[17]))),
            (next_ext_row[18])
                - ((current_ext_row[18])
                    * ((challenges.get_challenge(RamIndeterminate))
                        - (((((next_base_row[50]) * (challenges.get_challenge(RamClkWeight)))
                            + ((next_base_row[52])
                                * (challenges.get_challenge(RamRampWeight))))
                            + ((next_base_row[53]) * (challenges.get_challenge(RamRamvWeight))))
                            + ((next_base_row[51])
                                * (challenges.get_challenge(RamPreviousInstructionWeight)))))),
            ((node_37)
                * (((node_86)
                    * ((challenges.get_challenge(ClockJumpDifferenceLookupIndeterminate))
                        - ((next_base_row[50]) - (current_base_row[50]))))
                    - (BFieldElement::new(1))))
                + ((node_31) * (node_86)),
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
        let ext_constraints = [
            (((ext_row[16]) * (ext_row[14])) + ((ext_row[17]) * (ext_row[15])))
                - (BFieldElement::new(1)),
        ];
        base_constraints
            .into_iter()
            .chain(ext_constraints.into_iter())
            .collect()
    }
}

impl Quotientable for ExtRamTable {
    fn num_initial_quotients() -> usize {
        8
    }

    fn num_consistency_quotients() -> usize {
        0
    }

    fn num_transition_quotients() -> usize {
        12
    }

    fn num_terminal_quotients() -> usize {
        1
    }

    #[allow(unused_variables)]
    fn initial_quotient_degree_bounds(interpolant_degree: Degree) -> Vec<Degree> {
        let zerofier_degree = 1 as Degree;
        [
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
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
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 4 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 4 as Degree - zerofier_degree,
        ]
        .to_vec()
    }

    #[allow(unused_variables)]
    fn terminal_quotient_degree_bounds(interpolant_degree: Degree) -> Vec<Degree> {
        let zerofier_degree = 1 as Degree;
        [interpolant_degree * 2 as Degree - zerofier_degree].to_vec()
    }
}
