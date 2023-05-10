use ndarray::ArrayView1;
use twenty_first::shared_math::b_field_element::BFieldElement;
use twenty_first::shared_math::mpolynomial::Degree;
use twenty_first::shared_math::x_field_element::XFieldElement;

use crate::table::challenges::ChallengeId::*;
use crate::table::challenges::Challenges;
use crate::table::extension_table::Evaluable;
use crate::table::extension_table::Quotientable;
use crate::table::u32_table::ExtU32Table;

// This file has been auto-generated. Any modifications _will_ be lost.
// To re-generate, execute:
// `cargo run --bin constraint-evaluation-generator`
impl Evaluable<BFieldElement> for ExtU32Table {
    #[inline]
    #[allow(unused_variables)]
    fn evaluate_initial_constraints(
        base_row: ArrayView1<BFieldElement>,
        ext_row: ArrayView1<XFieldElement>,
        challenges: &Challenges,
    ) -> Vec<XFieldElement> {
        let base_constraints: [BFieldElement; 0] = [];
        let ext_constraints = [
            (((base_row[138]) - (BFieldElement::new(1))) * (ext_row[45]))
                + ((base_row[138])
                    * (((ext_row[45])
                        * ((challenges.get_challenge(U32Indeterminate))
                            - (((((challenges.get_challenge(U32LhsWeight))
                                * (base_row[142]))
                                + ((challenges.get_challenge(U32RhsWeight))
                                    * (base_row[144])))
                                + ((challenges.get_challenge(U32CiWeight)) * (base_row[141])))
                                + ((challenges.get_challenge(U32ResultWeight))
                                    * (base_row[146])))))
                        - (base_row[147]))),
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
        let node_20 = (BFieldElement::new(1)) - ((base_row[142]) * (base_row[143]));
        let node_28 = (base_row[141]) - (BFieldElement::new(4));
        let node_24 = (BFieldElement::new(1)) - ((base_row[144]) * (base_row[145]));
        let node_31 = (base_row[141]) - (BFieldElement::new(20));
        let node_37 = (base_row[141]) - (BFieldElement::new(44));
        let node_40 = (base_row[141]) - (BFieldElement::new(60));
        let node_34 = (base_row[141]) - (BFieldElement::new(36));
        let node_54 = (node_28) * ((base_row[141]) - (BFieldElement::new(12)));
        let node_61 = (node_54) * (node_31);
        let node_42 = (base_row[138]) - (BFieldElement::new(1));
        let node_41 = ((((node_28) * (node_31)) * (node_34)) * (node_37)) * (node_40);
        let node_62 = (node_61) * (node_34);
        let node_68 = ((node_61) * (node_37)) * (node_40);

        let base_constraints = [
            (base_row[138]) * ((BFieldElement::new(1)) - (base_row[138])),
            (base_row[138]) * (base_row[139]),
            (BFieldElement::new(1))
                - ((base_row[140]) * ((base_row[139]) - (BFieldElement::new(33)))),
            (base_row[143]) * (node_20),
            (base_row[142]) * (node_20),
            (base_row[145]) * (node_24),
            (base_row[144]) * (node_24),
            ((((node_41) * (node_42)) * (node_20)) * (node_24))
                * ((base_row[146]) - (BFieldElement::new(2))),
            ((((node_41) * (base_row[138])) * (node_20)) * (node_24)) * (base_row[146]),
            ((((((node_54) * (node_34)) * (node_37)) * (node_40)) * (node_20)) * (node_24))
                * (base_row[146]),
            (((node_62) * (node_40)) * (node_24)) * ((base_row[146]) - (BFieldElement::new(1))),
            (((node_68) * (node_42)) * (node_20)) * ((base_row[146]) + (BFieldElement::new(1))),
            (((node_62) * (node_37)) * (node_20)) * (base_row[146]),
            ((node_68) * (base_row[138])) * (node_20),
            (node_42) * (base_row[147]),
        ];
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
        let node_27 = (next_base_row[138]) - (BFieldElement::new(1));
        let node_45 = (next_base_row[141]) - (BFieldElement::new(4));
        let node_48 = (next_base_row[141]) - (BFieldElement::new(20));
        let node_56 = (next_base_row[141]) - (BFieldElement::new(60));
        let node_23 = (current_base_row[144]) - ((BFieldElement::new(2)) * (next_base_row[144]));
        let node_51 = (next_base_row[141]) - (BFieldElement::new(36));
        let node_21 = (current_base_row[142]) - ((BFieldElement::new(2)) * (next_base_row[142]));
        let node_53 = (next_base_row[141]) - (BFieldElement::new(44));
        let node_89 = (node_45) * ((next_base_row[141]) - (BFieldElement::new(12)));
        let node_58 = (node_27) * (((((node_45) * (node_48)) * (node_51)) * (node_53)) * (node_56));
        let node_99 = (node_89) * (node_48);
        let node_59 = (next_base_row[146]) - (BFieldElement::new(1));
        let node_64 = (node_58) * (next_base_row[146]);
        let node_68 = (node_64) * (node_59);
        let node_112 = (node_99) * (node_51);
        let node_19 = (current_base_row[141]) - (BFieldElement::new(44));
        let node_42 = (node_23) - (BFieldElement::new(1));
        let node_114 = (node_27) * ((node_112) * (node_56));
        let node_33 = ((next_base_row[139]) - (current_base_row[139])) - (BFieldElement::new(1));
        let node_39 = (node_21) - (BFieldElement::new(1));
        let node_61 = (next_base_row[146]) - (BFieldElement::new(2));
        let node_66 = (current_base_row[146]) - (BFieldElement::new(1));
        let node_80 = (node_68)
            * ((((BFieldElement::new(1)) - (node_21)) - (node_23))
                + (((BFieldElement::new(2)) * (node_21)) * (node_23)));
        let node_102 = (node_27) * (((node_99) * (node_53)) * (node_56));
        let node_118 = (next_base_row[146]) * (next_base_row[146]);
        let node_130 = (next_ext_row[45]) - (current_ext_row[45]);

        let base_constraints = [
            ((next_base_row[138]) * (current_base_row[142])) * (node_19),
            (next_base_row[138]) * (current_base_row[144]),
            (node_27) * ((next_base_row[141]) - (current_base_row[141])),
            (((node_27) * (current_base_row[142])) * (node_19)) * (node_33),
            ((node_27) * (current_base_row[144])) * (node_33),
            (((node_27) * (node_19)) * (node_21)) * (node_39),
            ((node_27) * (node_23)) * (node_42),
            (((node_58) * (node_59)) * (node_61)) * (current_base_row[146]),
            ((node_64) * (node_61)) * (node_66),
            (((node_68) * (node_39)) * (node_23)) * (node_66),
            (((node_68) * (node_21)) * (node_42)) * (current_base_row[146]),
            ((node_80) * ((current_base_row[138]) - (BFieldElement::new(1))))
                * ((current_base_row[146]) - (BFieldElement::new(2))),
            ((node_80) * (current_base_row[138])) * (current_base_row[146]),
            ((node_27) * ((((node_89) * (node_51)) * (node_53)) * (node_56)))
                * (((current_base_row[146]) - ((BFieldElement::new(2)) * (next_base_row[146])))
                    - ((node_21) * (node_23))),
            (((node_102)
                * ((BFieldElement::new(1)) - ((next_base_row[142]) * (next_base_row[143]))))
                * (current_base_row[142]))
                * ((current_base_row[146]) - (current_base_row[139])),
            ((node_102) * (next_base_row[142])) * ((next_base_row[146]) - (current_base_row[146])),
            (node_114) * ((next_base_row[142]) - (current_base_row[142])),
            ((node_114) * (node_42)) * ((current_base_row[146]) - (node_118)),
            ((node_114) * (node_23))
                * ((current_base_row[146]) - ((node_118) * (current_base_row[142]))),
            ((node_27) * ((node_112) * (node_53)))
                * (((current_base_row[146]) - (next_base_row[146])) - (node_21)),
        ];
        let ext_constraints = [
            (node_27) * (node_130),
            (next_base_row[138])
                * (((node_130)
                    * ((challenges.get_challenge(U32Indeterminate))
                        - (((((challenges.get_challenge(U32CiWeight)) * (next_base_row[141]))
                            + ((challenges.get_challenge(U32LhsWeight))
                                * (next_base_row[142])))
                            + ((challenges.get_challenge(U32RhsWeight))
                                * (next_base_row[144])))
                            + ((challenges.get_challenge(U32ResultWeight))
                                * (next_base_row[146])))))
                    - (next_base_row[147])),
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
        let base_constraints = [
            (base_row[142]) * ((base_row[141]) - (BFieldElement::new(44))),
            base_row[144],
        ];
        let ext_constraints = [];
        base_constraints
            .into_iter()
            .map(|bfe| bfe.lift())
            .chain(ext_constraints.into_iter())
            .collect()
    }
}

impl Evaluable<XFieldElement> for ExtU32Table {
    #[inline]
    #[allow(unused_variables)]
    fn evaluate_initial_constraints(
        base_row: ArrayView1<XFieldElement>,
        ext_row: ArrayView1<XFieldElement>,
        challenges: &Challenges,
    ) -> Vec<XFieldElement> {
        let base_constraints = [];
        let ext_constraints = [
            (((base_row[138]) - (BFieldElement::new(1))) * (ext_row[45]))
                + ((base_row[138])
                    * (((ext_row[45])
                        * ((challenges.get_challenge(U32Indeterminate))
                            - (((((challenges.get_challenge(U32LhsWeight))
                                * (base_row[142]))
                                + ((challenges.get_challenge(U32RhsWeight))
                                    * (base_row[144])))
                                + ((challenges.get_challenge(U32CiWeight)) * (base_row[141])))
                                + ((challenges.get_challenge(U32ResultWeight))
                                    * (base_row[146])))))
                        - (base_row[147]))),
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
        let node_20 = (BFieldElement::new(1)) - ((base_row[142]) * (base_row[143]));
        let node_28 = (base_row[141]) - (BFieldElement::new(4));
        let node_24 = (BFieldElement::new(1)) - ((base_row[144]) * (base_row[145]));
        let node_31 = (base_row[141]) - (BFieldElement::new(20));
        let node_37 = (base_row[141]) - (BFieldElement::new(44));
        let node_40 = (base_row[141]) - (BFieldElement::new(60));
        let node_34 = (base_row[141]) - (BFieldElement::new(36));
        let node_54 = (node_28) * ((base_row[141]) - (BFieldElement::new(12)));
        let node_61 = (node_54) * (node_31);
        let node_42 = (base_row[138]) - (BFieldElement::new(1));
        let node_41 = ((((node_28) * (node_31)) * (node_34)) * (node_37)) * (node_40);
        let node_62 = (node_61) * (node_34);
        let node_68 = ((node_61) * (node_37)) * (node_40);

        let base_constraints = [
            (base_row[138]) * ((BFieldElement::new(1)) - (base_row[138])),
            (base_row[138]) * (base_row[139]),
            (BFieldElement::new(1))
                - ((base_row[140]) * ((base_row[139]) - (BFieldElement::new(33)))),
            (base_row[143]) * (node_20),
            (base_row[142]) * (node_20),
            (base_row[145]) * (node_24),
            (base_row[144]) * (node_24),
            ((((node_41) * (node_42)) * (node_20)) * (node_24))
                * ((base_row[146]) - (BFieldElement::new(2))),
            ((((node_41) * (base_row[138])) * (node_20)) * (node_24)) * (base_row[146]),
            ((((((node_54) * (node_34)) * (node_37)) * (node_40)) * (node_20)) * (node_24))
                * (base_row[146]),
            (((node_62) * (node_40)) * (node_24)) * ((base_row[146]) - (BFieldElement::new(1))),
            (((node_68) * (node_42)) * (node_20)) * ((base_row[146]) + (BFieldElement::new(1))),
            (((node_62) * (node_37)) * (node_20)) * (base_row[146]),
            ((node_68) * (base_row[138])) * (node_20),
            (node_42) * (base_row[147]),
        ];
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
        let node_27 = (next_base_row[138]) - (BFieldElement::new(1));
        let node_45 = (next_base_row[141]) - (BFieldElement::new(4));
        let node_48 = (next_base_row[141]) - (BFieldElement::new(20));
        let node_56 = (next_base_row[141]) - (BFieldElement::new(60));
        let node_23 = (current_base_row[144]) - ((BFieldElement::new(2)) * (next_base_row[144]));
        let node_51 = (next_base_row[141]) - (BFieldElement::new(36));
        let node_21 = (current_base_row[142]) - ((BFieldElement::new(2)) * (next_base_row[142]));
        let node_53 = (next_base_row[141]) - (BFieldElement::new(44));
        let node_89 = (node_45) * ((next_base_row[141]) - (BFieldElement::new(12)));
        let node_58 = (node_27) * (((((node_45) * (node_48)) * (node_51)) * (node_53)) * (node_56));
        let node_99 = (node_89) * (node_48);
        let node_59 = (next_base_row[146]) - (BFieldElement::new(1));
        let node_64 = (node_58) * (next_base_row[146]);
        let node_68 = (node_64) * (node_59);
        let node_112 = (node_99) * (node_51);
        let node_19 = (current_base_row[141]) - (BFieldElement::new(44));
        let node_42 = (node_23) - (BFieldElement::new(1));
        let node_114 = (node_27) * ((node_112) * (node_56));
        let node_33 = ((next_base_row[139]) - (current_base_row[139])) - (BFieldElement::new(1));
        let node_39 = (node_21) - (BFieldElement::new(1));
        let node_61 = (next_base_row[146]) - (BFieldElement::new(2));
        let node_66 = (current_base_row[146]) - (BFieldElement::new(1));
        let node_80 = (node_68)
            * ((((BFieldElement::new(1)) - (node_21)) - (node_23))
                + (((BFieldElement::new(2)) * (node_21)) * (node_23)));
        let node_102 = (node_27) * (((node_99) * (node_53)) * (node_56));
        let node_118 = (next_base_row[146]) * (next_base_row[146]);
        let node_130 = (next_ext_row[45]) - (current_ext_row[45]);

        let base_constraints = [
            ((next_base_row[138]) * (current_base_row[142])) * (node_19),
            (next_base_row[138]) * (current_base_row[144]),
            (node_27) * ((next_base_row[141]) - (current_base_row[141])),
            (((node_27) * (current_base_row[142])) * (node_19)) * (node_33),
            ((node_27) * (current_base_row[144])) * (node_33),
            (((node_27) * (node_19)) * (node_21)) * (node_39),
            ((node_27) * (node_23)) * (node_42),
            (((node_58) * (node_59)) * (node_61)) * (current_base_row[146]),
            ((node_64) * (node_61)) * (node_66),
            (((node_68) * (node_39)) * (node_23)) * (node_66),
            (((node_68) * (node_21)) * (node_42)) * (current_base_row[146]),
            ((node_80) * ((current_base_row[138]) - (BFieldElement::new(1))))
                * ((current_base_row[146]) - (BFieldElement::new(2))),
            ((node_80) * (current_base_row[138])) * (current_base_row[146]),
            ((node_27) * ((((node_89) * (node_51)) * (node_53)) * (node_56)))
                * (((current_base_row[146]) - ((BFieldElement::new(2)) * (next_base_row[146])))
                    - ((node_21) * (node_23))),
            (((node_102)
                * ((BFieldElement::new(1)) - ((next_base_row[142]) * (next_base_row[143]))))
                * (current_base_row[142]))
                * ((current_base_row[146]) - (current_base_row[139])),
            ((node_102) * (next_base_row[142])) * ((next_base_row[146]) - (current_base_row[146])),
            (node_114) * ((next_base_row[142]) - (current_base_row[142])),
            ((node_114) * (node_42)) * ((current_base_row[146]) - (node_118)),
            ((node_114) * (node_23))
                * ((current_base_row[146]) - ((node_118) * (current_base_row[142]))),
            ((node_27) * ((node_112) * (node_53)))
                * (((current_base_row[146]) - (next_base_row[146])) - (node_21)),
        ];
        let ext_constraints = [
            (node_27) * (node_130),
            (next_base_row[138])
                * (((node_130)
                    * ((challenges.get_challenge(U32Indeterminate))
                        - (((((challenges.get_challenge(U32CiWeight)) * (next_base_row[141]))
                            + ((challenges.get_challenge(U32LhsWeight))
                                * (next_base_row[142])))
                            + ((challenges.get_challenge(U32RhsWeight))
                                * (next_base_row[144])))
                            + ((challenges.get_challenge(U32ResultWeight))
                                * (next_base_row[146])))))
                    - (next_base_row[147])),
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
        let base_constraints = [
            (base_row[142]) * ((base_row[141]) - (BFieldElement::new(44))),
            base_row[144],
        ];
        let ext_constraints = [];
        base_constraints
            .into_iter()
            .chain(ext_constraints.into_iter())
            .collect()
    }
}

impl Quotientable for ExtU32Table {
    fn num_initial_quotients() -> usize {
        1
    }

    fn num_consistency_quotients() -> usize {
        15
    }

    fn num_transition_quotients() -> usize {
        22
    }

    fn num_terminal_quotients() -> usize {
        2
    }

    #[allow(unused_variables)]
    fn initial_quotient_degree_bounds(interpolant_degree: Degree) -> Vec<Degree> {
        let zerofier_degree = 1 as Degree;
        [interpolant_degree * 3 as Degree - zerofier_degree].to_vec()
    }

    #[allow(unused_variables)]
    fn consistency_quotient_degree_bounds(
        interpolant_degree: Degree,
        padded_height: usize,
    ) -> Vec<Degree> {
        let zerofier_degree = padded_height as Degree;
        [
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 11 as Degree - zerofier_degree,
            interpolant_degree * 11 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 8 as Degree - zerofier_degree,
            interpolant_degree * 9 as Degree - zerofier_degree,
            interpolant_degree * 8 as Degree - zerofier_degree,
            interpolant_degree * 8 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
        ]
        .to_vec()
    }

    #[allow(unused_variables)]
    fn transition_quotient_degree_bounds(
        interpolant_degree: Degree,
        padded_height: usize,
    ) -> Vec<Degree> {
        let zerofier_degree = padded_height as Degree - 1;
        [
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 4 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 4 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 9 as Degree - zerofier_degree,
            interpolant_degree * 9 as Degree - zerofier_degree,
            interpolant_degree * 11 as Degree - zerofier_degree,
            interpolant_degree * 11 as Degree - zerofier_degree,
            interpolant_degree * 12 as Degree - zerofier_degree,
            interpolant_degree * 12 as Degree - zerofier_degree,
            interpolant_degree * 8 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 8 as Degree - zerofier_degree,
            interpolant_degree * 7 as Degree - zerofier_degree,
            interpolant_degree * 9 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 7 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
        ]
        .to_vec()
    }

    #[allow(unused_variables)]
    fn terminal_quotient_degree_bounds(interpolant_degree: Degree) -> Vec<Degree> {
        let zerofier_degree = 1 as Degree;
        [
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
        ]
        .to_vec()
    }
}
