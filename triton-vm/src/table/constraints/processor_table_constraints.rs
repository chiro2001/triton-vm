use ndarray::ArrayView1;
use twenty_first::shared_math::b_field_element::BFieldElement;
use twenty_first::shared_math::mpolynomial::Degree;
use twenty_first::shared_math::x_field_element::XFieldElement;

use crate::table::challenges::ChallengeId::*;
use crate::table::challenges::Challenges;
use crate::table::extension_table::Evaluable;
use crate::table::extension_table::Quotientable;
use crate::table::processor_table::ExtProcessorTable;

// This file has been auto-generated. Any modifications _will_ be lost.
// To re-generate, execute:
// `cargo run --bin constraint-evaluation-generator`
impl Evaluable<BFieldElement> for ExtProcessorTable {
    #[inline]
    #[allow(unused_variables)]
    fn evaluate_initial_constraints(
        base_row: ArrayView1<BFieldElement>,
        ext_row: ArrayView1<XFieldElement>,
        challenges: &Challenges,
    ) -> Vec<XFieldElement> {
        let base_constraints = [
            base_row[4],
            base_row[7],
            base_row[18],
            base_row[19],
            base_row[20],
            base_row[21],
            base_row[22],
            base_row[23],
            base_row[24],
            base_row[25],
            base_row[26],
            base_row[27],
            base_row[28],
            base_row[29],
            base_row[30],
            base_row[31],
            base_row[32],
            base_row[33],
            base_row[34],
            base_row[35],
            base_row[36],
            (base_row[37]) - (BFieldElement::new(16)),
            base_row[38],
            base_row[44],
            base_row[43],
            base_row[6],
        ];
        let ext_constraints = [
            (ext_row[1]) - (BFieldElement::new(1)),
            ((ext_row[3])
                * ((challenges.get_challenge(InstructionLookupIndeterminate))
                    - (((challenges.get_challenge(ProgramInstructionWeight)) * (base_row[8]))
                        + ((challenges.get_challenge(ProgramNextInstructionWeight))
                            * (base_row[9])))))
                - (BFieldElement::new(1)),
            (ext_row[2]) - (BFieldElement::new(1)),
            (ext_row[4])
                - ((challenges.get_challenge(OpStackIndeterminate))
                    - (((challenges.get_challenge(OpStackIb1Weight)) * (base_row[11]))
                        + ((challenges.get_challenge(OpStackOspWeight))
                            * (BFieldElement::new(16))))),
            (ext_row[5]) - (challenges.get_challenge(RamIndeterminate)),
            (ext_row[6])
                - ((challenges.get_challenge(JumpStackIndeterminate))
                    - ((challenges.get_challenge(JumpStackCiWeight)) * (base_row[8]))),
            ext_row[11],
            (((base_row[8]) - (BFieldElement::new(48))) * ((ext_row[7]) - (BFieldElement::new(1))))
                + ((((((((((base_row[10]) - (BFieldElement::new(1)))
                    * ((base_row[11]) - (BFieldElement::new(1))))
                    * ((base_row[12]) - (BFieldElement::new(1))))
                    * ((base_row[13]) - (BFieldElement::new(1))))
                    * (base_row[14]))
                    * (base_row[15]))
                    * ((base_row[16]) - (BFieldElement::new(1))))
                    * ((base_row[17]) - (BFieldElement::new(1))))
                    * ((ext_row[7]) - (challenges.get_challenge(HashInputIndeterminate)))),
            (ext_row[8]) - (BFieldElement::new(1)),
            (ext_row[9]) - (BFieldElement::new(1)),
            ext_row[10],
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
        let base_constraints = [
            (base_row[10]) * ((base_row[10]) - (BFieldElement::new(1))),
            (base_row[11]) * ((base_row[11]) - (BFieldElement::new(1))),
            (base_row[12]) * ((base_row[12]) - (BFieldElement::new(1))),
            (base_row[13]) * ((base_row[13]) - (BFieldElement::new(1))),
            (base_row[14]) * ((base_row[14]) - (BFieldElement::new(1))),
            (base_row[15]) * ((base_row[15]) - (BFieldElement::new(1))),
            (base_row[16]) * ((base_row[16]) - (BFieldElement::new(1))),
            (base_row[17]) * ((base_row[17]) - (BFieldElement::new(1))),
            (base_row[5]) * ((base_row[5]) - (BFieldElement::new(1))),
            (base_row[8])
                - ((((((((base_row[10]) + ((BFieldElement::new(2)) * (base_row[11])))
                    + ((BFieldElement::new(4)) * (base_row[12])))
                    + ((BFieldElement::new(8)) * (base_row[13])))
                    + ((BFieldElement::new(16)) * (base_row[14])))
                    + ((BFieldElement::new(32)) * (base_row[15])))
                    + ((BFieldElement::new(64)) * (base_row[16])))
                    + ((BFieldElement::new(128)) * (base_row[17]))),
            ((base_row[5]) * ((base_row[4]) - (BFieldElement::new(1)))) * (base_row[45]),
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
        let node_746 = (current_base_row[17]) - (BFieldElement::new(1));
        let node_732 = (current_base_row[10]) - (BFieldElement::new(1));
        let node_736 = (current_base_row[12]) - (BFieldElement::new(1));
        let node_744 = (current_base_row[16]) - (BFieldElement::new(1));
        let node_750 = (current_base_row[11]) - (BFieldElement::new(1));
        let node_742 = (current_base_row[15]) - (BFieldElement::new(1));
        let node_758 = (node_732) * (node_750);
        let node_738 = (current_base_row[13]) - (BFieldElement::new(1));
        let node_740 = (current_base_row[14]) - (BFieldElement::new(1));
        let node_759 = (node_758) * (node_736);
        let node_737 = ((node_732) * (current_base_row[11])) * (node_736);
        let node_776 = (node_759) * (node_738);
        let node_840 = (node_758) * (current_base_row[12]);
        let node_761 = (node_759) * (current_base_row[13]);
        let node_752 = ((current_base_row[10]) * (node_750)) * (node_736);
        let node_794 = (node_776) * (node_740);
        let node_739 = (node_737) * (node_738);
        let node_781 = (node_737) * (current_base_row[13]);
        let node_841 = (node_840) * (node_738);
        let node_846 = (node_840) * (current_base_row[13]);
        let node_762 = (node_761) * (node_740);
        let node_777 = (node_776) * (current_base_row[14]);
        let node_753 = (node_752) * (node_738);
        let node_790 = (node_761) * (current_base_row[14]);
        let node_803 = (node_794) * (node_742);
        let node_741 = (node_739) * (node_740);
        let node_766 = (node_752) * (current_base_row[13]);
        let node_3965 = (BFieldElement::new(1)) - (next_base_row[5]);
        let node_775 =
            ((((node_753) * (current_base_row[14])) * (node_742)) * (node_744)) * (node_746);
        let node_799 = (node_739) * (current_base_row[14]);
        let node_782 = (node_781) * (node_740);
        let node_778 = (node_777) * (node_742);
        let node_804 = (node_803) * (node_744);
        let node_842 = (node_841) * (node_740);
        let node_743 = (node_741) * (node_742);
        let node_763 = (node_762) * (node_742);
        let node_796 = (node_794) * (current_base_row[15]);
        let node_809 = (node_781) * (current_base_row[14]);
        let node_847 = (node_846) * (node_740);
        let node_851 = (node_841) * (current_base_row[14]);
        let node_855 = (node_846) * (current_base_row[14]);
        let node_806 = (node_762) * (current_base_row[15]);
        let node_770 = ((((node_766) * (node_740)) * (node_742)) * (node_744)) * (node_746);
        let node_816 = (node_790) * (current_base_row[15]);
        let node_813 = (node_777) * (current_base_row[15]);
        let node_391 = (next_base_row[7]) - (current_base_row[7]);
        let node_791 = (node_790) * (node_742);
        let node_393 = (next_base_row[18]) - (current_base_row[18]);
        let node_394 = (next_base_row[19]) - (current_base_row[19]);
        let node_395 = (next_base_row[20]) - (current_base_row[20]);
        let node_418 = (next_base_row[44]) - (current_base_row[44]);
        let node_419 = (next_base_row[43]) - (current_base_row[43]);
        let node_445 = (BFieldElement::new(1)) - (current_base_row[39]);
        let node_392 = (node_391) - (BFieldElement::new(1));
        let node_440 = (BFieldElement::new(1)) - (current_base_row[42]);
        let node_441 = (BFieldElement::new(1)) - (current_base_row[41]);
        let node_443 = (BFieldElement::new(1)) - (current_base_row[40]);
        let node_821 = ((node_803) * (current_base_row[16])) * (node_746);
        let node_785 = (((node_782) * (node_742)) * (node_744)) * (node_746);
        let node_838 = (((node_799) * (current_base_row[15])) * (node_744)) * (node_746);
        let node_802 = (((node_799) * (node_742)) * (node_744)) * (node_746);
        let node_805 = (node_804) * (node_746);
        let node_823 = ((node_763) * (current_base_row[16])) * (node_746);
        let node_825 = ((node_778) * (current_base_row[16])) * (node_746);
        let node_845 = (((node_842) * (node_742)) * (node_744)) * (node_746);
        let node_747 = ((node_743) * (node_744)) * (node_746);
        let node_757 = ((((node_753) * (node_740)) * (node_742)) * (node_744)) * (node_746);
        let node_780 = ((node_778) * (node_744)) * (node_746);
        let node_789 =
            ((((node_766) * (current_base_row[14])) * (node_742)) * (node_744)) * (node_746);
        let node_798 = ((node_796) * (node_744)) * (node_746);
        let node_812 = (((node_809) * (node_742)) * (node_744)) * (node_746);
        let node_830 = (((node_741) * (current_base_row[15])) * (node_744)) * (node_746);
        let node_833 = (((node_782) * (current_base_row[15])) * (node_744)) * (node_746);
        let node_835 = ((node_796) * (current_base_row[16])) * (node_746);
        let node_850 = (((node_847) * (node_742)) * (node_744)) * (node_746);
        let node_854 = (((node_851) * (node_742)) * (node_744)) * (node_746);
        let node_858 = (((node_855) * (node_742)) * (node_744)) * (node_746);
        let node_861 = (((node_842) * (current_base_row[15])) * (node_744)) * (node_746);
        let node_864 = (((node_847) * (current_base_row[15])) * (node_744)) * (node_746);
        let node_867 = (((node_851) * (current_base_row[15])) * (node_744)) * (node_746);
        let node_870 = (((node_855) * (current_base_row[15])) * (node_744)) * (node_746);
        let node_872 = ((node_806) * (current_base_row[16])) * (node_746);
        let node_874 = ((node_813) * (current_base_row[16])) * (node_746);
        let node_876 = ((node_816) * (current_base_row[16])) * (node_746);
        let node_879 = (((node_809) * (current_base_row[15])) * (node_744)) * (node_746);
        let node_881 = (node_804) * (current_base_row[17]);
        let node_883 = ((node_743) * (current_base_row[16])) * (node_746);
        let node_765 = ((node_763) * (node_744)) * (node_746);
        let node_808 = ((node_806) * (node_744)) * (node_746);
        let node_793 = ((node_791) * (node_744)) * (node_746);
        let node_818 = ((node_816) * (node_744)) * (node_746);
        let node_580 = (next_base_row[32]) - (current_base_row[32]);
        let node_583 = (next_base_row[33]) - (current_base_row[33]);
        let node_586 = (next_base_row[34]) - (current_base_row[34]);
        let node_589 = (next_base_row[35]) - (current_base_row[35]);
        let node_592 = (next_base_row[36]) - (current_base_row[36]);
        let node_594 = (next_base_row[38]) - (current_base_row[38]);
        let node_595 = (next_base_row[37]) - (current_base_row[37]);
        let node_577 = (next_base_row[31]) - (current_base_row[31]);
        let node_556 = (next_base_row[24]) - (current_base_row[24]);
        let node_559 = (next_base_row[25]) - (current_base_row[25]);
        let node_562 = (next_base_row[26]) - (current_base_row[26]);
        let node_565 = (next_base_row[27]) - (current_base_row[27]);
        let node_568 = (next_base_row[28]) - (current_base_row[28]);
        let node_571 = (next_base_row[29]) - (current_base_row[29]);
        let node_574 = (next_base_row[30]) - (current_base_row[30]);
        let node_458 = (node_440) * (current_base_row[41]);
        let node_473 = (current_base_row[42]) * (node_441);
        let node_488 = (current_base_row[42]) * (current_base_row[41]);
        let node_815 = ((node_813) * (node_744)) * (node_746);
        let node_827 = ((node_791) * (current_base_row[16])) * (node_746);
        let node_442 = (node_440) * (node_441);
        let node_553 = (next_base_row[23]) - (current_base_row[23]);
        let node_550 = (next_base_row[22]) - (current_base_row[22]);
        let node_399 = (next_base_row[24]) - (current_base_row[25]);
        let node_400 = (next_base_row[25]) - (current_base_row[26]);
        let node_401 = (next_base_row[26]) - (current_base_row[27]);
        let node_402 = (next_base_row[27]) - (current_base_row[28]);
        let node_403 = (next_base_row[28]) - (current_base_row[29]);
        let node_404 = (next_base_row[29]) - (current_base_row[30]);
        let node_405 = (next_base_row[30]) - (current_base_row[31]);
        let node_406 = (next_base_row[31]) - (current_base_row[32]);
        let node_407 = (next_base_row[32]) - (current_base_row[33]);
        let node_408 = (next_base_row[33]) - (current_base_row[34]);
        let node_409 = (next_base_row[34]) - (current_base_row[35]);
        let node_410 = (next_base_row[35]) - (current_base_row[36]);
        let node_411 = (next_base_row[36]) - (current_base_row[38]);
        let node_413 = (next_base_row[37]) - ((current_base_row[37]) - (BFieldElement::new(1)));
        let node_417 = (((current_base_row[37]) - (BFieldElement::new(16)))
            * (current_base_row[42]))
            - (BFieldElement::new(1));
        let node_397 = (next_base_row[22]) - (current_base_row[23]);
        let node_398 = (next_base_row[23]) - (current_base_row[24]);
        let node_447 = (next_base_row[21]) - (current_base_row[21]);
        let node_4349 = (next_ext_row[10]) - (current_ext_row[10]);
        let node_451 = (node_442) * (current_base_row[40]);
        let node_459 = (node_458) * (node_443);
        let node_466 = (node_458) * (current_base_row[40]);
        let node_474 = (node_473) * (node_443);
        let node_481 = (node_473) * (current_base_row[40]);
        let node_489 = (node_488) * (node_443);
        let node_496 = (node_488) * (current_base_row[40]);
        let node_4327 = (challenges.get_challenge(U32Indeterminate))
            - ((challenges.get_challenge(U32LhsWeight)) * (current_base_row[21]));
        let node_396 = (next_base_row[21]) - (current_base_row[22]);
        let node_444 = (node_442) * (node_443);
        let node_421 = (next_base_row[22]) - (current_base_row[21]);
        let node_422 = (next_base_row[23]) - (current_base_row[22]);
        let node_423 = (next_base_row[24]) - (current_base_row[23]);
        let node_424 = (next_base_row[25]) - (current_base_row[24]);
        let node_425 = (next_base_row[26]) - (current_base_row[25]);
        let node_426 = (next_base_row[27]) - (current_base_row[26]);
        let node_427 = (next_base_row[28]) - (current_base_row[27]);
        let node_428 = (next_base_row[29]) - (current_base_row[28]);
        let node_429 = (next_base_row[30]) - (current_base_row[29]);
        let node_430 = (next_base_row[31]) - (current_base_row[30]);
        let node_431 = (next_base_row[32]) - (current_base_row[31]);
        let node_432 = (next_base_row[33]) - (current_base_row[32]);
        let node_433 = (next_base_row[34]) - (current_base_row[33]);
        let node_434 = (next_base_row[35]) - (current_base_row[34]);
        let node_435 = (next_base_row[36]) - (current_base_row[35]);
        let node_436 = (next_base_row[38]) - (current_base_row[36]);
        let node_438 = (next_base_row[37]) - ((current_base_row[37]) + (BFieldElement::new(1)));
        let node_4324 = (challenges.get_challenge(U32CiWeight)) * (current_base_row[8]);
        let node_4328 = (challenges.get_challenge(U32RhsWeight)) * (current_base_row[22]);
        let node_503 = (current_base_row[39]) - (BFieldElement::new(1));
        let node_4331 = (challenges.get_challenge(U32ResultWeight)) * (next_base_row[21]);
        let node_504 = (current_base_row[39]) * (node_503);
        let node_664 = (current_base_row[22]) - (current_base_row[21]);
        let node_449 = (node_444) * (current_base_row[39]);
        let node_452 = (node_451) * (node_445);
        let node_439 = (node_391) - (BFieldElement::new(2));
        let node_455 = (node_451) * (current_base_row[39]);
        let node_460 = (node_459) * (node_445);
        let node_463 = (node_459) * (current_base_row[39]);
        let node_467 = (node_466) * (node_445);
        let node_470 = (node_466) * (current_base_row[39]);
        let node_475 = (node_474) * (node_445);
        let node_478 = (node_474) * (current_base_row[39]);
        let node_482 = (node_481) * (node_445);
        let node_485 = (node_481) * (current_base_row[39]);
        let node_490 = (node_489) * (node_445);
        let node_493 = (node_489) * (current_base_row[39]);
        let node_497 = (node_496) * (node_445);
        let node_500 = (node_496) * (current_base_row[39]);
        let node_4264 = ((((((((((challenges.get_challenge(HashStateWeight0))
            * (next_base_row[21]))
            + ((challenges.get_challenge(HashStateWeight1)) * (next_base_row[22])))
            + ((challenges.get_challenge(HashStateWeight2)) * (next_base_row[23])))
            + ((challenges.get_challenge(HashStateWeight3)) * (next_base_row[24])))
            + ((challenges.get_challenge(HashStateWeight4)) * (next_base_row[25])))
            + ((challenges.get_challenge(HashStateWeight5)) * (next_base_row[26])))
            + ((challenges.get_challenge(HashStateWeight6)) * (next_base_row[27])))
            + ((challenges.get_challenge(HashStateWeight7)) * (next_base_row[28])))
            + ((challenges.get_challenge(HashStateWeight8)) * (next_base_row[29])))
            + ((challenges.get_challenge(HashStateWeight9)) * (next_base_row[30]));
        let node_4329 = (node_4327) - (node_4328);
        let node_517 = (BFieldElement::new(2)) * (current_base_row[40]);
        let node_665 = (current_base_row[39]) * (node_664);
        let node_4304 = (((next_ext_row[9])
            - ((challenges.get_challenge(SpongeIndeterminate)) * (current_ext_row[9])))
            - ((challenges.get_challenge(HashCIWeight)) * (current_base_row[8])))
            - (node_4264);
        let node_4321 = (challenges.get_challenge(U32Indeterminate))
            - ((challenges.get_challenge(U32LhsWeight)) * (next_base_row[21]));
        let node_4322 = (challenges.get_challenge(U32RhsWeight)) * (next_base_row[22]);
        let node_4354 =
            ((node_4349) * (((node_4329) - (node_4324)) - (node_4331))) - (BFieldElement::new(1));
        let node_446 = (node_444) * (node_445);
        let node_620 = (next_base_row[8]) - (current_base_row[8]);
        let node_658 = (current_base_row[21]) + (current_base_row[22]);
        let node_661 = (next_base_row[21]) - ((current_base_row[21]) * (current_base_row[22]));
        let node_662 = (next_base_row[21]) * (current_base_row[21]);
        let node_666 = (node_665) - (BFieldElement::new(1));
        let node_680 = (current_base_row[22]) * (next_base_row[22]);
        let node_689 = (current_base_row[21]) * (current_base_row[24]);
        let node_690 = (current_base_row[23]) * (current_base_row[25]);
        let node_692 = (current_base_row[22]) * (current_base_row[26]);
        let node_710 = (current_base_row[23]) * (next_base_row[22]);
        let node_712 = (current_base_row[22]) * (next_base_row[23]);
        let node_450 = (node_449) * (node_396);
        let node_698 = (current_base_row[23]) * (current_base_row[26]);
        let node_718 = (current_base_row[23]) * (next_base_row[23]);
        let node_454 = (node_452) * ((next_base_row[21]) - (current_base_row[23]));
        let node_600 = ((current_base_row[21]) * (current_base_row[41])) - (BFieldElement::new(1));
        let node_457 = (node_455) * ((next_base_row[21]) - (current_base_row[24]));
        let node_462 = (node_460) * ((next_base_row[21]) - (current_base_row[25]));
        let node_465 = (node_463) * ((next_base_row[21]) - (current_base_row[26]));
        let node_469 = (node_467) * ((next_base_row[21]) - (current_base_row[27]));
        let node_472 = (node_470) * ((next_base_row[21]) - (current_base_row[28]));
        let node_477 = (node_475) * ((next_base_row[21]) - (current_base_row[29]));
        let node_480 = (node_478) * ((next_base_row[21]) - (current_base_row[30]));
        let node_484 = (node_482) * ((next_base_row[21]) - (current_base_row[31]));
        let node_487 = (node_485) * ((next_base_row[21]) - (current_base_row[32]));
        let node_492 = (node_490) * ((next_base_row[21]) - (current_base_row[33]));
        let node_495 = (node_493) * ((next_base_row[21]) - (current_base_row[34]));
        let node_499 = (node_497) * ((next_base_row[21]) - (current_base_row[35]));
        let node_502 = (node_500) * ((next_base_row[21]) - (current_base_row[36]));
        let node_506 = (current_base_row[40]) * ((current_base_row[40]) - (BFieldElement::new(1)));
        let node_508 = (current_base_row[41]) * ((current_base_row[41]) - (BFieldElement::new(1)));
        let node_510 = (current_base_row[42]) * ((current_base_row[42]) - (BFieldElement::new(1)));
        let node_519 = ((((current_base_row[9])
            - ((BFieldElement::new(8)) * (current_base_row[42])))
            - ((BFieldElement::new(4)) * (current_base_row[41])))
            - (node_517))
            - (current_base_row[39]);
        let node_4141 = (next_ext_row[3]) - (current_ext_row[3]);
        let node_4149 = (next_base_row[10]) - (BFieldElement::new(1));
        let node_4153 = (next_base_row[12]) - (BFieldElement::new(1));
        let node_4155 = (next_base_row[13]) - (BFieldElement::new(1));
        let node_4163 = (next_base_row[17]) - (BFieldElement::new(1));
        let node_4362 =
            ((node_4349) * (((node_4327) - (node_4324)) - (node_4331))) - (BFieldElement::new(1));
        let node_4345 = (((node_4321) - (node_4328))
            - ((challenges.get_challenge(U32CiWeight)) * (BFieldElement::new(12))))
            - (challenges.get_challenge(U32ResultWeight));
        let node_4348 = ((node_4327) - (node_4322))
            - ((challenges.get_challenge(U32CiWeight)) * (BFieldElement::new(4)));

        let base_constraints = [
            ((next_base_row[4]) - (current_base_row[4])) - (BFieldElement::new(1)),
            (current_base_row[5]) * ((next_base_row[5]) - (current_base_row[5])),
            ((next_base_row[6]) - (current_base_row[8])) * (node_3965),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_392))
                + ((node_757)
                    * ((next_base_row[21])
                        - (current_base_row[9]))))
                + ((node_765) * (node_392)))
                + ((node_770) * ((node_446) * (node_447))))
                + ((node_775) * (node_446)))
                + ((node_780) * (node_392)))
                + ((node_785)
                    * ((current_base_row[9])
                        - ((current_base_row[39]) + (node_517)))))
                + ((node_789)
                    * ((next_base_row[18])
                        - ((current_base_row[18])
                            + (BFieldElement::new(1))))))
                + ((node_793)
                    * ((next_base_row[18])
                        - ((current_base_row[18])
                            - (BFieldElement::new(1))))))
                + ((node_798)
                    * ((next_base_row[7]) - (current_base_row[20]))))
                + ((node_802)
                    * ((current_base_row[21])
                        - (BFieldElement::new(1)))))
                + ((node_805) * (node_620)))
                + ((node_808)
                    * ((next_base_row[43]) - (current_base_row[21]))))
                + ((node_812)
                    * ((next_base_row[43]) - (current_base_row[22]))))
                + ((node_815) * (node_392)))
                + ((node_818) * (node_504)))
                + ((node_821)
                    * ((current_base_row[26]) - (current_base_row[21]))))
                + ((node_823) * (node_392)))
                + ((node_825) * (node_392)))
                + ((node_827) * (node_392)))
                + ((node_830) * ((next_base_row[21]) - (node_658))))
                + ((node_833) * (node_661)))
                + ((node_835) * ((node_662) - (BFieldElement::new(1)))))
                + ((node_838) * ((current_base_row[39]) * (node_666))))
                + ((node_845)
                    * ((current_base_row[21])
                        - (((BFieldElement::new(4294967296))
                            * (next_base_row[22]))
                            + (next_base_row[21])))))
                + ((node_850) * (node_392)))
                + ((node_854) * (node_392)))
                + ((node_858) * (node_392)))
                + ((node_861) * (node_392)))
                + ((node_864) * (node_392)))
                + ((node_867)
                    * (((current_base_row[21]) - (node_680)) - (next_base_row[21]))))
                + ((node_870) * (node_392)))
                + ((node_872)
                    * ((next_base_row[21])
                        - ((current_base_row[21]) + (current_base_row[24])))))
                + ((node_874)
                    * ((next_base_row[21]) - (((node_689) - (node_690)) - (node_692)))))
                + ((node_876)
                    * ((((node_662) - (node_710)) - (node_712)) - (BFieldElement::new(1)))))
                + ((node_879) * (node_661)))
                + ((node_881) * (node_392)))
                + ((node_883) * (node_392)))
                * (node_3965))
                + ((node_391) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_393))
                + ((node_757) * (node_421)))
                + ((node_765) * (node_393)))
                + ((node_770) * (node_450)))
                + ((node_775) * ((node_449) * (node_421))))
                + ((node_780) * (node_393)))
                + ((node_785) * (node_504)))
                + ((node_789)
                    * (((next_base_row[19])
                        - (current_base_row[7]))
                        - (BFieldElement::new(2)))))
                + ((node_793)
                    * ((next_base_row[7]) - (current_base_row[19]))))
                + ((node_798) * (node_393)))
                + ((node_802) * (node_392)))
                + ((node_805) * (node_392)))
                + ((node_808)
                    * ((next_base_row[21]) - (next_base_row[44]))))
                + ((node_812)
                    * ((next_base_row[44]) - (current_base_row[21]))))
                + ((node_815) * (node_393)))
                + ((node_818)
                    * ((((next_base_row[31]) * (BFieldElement::new(2)))
                        + (current_base_row[39]))
                        - (current_base_row[31]))))
                + ((node_821)
                    * ((current_base_row[27]) - (current_base_row[22]))))
                + ((node_823) * (node_393)))
                + ((node_825) * (node_393)))
                + ((node_827) * (node_393)))
                + ((node_830) * (node_392)))
                + ((node_833) * (node_392)))
                + ((node_835) * (node_392)))
                + ((node_838) * ((node_664) * (node_666))))
                + ((node_845)
                    * ((next_base_row[21])
                        * (((current_base_row[39])
                            * ((next_base_row[22])
                                - (BFieldElement::new(4294967295))))
                            - (BFieldElement::new(1))))))
                + ((node_850) * (node_393)))
                + ((node_854) * (node_393)))
                + ((node_858) * (node_393)))
                + ((node_861) * (node_393)))
                + ((node_864) * (node_393)))
                + ((node_867) * (node_553)))
                + ((node_870) * (node_393)))
                + ((node_872)
                    * ((next_base_row[22])
                        - ((current_base_row[22]) + (current_base_row[25])))))
                + ((node_874)
                    * ((next_base_row[22])
                        - ((((((current_base_row[22]) * (current_base_row[24]))
                            + ((current_base_row[21]) * (current_base_row[25])))
                            - (node_698))
                            + (node_690))
                            + (node_692)))))
                + ((node_876)
                    * ((((((current_base_row[22]) * (next_base_row[21]))
                        + ((current_base_row[21]) * (next_base_row[22])))
                        - (node_718))
                        + (node_710))
                        + (node_712))))
                + ((node_879)
                    * ((next_base_row[22])
                        - ((current_base_row[21]) * (current_base_row[23])))))
                + ((node_881) * (node_393)))
                + ((node_883) * (node_393)))
                * (node_3965))
                + ((node_620) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_394))
                + ((node_757) * (node_422)))
                + ((node_765) * (node_394)))
                + ((node_770) * (node_454)))
                + ((node_775)
                    * ((node_452)
                        * ((next_base_row[23])
                            - (current_base_row[21])))))
                + ((node_780) * (node_394)))
                + ((node_785)
                    * ((((node_392) * (current_base_row[21]))
                        + (((node_439) * (node_600))
                            * (node_503)))
                        + ((((node_391)
                            - (BFieldElement::new(3)))
                            * (node_600))
                            * (current_base_row[39])))))
                + ((node_789)
                    * ((next_base_row[20])
                        - (current_base_row[9]))))
                + ((node_793) * (node_447)))
                + ((node_798) * (node_394)))
                + ((node_802) * (node_393)))
                + ((node_805) * (node_393)))
                + ((node_808) * (node_392)))
                + ((node_812) * (node_392)))
                + ((node_815) * (node_394)))
                + ((node_818)
                    * (((node_445)
                        * ((current_base_row[26]) - (next_base_row[21])))
                        + ((current_base_row[39])
                            * ((current_base_row[26])
                                - (next_base_row[26]))))))
                + ((node_821)
                    * ((current_base_row[28]) - (current_base_row[23]))))
                + ((node_823) * (node_394)))
                + ((node_825) * (node_394)))
                + ((node_827) * (node_394)))
                + ((node_830) * (node_393)))
                + ((node_833) * (node_393)))
                + ((node_835) * (node_393)))
                + ((node_838)
                    * ((next_base_row[21])
                        - ((BFieldElement::new(1)) - (node_665)))))
                + ((node_845) * (node_422)))
                + ((node_850) * (node_394)))
                + ((node_854) * (node_394)))
                + ((node_858) * (node_394)))
                + ((node_861) * (node_394)))
                + ((node_864) * (node_394)))
                + ((node_867) * (node_392)))
                + ((node_870) * (node_394)))
                + ((node_872)
                    * ((next_base_row[23])
                        - ((current_base_row[23]) + (current_base_row[26])))))
                + ((node_874)
                    * ((next_base_row[23])
                        - (((((current_base_row[23]) * (current_base_row[24]))
                            + ((current_base_row[22]) * (current_base_row[25])))
                            + ((current_base_row[21]) * (current_base_row[26])))
                            + (node_698)))))
                + ((node_876)
                    * (((((current_base_row[23]) * (next_base_row[21])) + (node_680))
                        + ((current_base_row[21]) * (next_base_row[23])))
                        + (node_718))))
                + ((node_879) * ((next_base_row[23]) - (node_689))))
                + ((node_881) * (node_394)))
                + ((node_883) * (node_394)))
                * (node_3965))
                + (((next_base_row[9]) - (current_base_row[9])) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_395))
                + ((node_757) * (node_423)))
                + ((node_765) * (node_395)))
                + ((node_770) * (node_457)))
                + ((node_775)
                    * ((node_455)
                        * ((next_base_row[24])
                            - (current_base_row[21])))))
                + ((node_780) * (node_395)))
                + ((node_785) * (node_393)))
                + ((node_789)
                    * ((next_base_row[7]) - (current_base_row[9]))))
                + ((node_793) * (node_550)))
                + ((node_798) * (node_395)))
                + ((node_802) * (node_394)))
                + ((node_805) * (node_394)))
                + ((node_808) * (node_393)))
                + ((node_812) * (node_393)))
                + ((node_815) * (node_395)))
                + ((node_818)
                    * (((node_445)
                        * ((current_base_row[27]) - (next_base_row[22])))
                        + ((current_base_row[39])
                            * ((current_base_row[27])
                                - (next_base_row[27]))))))
                + ((node_821)
                    * ((current_base_row[29]) - (current_base_row[24]))))
                + ((node_823) * (node_395)))
                + ((node_825) * (node_395)))
                + ((node_827) * (node_395)))
                + ((node_830) * (node_394)))
                + ((node_833) * (node_394)))
                + ((node_835) * (node_394)))
                + ((node_838) * (node_392)))
                + ((node_845) * (node_423)))
                + ((node_850) * (node_395)))
                + ((node_854) * (node_395)))
                + ((node_858) * (node_395)))
                + ((node_861) * (node_395)))
                + ((node_864) * (node_395)))
                + ((node_867) * (node_393)))
                + ((node_870) * (node_395)))
                + ((node_872) * (node_556)))
                + ((node_874) * (node_556)))
                + ((node_876) * (node_556)))
                + ((node_879) * (node_399)))
                + ((node_881) * (node_395)))
                + ((node_883) * (node_395)))
                * (node_3965))
                + ((node_393) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_396))
                + ((node_757) * (node_424)))
                + ((node_765) * (node_421)))
                + ((node_770) * (node_462)))
                + ((node_775)
                    * ((node_460)
                        * ((next_base_row[25])
                            - (current_base_row[21])))))
                + ((node_780) * (node_447)))
                + ((node_785) * (node_394)))
                + ((node_789) * (node_447)))
                + ((node_793) * (node_553)))
                + ((node_798) * (node_447)))
                + ((node_802) * (node_395)))
                + ((node_805) * (node_395)))
                + ((node_808) * (node_394)))
                + ((node_812) * (node_394)))
                + ((node_815) * (node_577)))
                + ((node_818)
                    * (((node_445)
                        * ((current_base_row[28]) - (next_base_row[23])))
                        + ((current_base_row[39])
                            * ((current_base_row[28])
                                - (next_base_row[28]))))))
                + ((node_821)
                    * ((current_base_row[30]) - (current_base_row[25]))))
                + ((node_823) * (node_447)))
                + ((node_825) * (node_447)))
                + ((node_827) * (node_577)))
                + ((node_830) * (node_395)))
                + ((node_833) * (node_395)))
                + ((node_835) * (node_395)))
                + ((node_838) * (node_393)))
                + ((node_845) * (node_424)))
                + ((node_850) * (node_397)))
                + ((node_854) * (node_397)))
                + ((node_858) * (node_397)))
                + ((node_861) * (node_550)))
                + ((node_864) * (node_397)))
                + ((node_867) * (node_394)))
                + ((node_870) * (node_550)))
                + ((node_872) * (node_559)))
                + ((node_874) * (node_559)))
                + ((node_876) * (node_559)))
                + ((node_879) * (node_400)))
                + ((node_881) * (node_421)))
                + ((node_883) * (node_396)))
                * (node_3965))
                + ((node_394) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_397))
                + ((node_757) * (node_425)))
                + ((node_765) * (node_422)))
                + ((node_770) * (node_465)))
                + ((node_775)
                    * ((node_463)
                        * ((next_base_row[26])
                            - (current_base_row[21])))))
                + ((node_780) * (node_550)))
                + ((node_785) * (node_395)))
                + ((node_789) * (node_550)))
                + ((node_793) * (node_556)))
                + ((node_798) * (node_550)))
                + ((node_802) * (node_396)))
                + ((node_805) * (node_447)))
                + ((node_808) * (node_395)))
                + ((node_812) * (node_395)))
                + ((node_815) * (node_580)))
                + ((node_818)
                    * (((node_445)
                        * ((current_base_row[29]) - (next_base_row[24])))
                        + ((current_base_row[39])
                            * ((current_base_row[29])
                                - (next_base_row[29]))))))
                + ((node_821) * (node_392)))
                + ((node_823) * (node_550)))
                + ((node_825) * (node_550)))
                + ((node_827) * (node_580)))
                + ((node_830) * (node_397)))
                + ((node_833) * (node_397)))
                + ((node_835) * (node_550)))
                + ((node_838) * (node_394)))
                + ((node_845) * (node_425)))
                + ((node_850) * (node_398)))
                + ((node_854) * (node_398)))
                + ((node_858) * (node_398)))
                + ((node_861) * (node_553)))
                + ((node_864) * (node_398)))
                + ((node_867) * (node_395)))
                + ((node_870) * (node_553)))
                + ((node_872) * (node_562)))
                + ((node_874) * (node_562)))
                + ((node_876) * (node_562)))
                + ((node_879) * (node_401)))
                + ((node_881) * (node_422)))
                + ((node_883) * (node_397)))
                * (node_3965))
                + ((node_395) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_398))
                + ((node_757) * (node_426)))
                + ((node_765) * (node_423)))
                + ((node_770) * (node_469)))
                + ((node_775)
                    * ((node_467)
                        * ((next_base_row[27])
                            - (current_base_row[21])))))
                + ((node_780) * (node_553)))
                + ((node_785) * (node_396)))
                + ((node_789) * (node_553)))
                + ((node_793) * (node_559)))
                + ((node_798) * (node_553)))
                + ((node_802) * (node_397)))
                + ((node_805) * (node_550)))
                + ((node_808) * (node_421)))
                + ((node_812) * (node_396)))
                + ((node_815) * (node_583)))
                + ((node_818)
                    * (((node_445)
                        * ((current_base_row[30]) - (next_base_row[25])))
                        + ((current_base_row[39])
                            * ((current_base_row[30])
                                - (next_base_row[30]))))))
                + ((node_821) * (node_393)))
                + ((node_823) * (node_553)))
                + ((node_825) * (node_553)))
                + ((node_827) * (node_583)))
                + ((node_830) * (node_398)))
                + ((node_833) * (node_398)))
                + ((node_835) * (node_553)))
                + ((node_838) * (node_395)))
                + ((node_845) * (node_426)))
                + ((node_850) * (node_399)))
                + ((node_854) * (node_399)))
                + ((node_858) * (node_399)))
                + ((node_861) * (node_556)))
                + ((node_864) * (node_399)))
                + ((node_867) * (node_556)))
                + ((node_870) * (node_556)))
                + ((node_872) * (node_565)))
                + ((node_874) * (node_565)))
                + ((node_876) * (node_565)))
                + ((node_879) * (node_402)))
                + ((node_881) * (node_423)))
                + ((node_883) * (node_398)))
                * (node_3965))
                + ((node_447) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_399))
                + ((node_757) * (node_427)))
                + ((node_765) * (node_424)))
                + ((node_770) * (node_472)))
                + ((node_775)
                    * ((node_470)
                        * ((next_base_row[28])
                            - (current_base_row[21])))))
                + ((node_780) * (node_556)))
                + ((node_785) * (node_397)))
                + ((node_789) * (node_556)))
                + ((node_793) * (node_562)))
                + ((node_798) * (node_556)))
                + ((node_802) * (node_398)))
                + ((node_805) * (node_553)))
                + ((node_808) * (node_422)))
                + ((node_812) * (node_397)))
                + ((node_815) * (node_586)))
                + ((node_818) * (node_580)))
                + ((node_821) * (node_394)))
                + ((node_823) * (node_556)))
                + ((node_825) * (node_556)))
                + ((node_827) * (node_586)))
                + ((node_830) * (node_399)))
                + ((node_833) * (node_399)))
                + ((node_835) * (node_556)))
                + ((node_838) * (node_397)))
                + ((node_845) * (node_427)))
                + ((node_850) * (node_400)))
                + ((node_854) * (node_400)))
                + ((node_858) * (node_400)))
                + ((node_861) * (node_559)))
                + ((node_864) * (node_400)))
                + ((node_867) * (node_559)))
                + ((node_870) * (node_559)))
                + ((node_872) * (node_568)))
                + ((node_874) * (node_568)))
                + ((node_876) * (node_568)))
                + ((node_879) * (node_403)))
                + ((node_881) * (node_424)))
                + ((node_883) * (node_399)))
                * (node_3965))
                + ((node_550) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_400))
                + ((node_757) * (node_428)))
                + ((node_765) * (node_425)))
                + ((node_770) * (node_477)))
                + ((node_775)
                    * ((node_475)
                        * ((next_base_row[29])
                            - (current_base_row[21])))))
                + ((node_780) * (node_559)))
                + ((node_785) * (node_398)))
                + ((node_789) * (node_559)))
                + ((node_793) * (node_565)))
                + ((node_798) * (node_559)))
                + ((node_802) * (node_399)))
                + ((node_805) * (node_556)))
                + ((node_808) * (node_423)))
                + ((node_812) * (node_398)))
                + ((node_815) * (node_589)))
                + ((node_818) * (node_583)))
                + ((node_821) * (node_395)))
                + ((node_823) * (node_559)))
                + ((node_825) * (node_559)))
                + ((node_827) * (node_589)))
                + ((node_830) * (node_400)))
                + ((node_833) * (node_400)))
                + ((node_835) * (node_559)))
                + ((node_838) * (node_398)))
                + ((node_845) * (node_428)))
                + ((node_850) * (node_401)))
                + ((node_854) * (node_401)))
                + ((node_858) * (node_401)))
                + ((node_861) * (node_562)))
                + ((node_864) * (node_401)))
                + ((node_867) * (node_562)))
                + ((node_870) * (node_562)))
                + ((node_872) * (node_571)))
                + ((node_874) * (node_571)))
                + ((node_876) * (node_571)))
                + ((node_879) * (node_404)))
                + ((node_881) * (node_425)))
                + ((node_883) * (node_400)))
                * (node_3965))
                + ((node_553) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_401))
                + ((node_757) * (node_429)))
                + ((node_765) * (node_426)))
                + ((node_770) * (node_480)))
                + ((node_775)
                    * ((node_478)
                        * ((next_base_row[30])
                            - (current_base_row[21])))))
                + ((node_780) * (node_562)))
                + ((node_785) * (node_399)))
                + ((node_789) * (node_562)))
                + ((node_793) * (node_568)))
                + ((node_798) * (node_562)))
                + ((node_802) * (node_400)))
                + ((node_805) * (node_559)))
                + ((node_808) * (node_424)))
                + ((node_812) * (node_399)))
                + ((node_815) * (node_592)))
                + ((node_818) * (node_586)))
                + ((node_821) * (node_447)))
                + ((node_823) * (node_562)))
                + ((node_825) * (node_562)))
                + ((node_827) * (node_592)))
                + ((node_830) * (node_401)))
                + ((node_833) * (node_401)))
                + ((node_835) * (node_562)))
                + ((node_838) * (node_399)))
                + ((node_845) * (node_429)))
                + ((node_850) * (node_402)))
                + ((node_854) * (node_402)))
                + ((node_858) * (node_402)))
                + ((node_861) * (node_565)))
                + ((node_864) * (node_402)))
                + ((node_867) * (node_565)))
                + ((node_870) * (node_565)))
                + ((node_872) * (node_574)))
                + ((node_874) * (node_574)))
                + ((node_876) * (node_574)))
                + ((node_879) * (node_405)))
                + ((node_881) * (node_426)))
                + ((node_883) * (node_401)))
                * (node_3965))
                + ((node_556) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_402))
                + ((node_757) * (node_430)))
                + ((node_765) * (node_427)))
                + ((node_770) * (node_484)))
                + ((node_775)
                    * ((node_482)
                        * ((next_base_row[31])
                            - (current_base_row[21])))))
                + ((node_780) * (node_565)))
                + ((node_785) * (node_400)))
                + ((node_789) * (node_565)))
                + ((node_793) * (node_571)))
                + ((node_798) * (node_565)))
                + ((node_802) * (node_401)))
                + ((node_805) * (node_562)))
                + ((node_808) * (node_425)))
                + ((node_812) * (node_400)))
                + ((node_815) * (node_594)))
                + ((node_818) * (node_589)))
                + ((node_821) * (node_550)))
                + ((node_823) * (node_565)))
                + ((node_825) * (node_565)))
                + ((node_827) * (node_594)))
                + ((node_830) * (node_402)))
                + ((node_833) * (node_402)))
                + ((node_835) * (node_565)))
                + ((node_838) * (node_400)))
                + ((node_845) * (node_430)))
                + ((node_850) * (node_403)))
                + ((node_854) * (node_403)))
                + ((node_858) * (node_403)))
                + ((node_861) * (node_568)))
                + ((node_864) * (node_403)))
                + ((node_867) * (node_568)))
                + ((node_870) * (node_568)))
                + ((node_872) * (node_577)))
                + ((node_874) * (node_577)))
                + ((node_876) * (node_577)))
                + ((node_879) * (node_406)))
                + ((node_881) * (node_427)))
                + ((node_883) * (node_402)))
                * (node_3965))
                + ((node_559) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_403))
                + ((node_757) * (node_431)))
                + ((node_765) * (node_428)))
                + ((node_770) * (node_487)))
                + ((node_775)
                    * ((node_485)
                        * ((next_base_row[32])
                            - (current_base_row[21])))))
                + ((node_780) * (node_568)))
                + ((node_785) * (node_401)))
                + ((node_789) * (node_568)))
                + ((node_793) * (node_574)))
                + ((node_798) * (node_568)))
                + ((node_802) * (node_402)))
                + ((node_805) * (node_565)))
                + ((node_808) * (node_426)))
                + ((node_812) * (node_401)))
                + ((node_815) * (node_595)))
                + ((node_818) * (node_592)))
                + ((node_821) * (node_553)))
                + ((node_823) * (node_568)))
                + ((node_825) * (node_568)))
                + ((node_827) * (node_595)))
                + ((node_830) * (node_403)))
                + ((node_833) * (node_403)))
                + ((node_835) * (node_568)))
                + ((node_838) * (node_401)))
                + ((node_845) * (node_431)))
                + ((node_850) * (node_404)))
                + ((node_854) * (node_404)))
                + ((node_858) * (node_404)))
                + ((node_861) * (node_571)))
                + ((node_864) * (node_404)))
                + ((node_867) * (node_571)))
                + ((node_870) * (node_571)))
                + ((node_872) * (node_580)))
                + ((node_874) * (node_580)))
                + ((node_876) * (node_580)))
                + ((node_879) * (node_407)))
                + ((node_881) * (node_428)))
                + ((node_883) * (node_403)))
                * (node_3965))
                + ((node_562) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_404))
                + ((node_757) * (node_432)))
                + ((node_765) * (node_429)))
                + ((node_770) * (node_492)))
                + ((node_775)
                    * ((node_490)
                        * ((next_base_row[33])
                            - (current_base_row[21])))))
                + ((node_780) * (node_571)))
                + ((node_785) * (node_402)))
                + ((node_789) * (node_571)))
                + ((node_793) * (node_577)))
                + ((node_798) * (node_571)))
                + ((node_802) * (node_403)))
                + ((node_805) * (node_568)))
                + ((node_808) * (node_427)))
                + ((node_812) * (node_402)))
                + ((node_815) * (node_418)))
                + ((node_818) * (node_594)))
                + ((node_821) * (node_556)))
                + ((node_823) * (node_571)))
                + ((node_825) * (node_571)))
                + ((node_827) * (node_418)))
                + ((node_830) * (node_404)))
                + ((node_833) * (node_404)))
                + ((node_835) * (node_571)))
                + ((node_838) * (node_402)))
                + ((node_845) * (node_432)))
                + ((node_850) * (node_405)))
                + ((node_854) * (node_405)))
                + ((node_858) * (node_405)))
                + ((node_861) * (node_574)))
                + ((node_864) * (node_405)))
                + ((node_867) * (node_574)))
                + ((node_870) * (node_574)))
                + ((node_872) * (node_583)))
                + ((node_874) * (node_583)))
                + ((node_876) * (node_583)))
                + ((node_879) * (node_408)))
                + ((node_881) * (node_429)))
                + ((node_883) * (node_404)))
                * (node_3965))
                + ((node_565) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_405))
                + ((node_757) * (node_433)))
                + ((node_765) * (node_430)))
                + ((node_770) * (node_495)))
                + ((node_775)
                    * ((node_493)
                        * ((next_base_row[34])
                            - (current_base_row[21])))))
                + ((node_780) * (node_574)))
                + ((node_785) * (node_403)))
                + ((node_789) * (node_574)))
                + ((node_793) * (node_580)))
                + ((node_798) * (node_574)))
                + ((node_802) * (node_404)))
                + ((node_805) * (node_571)))
                + ((node_808) * (node_428)))
                + ((node_812) * (node_403)))
                + ((node_815) * (node_419)))
                + ((node_818) * (node_595)))
                + ((node_821) * (node_559)))
                + ((node_823) * (node_574)))
                + ((node_825) * (node_574)))
                + ((node_827) * (node_419)))
                + ((node_830) * (node_405)))
                + ((node_833) * (node_405)))
                + ((node_835) * (node_574)))
                + ((node_838) * (node_403)))
                + ((node_845) * (node_433)))
                + ((node_850) * (node_406)))
                + ((node_854) * (node_406)))
                + ((node_858) * (node_406)))
                + ((node_861) * (node_577)))
                + ((node_864) * (node_406)))
                + ((node_867) * (node_577)))
                + ((node_870) * (node_577)))
                + ((node_872) * (node_586)))
                + ((node_874) * (node_586)))
                + ((node_876) * (node_586)))
                + ((node_879) * (node_409)))
                + ((node_881) * (node_430)))
                + ((node_883) * (node_405)))
                * (node_3965))
                + ((node_568) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((node_747)
                * (node_406))
                + ((node_757) * (node_434)))
                + ((node_765) * (node_431)))
                + ((node_770) * (node_499)))
                + ((node_775)
                    * ((node_497)
                        * ((next_base_row[35])
                            - (current_base_row[21])))))
                + ((node_780) * (node_577)))
                + ((node_785) * (node_404)))
                + ((node_789) * (node_577)))
                + ((node_793) * (node_583)))
                + ((node_798) * (node_577)))
                + ((node_802) * (node_405)))
                + ((node_805) * (node_574)))
                + ((node_808) * (node_429)))
                + ((node_812) * (node_404)))
                + ((node_818) * (node_392)))
                + ((node_821) * (node_562)))
                + ((node_823) * (node_577)))
                + ((node_825) * (node_577)))
                + ((node_830) * (node_406)))
                + ((node_833) * (node_406)))
                + ((node_835) * (node_577)))
                + ((node_838) * (node_404)))
                + ((node_845) * (node_434)))
                + ((node_850) * (node_407)))
                + ((node_854) * (node_407)))
                + ((node_858) * (node_407)))
                + ((node_861) * (node_580)))
                + ((node_864) * (node_407)))
                + ((node_867) * (node_580)))
                + ((node_870) * (node_580)))
                + ((node_872) * (node_589)))
                + ((node_874) * (node_589)))
                + ((node_876) * (node_589)))
                + ((node_879) * (node_410)))
                + ((node_881) * (node_431)))
                + ((node_883) * (node_406)))
                * (node_3965))
                + ((node_571) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((node_747)
                * (node_407))
                + ((node_757) * (node_435)))
                + ((node_765) * (node_432)))
                + ((node_770) * (node_502)))
                + ((node_775)
                    * ((node_500)
                        * ((next_base_row[36])
                            - (current_base_row[21])))))
                + ((node_780) * (node_580)))
                + ((node_785) * (node_405)))
                + ((node_789) * (node_580)))
                + ((node_793) * (node_586)))
                + ((node_798) * (node_580)))
                + ((node_802) * (node_406)))
                + ((node_805) * (node_577)))
                + ((node_808) * (node_430)))
                + ((node_812) * (node_405)))
                + ((node_818) * (node_393)))
                + ((node_821) * (node_565)))
                + ((node_823) * (node_580)))
                + ((node_825) * (node_580)))
                + ((node_830) * (node_407)))
                + ((node_833) * (node_407)))
                + ((node_835) * (node_580)))
                + ((node_838) * (node_405)))
                + ((node_845) * (node_435)))
                + ((node_850) * (node_408)))
                + ((node_854) * (node_408)))
                + ((node_858) * (node_408)))
                + ((node_861) * (node_583)))
                + ((node_864) * (node_408)))
                + ((node_867) * (node_583)))
                + ((node_870) * (node_583)))
                + ((node_872) * (node_592)))
                + ((node_874) * (node_592)))
                + ((node_876) * (node_592)))
                + ((node_879) * (node_411)))
                + ((node_881) * (node_432)))
                + ((node_883) * (node_407)))
                * (node_3965))
                + ((node_574) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((node_747)
                * (node_408))
                + ((node_757) * (node_436)))
                + ((node_765) * (node_433)))
                + ((node_770) * (node_504)))
                + ((node_775) * (node_450)))
                + ((node_780) * (node_583)))
                + ((node_785) * (node_406)))
                + ((node_789) * (node_583)))
                + ((node_793) * (node_589)))
                + ((node_798) * (node_583)))
                + ((node_802) * (node_407)))
                + ((node_805) * (node_580)))
                + ((node_808) * (node_431)))
                + ((node_812) * (node_406)))
                + ((node_818) * (node_394)))
                + ((node_821) * (node_568)))
                + ((node_823) * (node_583)))
                + ((node_825) * (node_583)))
                + ((node_830) * (node_408)))
                + ((node_833) * (node_408)))
                + ((node_835) * (node_583)))
                + ((node_838) * (node_406)))
                + ((node_845) * (node_436)))
                + ((node_850) * (node_409)))
                + ((node_854) * (node_409)))
                + ((node_858) * (node_409)))
                + ((node_861) * (node_586)))
                + ((node_864) * (node_409)))
                + ((node_867) * (node_586)))
                + ((node_870) * (node_586)))
                + ((node_872) * (node_594)))
                + ((node_874) * (node_594)))
                + ((node_876) * (node_594)))
                + ((node_879) * (node_413)))
                + ((node_881) * (node_433)))
                + ((node_883) * (node_408)))
                * (node_3965))
                + ((node_577) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((node_747)
                * (node_409))
                + ((node_757) * (node_438)))
                + ((node_765) * (node_434)))
                + ((node_770) * (node_506)))
                + ((node_775) * (node_454)))
                + ((node_780) * (node_586)))
                + ((node_785) * (node_407)))
                + ((node_789) * (node_586)))
                + ((node_793) * (node_592)))
                + ((node_798) * (node_586)))
                + ((node_802) * (node_408)))
                + ((node_805) * (node_583)))
                + ((node_808) * (node_432)))
                + ((node_812) * (node_407)))
                + ((node_818) * (node_395)))
                + ((node_821) * (node_571)))
                + ((node_823) * (node_586)))
                + ((node_825) * (node_586)))
                + ((node_830) * (node_409)))
                + ((node_833) * (node_409)))
                + ((node_835) * (node_586)))
                + ((node_838) * (node_407)))
                + ((node_845) * (node_438)))
                + ((node_850) * (node_410)))
                + ((node_854) * (node_410)))
                + ((node_858) * (node_410)))
                + ((node_861) * (node_589)))
                + ((node_864) * (node_410)))
                + ((node_867) * (node_589)))
                + ((node_870) * (node_589)))
                + ((node_872) * (node_595)))
                + ((node_874) * (node_595)))
                + ((node_876) * (node_595)))
                + ((node_879) * (node_417)))
                + ((node_881) * (node_434)))
                + ((node_883) * (node_409)))
                * (node_3965))
                + ((node_580) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((node_747)
                * (node_410))
                + ((node_757) * (node_439)))
                + ((node_765) * (node_435)))
                + ((node_770) * (node_508)))
                + ((node_775) * (node_457)))
                + ((node_780) * (node_589)))
                + ((node_785) * (node_408)))
                + ((node_789) * (node_589)))
                + ((node_793) * (node_594)))
                + ((node_798) * (node_589)))
                + ((node_802) * (node_409)))
                + ((node_805) * (node_586)))
                + ((node_808) * (node_433)))
                + ((node_812) * (node_408)))
                + ((node_818) * (node_418)))
                + ((node_821) * (node_574)))
                + ((node_823) * (node_589)))
                + ((node_825) * (node_589)))
                + ((node_830) * (node_410)))
                + ((node_833) * (node_410)))
                + ((node_835) * (node_589)))
                + ((node_838) * (node_408)))
                + ((node_845) * (node_392)))
                + ((node_850) * (node_411)))
                + ((node_854) * (node_411)))
                + ((node_858) * (node_411)))
                + ((node_861) * (node_592)))
                + ((node_864) * (node_411)))
                + ((node_867) * (node_592)))
                + ((node_870) * (node_592)))
                + ((node_872) * (node_392)))
                + ((node_874) * (node_392)))
                + ((node_876) * (node_392)))
                + ((node_879) * (node_392)))
                + ((node_881) * (node_435)))
                + ((node_883) * (node_410)))
                * (node_3965))
                + ((node_583) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((node_747)
                * (node_411))
                + ((node_757) * (node_393)))
                + ((node_765) * (node_436)))
                + ((node_770) * (node_510)))
                + ((node_775) * (node_462)))
                + ((node_780) * (node_592)))
                + ((node_785) * (node_409)))
                + ((node_789) * (node_592)))
                + ((node_793) * (node_595)))
                + ((node_798) * (node_592)))
                + ((node_802) * (node_410)))
                + ((node_805) * (node_589)))
                + ((node_808) * (node_434)))
                + ((node_812) * (node_409)))
                + ((node_818) * (node_419)))
                + ((node_821) * (node_577)))
                + ((node_823) * (node_592)))
                + ((node_825) * (node_592)))
                + ((node_830) * (node_411)))
                + ((node_833) * (node_411)))
                + ((node_835) * (node_592)))
                + ((node_838) * (node_409)))
                + ((node_845) * (node_393)))
                + ((node_850) * (node_413)))
                + ((node_854) * (node_413)))
                + ((node_858) * (node_413)))
                + ((node_861) * (node_594)))
                + ((node_864) * (node_413)))
                + ((node_867) * (node_594)))
                + ((node_870) * (node_594)))
                + ((node_872) * (node_393)))
                + ((node_874) * (node_393)))
                + ((node_876) * (node_393)))
                + ((node_879) * (node_393)))
                + ((node_881) * (node_436)))
                + ((node_883) * (node_411)))
                * (node_3965))
                + ((node_586) * (next_base_row[5])),
            (((((((((((((((((((((((((((((((((((((node_747)
                * (node_413))
                + ((node_757) * (node_394)))
                + ((node_765) * (node_438)))
                + ((node_770) * (node_519)))
                + ((node_775) * (node_465)))
                + ((node_780) * (node_594)))
                + ((node_785) * (node_410)))
                + ((node_789) * (node_594)))
                + ((node_793) * (node_418)))
                + ((node_798) * (node_594)))
                + ((node_802) * (node_411)))
                + ((node_805) * (node_592)))
                + ((node_808) * (node_435)))
                + ((node_812) * (node_410)))
                + ((node_821) * (node_580)))
                + ((node_823) * (node_594)))
                + ((node_825) * (node_594)))
                + ((node_830) * (node_413)))
                + ((node_833) * (node_413)))
                + ((node_835) * (node_594)))
                + ((node_838) * (node_410)))
                + ((node_845) * (node_394)))
                + ((node_850) * (node_417)))
                + ((node_854) * (node_417)))
                + ((node_858) * (node_417)))
                + ((node_861) * (node_595)))
                + ((node_864) * (node_417)))
                + ((node_867) * (node_595)))
                + ((node_870) * (node_595)))
                + ((node_872) * (node_394)))
                + ((node_874) * (node_394)))
                + ((node_876) * (node_394)))
                + ((node_879) * (node_394)))
                + ((node_881) * (node_438)))
                + ((node_883) * (node_413)))
                * (node_3965))
                + ((node_589) * (next_base_row[5])),
            (((((((((((((((((((((((((((((((((((((node_747)
                * (node_417))
                + ((node_757) * (node_395)))
                + ((node_765) * (node_418)))
                + ((node_770) * (node_439)))
                + ((node_775) * (node_469)))
                + ((node_780) * (node_595)))
                + ((node_785) * (node_411)))
                + ((node_789) * (node_595)))
                + ((node_793) * (node_419)))
                + ((node_798) * (node_595)))
                + ((node_802) * (node_413)))
                + ((node_805) * (node_594)))
                + ((node_808) * (node_436)))
                + ((node_812) * (node_411)))
                + ((node_821) * (node_583)))
                + ((node_823) * (node_595)))
                + ((node_825) * (node_595)))
                + ((node_830) * (node_417)))
                + ((node_833) * (node_417)))
                + ((node_835) * (node_595)))
                + ((node_838) * (node_411)))
                + ((node_845) * (node_395)))
                + ((node_850) * (node_418)))
                + ((node_854) * (node_418)))
                + ((node_858) * (node_418)))
                + ((node_861) * (node_418)))
                + ((node_864) * (node_418)))
                + ((node_867) * (node_418)))
                + ((node_870) * (node_418)))
                + ((node_872) * (node_395)))
                + ((node_874) * (node_395)))
                + ((node_876) * (node_395)))
                + ((node_879) * (node_395)))
                + ((node_881) * (node_418)))
                + ((node_883) * (node_417)))
                * (node_3965))
                + ((node_592) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((node_747)
                * (node_418))
                + ((node_757) * (node_418)))
                + ((node_765) * (node_419)))
                + ((node_770) * (node_393)))
                + ((node_775) * (node_472)))
                + ((node_780) * (node_418)))
                + ((node_785) * (node_413)))
                + ((node_789) * (node_418)))
                + ((node_798) * (node_418)))
                + ((node_802) * (node_417)))
                + ((node_805) * (node_595)))
                + ((node_808) * (node_438)))
                + ((node_812) * (node_413)))
                + ((node_821) * (node_586)))
                + ((node_823) * (node_418)))
                + ((node_825) * (node_418)))
                + ((node_830) * (node_418)))
                + ((node_833) * (node_418)))
                + ((node_835) * (node_418)))
                + ((node_838) * (node_413)))
                + ((node_845) * (node_418)))
                + ((node_850) * (node_419)))
                + ((node_854) * (node_419)))
                + ((node_858) * (node_419)))
                + ((node_861) * (node_419)))
                + ((node_864) * (node_419)))
                + ((node_867) * (node_419)))
                + ((node_870) * (node_419)))
                + ((node_872) * (node_418)))
                + ((node_874) * (node_418)))
                + ((node_876) * (node_418)))
                + ((node_879) * (node_418)))
                + ((node_881) * (node_419)))
                + ((node_883) * (node_418)))
                * (node_3965))
                + ((node_594) * (next_base_row[5])),
            ((((((((((((((((((((((((((node_747) * (node_419))
                + ((node_757) * (node_419)))
                + ((node_770) * (node_394)))
                + ((node_775) * (node_477)))
                + ((node_780) * (node_419)))
                + ((node_785) * (node_417)))
                + ((node_789) * (node_419)))
                + ((node_798) * (node_419)))
                + ((node_802) * (node_418)))
                + ((node_805) * (node_418)))
                + ((node_812) * (node_417)))
                + ((node_821) * (node_589)))
                + ((node_823) * (node_419)))
                + ((node_825) * (node_419)))
                + ((node_830) * (node_419)))
                + ((node_833) * (node_419)))
                + ((node_835) * (node_419)))
                + ((node_838) * (node_417)))
                + ((node_845) * (node_419)))
                + ((node_872) * (node_419)))
                + ((node_874) * (node_419)))
                + ((node_876) * (node_419)))
                + ((node_879) * (node_419)))
                + ((node_883) * (node_419)))
                * (node_3965))
                + ((node_595) * (next_base_row[5])),
            (((((((((node_770) * (node_395)) + ((node_775) * (node_480)))
                + ((node_785) * (node_418)))
                + ((node_802) * (node_419)))
                + ((node_805) * (node_419)))
                + ((node_821) * (node_592)))
                + ((node_838) * (node_418)))
                * (node_3965))
                + ((node_418) * (next_base_row[5])),
            (((((((node_770) * (node_421)) + ((node_775) * (node_484)))
                + ((node_785) * (node_419)))
                + ((node_821) * (node_594)))
                + ((node_838) * (node_419)))
                * (node_3965))
                + ((node_419) * (next_base_row[5])),
            ((((node_770) * (node_422)) + ((node_775) * (node_487))) + ((node_821) * (node_595)))
                * (node_3965),
            ((((node_770) * (node_423)) + ((node_775) * (node_492))) + ((node_821) * (node_418)))
                * (node_3965),
            ((((node_770) * (node_424)) + ((node_775) * (node_495))) + ((node_821) * (node_419)))
                * (node_3965),
            (((node_770) * (node_425)) + ((node_775) * (node_499))) * (node_3965),
            (((node_770) * (node_426)) + ((node_775) * (node_502))) * (node_3965),
            (((node_770) * (node_427))
                + ((node_775) * (((BFieldElement::new(1)) - (node_449)) * (node_550))))
                * (node_3965),
            (((node_770) * (node_428))
                + ((node_775) * (((BFieldElement::new(1)) - (node_452)) * (node_553))))
                * (node_3965),
            (((node_770) * (node_429))
                + ((node_775) * (((BFieldElement::new(1)) - (node_455)) * (node_556))))
                * (node_3965),
            (((node_770) * (node_430))
                + ((node_775) * (((BFieldElement::new(1)) - (node_460)) * (node_559))))
                * (node_3965),
            (((node_770) * (node_431))
                + ((node_775) * (((BFieldElement::new(1)) - (node_463)) * (node_562))))
                * (node_3965),
            (((node_770) * (node_432))
                + ((node_775) * (((BFieldElement::new(1)) - (node_467)) * (node_565))))
                * (node_3965),
            (((node_770) * (node_433))
                + ((node_775) * (((BFieldElement::new(1)) - (node_470)) * (node_568))))
                * (node_3965),
            (((node_770) * (node_434))
                + ((node_775) * (((BFieldElement::new(1)) - (node_475)) * (node_571))))
                * (node_3965),
            (((node_770) * (node_435))
                + ((node_775) * (((BFieldElement::new(1)) - (node_478)) * (node_574))))
                * (node_3965),
            (((node_770) * (node_436))
                + ((node_775) * (((BFieldElement::new(1)) - (node_482)) * (node_577))))
                * (node_3965),
            (((node_770) * (node_438))
                + ((node_775) * (((BFieldElement::new(1)) - (node_485)) * (node_580))))
                * (node_3965),
            (((node_770) * (node_418))
                + ((node_775) * (((BFieldElement::new(1)) - (node_490)) * (node_583))))
                * (node_3965),
            (((node_770) * (node_419))
                + ((node_775) * (((BFieldElement::new(1)) - (node_493)) * (node_586))))
                * (node_3965),
            ((node_775) * (((BFieldElement::new(1)) - (node_497)) * (node_589))) * (node_3965),
            ((node_775) * (((BFieldElement::new(1)) - (node_500)) * (node_592))) * (node_3965),
            ((node_775) * (node_594)) * (node_3965),
            ((node_775) * (node_595)) * (node_3965),
            ((node_775) * (node_504)) * (node_3965),
            ((node_775) * (node_506)) * (node_3965),
            ((node_775) * (node_508)) * (node_3965),
            ((node_775) * (node_510)) * (node_3965),
            ((node_775) * (node_519)) * (node_3965),
            ((node_775) * (node_439)) * (node_3965),
            ((node_775) * (node_393)) * (node_3965),
            ((node_775) * (node_394)) * (node_3965),
            ((node_775) * (node_395)) * (node_3965),
            ((node_775) * (node_418)) * (node_3965),
            ((node_775) * (node_419)) * (node_3965),
        ];
        let ext_constraints = [
            (((next_ext_row[11]) - (current_ext_row[11]))
                * ((challenges.get_challenge(ClockJumpDifferenceLookupIndeterminate))
                    - (next_base_row[4])))
                - (next_base_row[45]),
            (((current_base_row[8]) - (BFieldElement::new(128)))
                * ((next_ext_row[1]) - (current_ext_row[1])))
                + ((node_881)
                    * (((next_ext_row[1])
                        - ((challenges.get_challenge(StandardInputIndeterminate))
                            * (current_ext_row[1])))
                        - (next_base_row[21]))),
            ((node_3965)
                * (((node_4141)
                    * ((challenges.get_challenge(InstructionLookupIndeterminate))
                        - ((((challenges.get_challenge(ProgramAddressWeight))
                            * (next_base_row[7]))
                            + ((challenges.get_challenge(ProgramInstructionWeight))
                                * (next_base_row[8])))
                            + ((challenges.get_challenge(ProgramNextInstructionWeight))
                                * (next_base_row[9])))))
                    - (BFieldElement::new(1))))
                + ((next_base_row[5]) * (node_4141)),
            (((next_base_row[8]) - (BFieldElement::new(66)))
                * ((next_ext_row[2]) - (current_ext_row[2])))
                + (((((((((node_4149) * (next_base_row[11])) * (node_4153)) * (node_4155))
                    * ((next_base_row[14]) - (BFieldElement::new(1))))
                    * ((next_base_row[15]) - (BFieldElement::new(1))))
                    * (next_base_row[16]))
                    * (node_4163))
                    * (((next_ext_row[2])
                        - ((challenges.get_challenge(StandardOutputIndeterminate))
                            * (current_ext_row[2])))
                        - (next_base_row[21]))),
            (next_ext_row[4])
                - ((current_ext_row[4])
                    * ((challenges.get_challenge(OpStackIndeterminate))
                        - (((((challenges.get_challenge(OpStackClkWeight))
                            * (next_base_row[4]))
                            + ((challenges.get_challenge(OpStackIb1Weight))
                                * (next_base_row[11])))
                            + ((challenges.get_challenge(OpStackOspWeight))
                                * (next_base_row[37])))
                            + ((challenges.get_challenge(OpStackOsvWeight))
                                * (next_base_row[38]))))),
            (next_ext_row[5])
                - ((current_ext_row[5])
                    * ((challenges.get_challenge(RamIndeterminate))
                        - (((((challenges.get_challenge(RamClkWeight)) * (next_base_row[4]))
                            + ((challenges.get_challenge(RamRampWeight))
                                * (next_base_row[43])))
                            + ((challenges.get_challenge(RamRamvWeight)) * (next_base_row[44])))
                            + ((challenges.get_challenge(RamPreviousInstructionWeight))
                                * (next_base_row[6]))))),
            (next_ext_row[6])
                - ((current_ext_row[6])
                    * ((challenges.get_challenge(JumpStackIndeterminate))
                        - ((((((challenges.get_challenge(JumpStackClkWeight))
                            * (next_base_row[4]))
                            + ((challenges.get_challenge(JumpStackCiWeight))
                                * (next_base_row[8])))
                            + ((challenges.get_challenge(JumpStackJspWeight))
                                * (next_base_row[18])))
                            + ((challenges.get_challenge(JumpStackJsoWeight))
                                * (next_base_row[19])))
                            + ((challenges.get_challenge(JumpStackJsdWeight))
                                * (next_base_row[20]))))),
            (((next_base_row[8]) - (BFieldElement::new(48)))
                * ((next_ext_row[7]) - (current_ext_row[7])))
                + (((((((((node_4149) * ((next_base_row[11]) - (BFieldElement::new(1))))
                    * (node_4153))
                    * (node_4155))
                    * (next_base_row[14]))
                    * (next_base_row[15]))
                    * ((next_base_row[16]) - (BFieldElement::new(1))))
                    * (node_4163))
                    * (((next_ext_row[7])
                        - ((challenges.get_challenge(HashInputIndeterminate))
                            * (current_ext_row[7])))
                        - (node_4264))),
            (((current_base_row[8]) - (BFieldElement::new(48)))
                * ((next_ext_row[8]) - (current_ext_row[8])))
                + ((node_815)
                    * (((next_ext_row[8])
                        - ((challenges.get_challenge(HashDigestIndeterminate))
                            * (current_ext_row[8])))
                        - ((((((challenges.get_challenge(HashStateWeight0))
                            * (next_base_row[26]))
                            + ((challenges.get_challenge(HashStateWeight1))
                                * (next_base_row[27])))
                            + ((challenges.get_challenge(HashStateWeight2))
                                * (next_base_row[28])))
                            + ((challenges.get_challenge(HashStateWeight3))
                                * (next_base_row[29])))
                            + ((challenges.get_challenge(HashStateWeight4))
                                * (next_base_row[30]))))),
            (((((((current_base_row[8]) - (BFieldElement::new(72)))
                * ((current_base_row[8]) - (BFieldElement::new(80))))
                * ((current_base_row[8]) - (BFieldElement::new(88))))
                * ((next_ext_row[9]) - (current_ext_row[9])))
                + ((node_823) * (node_4304)))
                + ((node_825) * (node_4304)))
                + ((node_827) * (node_4304)),
            (((((((((node_845)
                * (((node_4349) * (((node_4321) - (node_4322)) - (node_4324)))
                    - (BFieldElement::new(1))))
                + ((node_850) * (node_4354)))
                + ((node_854) * (node_4354)))
                + ((node_858)
                    * (((node_4349)
                        * (((node_4329)
                            - ((challenges.get_challenge(U32CiWeight))
                                * (BFieldElement::new(20))))
                            - (((challenges.get_challenge(U32ResultWeight))
                                * ((node_658) - (next_base_row[21])))
                                * (BFieldElement::new(9223372034707292161)))))
                        - (BFieldElement::new(1)))))
                + ((node_864) * (node_4354)))
                + ((node_861) * (node_4362)))
                + ((node_867)
                    * (((((node_4349) * (node_4345)) * (node_4348)) - (node_4345))
                        - (node_4348))))
                + ((node_870) * (node_4362)))
                + (((BFieldElement::new(1)) - (current_base_row[12])) * (node_4349)),
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
        let base_constraints = [base_row[8]];
        let ext_constraints = [];
        base_constraints
            .into_iter()
            .map(|bfe| bfe.lift())
            .chain(ext_constraints.into_iter())
            .collect()
    }
}

impl Evaluable<XFieldElement> for ExtProcessorTable {
    #[inline]
    #[allow(unused_variables)]
    fn evaluate_initial_constraints(
        base_row: ArrayView1<XFieldElement>,
        ext_row: ArrayView1<XFieldElement>,
        challenges: &Challenges,
    ) -> Vec<XFieldElement> {
        let base_constraints = [
            base_row[4],
            base_row[7],
            base_row[18],
            base_row[19],
            base_row[20],
            base_row[21],
            base_row[22],
            base_row[23],
            base_row[24],
            base_row[25],
            base_row[26],
            base_row[27],
            base_row[28],
            base_row[29],
            base_row[30],
            base_row[31],
            base_row[32],
            base_row[33],
            base_row[34],
            base_row[35],
            base_row[36],
            (base_row[37]) - (BFieldElement::new(16)),
            base_row[38],
            base_row[44],
            base_row[43],
            base_row[6],
        ];
        let ext_constraints = [
            (ext_row[1]) - (BFieldElement::new(1)),
            ((ext_row[3])
                * ((challenges.get_challenge(InstructionLookupIndeterminate))
                    - (((challenges.get_challenge(ProgramInstructionWeight)) * (base_row[8]))
                        + ((challenges.get_challenge(ProgramNextInstructionWeight))
                            * (base_row[9])))))
                - (BFieldElement::new(1)),
            (ext_row[2]) - (BFieldElement::new(1)),
            (ext_row[4])
                - ((challenges.get_challenge(OpStackIndeterminate))
                    - (((challenges.get_challenge(OpStackIb1Weight)) * (base_row[11]))
                        + ((challenges.get_challenge(OpStackOspWeight))
                            * (BFieldElement::new(16))))),
            (ext_row[5]) - (challenges.get_challenge(RamIndeterminate)),
            (ext_row[6])
                - ((challenges.get_challenge(JumpStackIndeterminate))
                    - ((challenges.get_challenge(JumpStackCiWeight)) * (base_row[8]))),
            ext_row[11],
            (((base_row[8]) - (BFieldElement::new(48))) * ((ext_row[7]) - (BFieldElement::new(1))))
                + ((((((((((base_row[10]) - (BFieldElement::new(1)))
                    * ((base_row[11]) - (BFieldElement::new(1))))
                    * ((base_row[12]) - (BFieldElement::new(1))))
                    * ((base_row[13]) - (BFieldElement::new(1))))
                    * (base_row[14]))
                    * (base_row[15]))
                    * ((base_row[16]) - (BFieldElement::new(1))))
                    * ((base_row[17]) - (BFieldElement::new(1))))
                    * ((ext_row[7]) - (challenges.get_challenge(HashInputIndeterminate)))),
            (ext_row[8]) - (BFieldElement::new(1)),
            (ext_row[9]) - (BFieldElement::new(1)),
            ext_row[10],
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
        let base_constraints = [
            (base_row[10]) * ((base_row[10]) - (BFieldElement::new(1))),
            (base_row[11]) * ((base_row[11]) - (BFieldElement::new(1))),
            (base_row[12]) * ((base_row[12]) - (BFieldElement::new(1))),
            (base_row[13]) * ((base_row[13]) - (BFieldElement::new(1))),
            (base_row[14]) * ((base_row[14]) - (BFieldElement::new(1))),
            (base_row[15]) * ((base_row[15]) - (BFieldElement::new(1))),
            (base_row[16]) * ((base_row[16]) - (BFieldElement::new(1))),
            (base_row[17]) * ((base_row[17]) - (BFieldElement::new(1))),
            (base_row[5]) * ((base_row[5]) - (BFieldElement::new(1))),
            (base_row[8])
                - ((((((((base_row[10]) + ((BFieldElement::new(2)) * (base_row[11])))
                    + ((BFieldElement::new(4)) * (base_row[12])))
                    + ((BFieldElement::new(8)) * (base_row[13])))
                    + ((BFieldElement::new(16)) * (base_row[14])))
                    + ((BFieldElement::new(32)) * (base_row[15])))
                    + ((BFieldElement::new(64)) * (base_row[16])))
                    + ((BFieldElement::new(128)) * (base_row[17]))),
            ((base_row[5]) * ((base_row[4]) - (BFieldElement::new(1)))) * (base_row[45]),
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
        let node_746 = (current_base_row[17]) - (BFieldElement::new(1));
        let node_732 = (current_base_row[10]) - (BFieldElement::new(1));
        let node_736 = (current_base_row[12]) - (BFieldElement::new(1));
        let node_744 = (current_base_row[16]) - (BFieldElement::new(1));
        let node_750 = (current_base_row[11]) - (BFieldElement::new(1));
        let node_742 = (current_base_row[15]) - (BFieldElement::new(1));
        let node_758 = (node_732) * (node_750);
        let node_738 = (current_base_row[13]) - (BFieldElement::new(1));
        let node_740 = (current_base_row[14]) - (BFieldElement::new(1));
        let node_759 = (node_758) * (node_736);
        let node_737 = ((node_732) * (current_base_row[11])) * (node_736);
        let node_776 = (node_759) * (node_738);
        let node_840 = (node_758) * (current_base_row[12]);
        let node_761 = (node_759) * (current_base_row[13]);
        let node_752 = ((current_base_row[10]) * (node_750)) * (node_736);
        let node_794 = (node_776) * (node_740);
        let node_739 = (node_737) * (node_738);
        let node_781 = (node_737) * (current_base_row[13]);
        let node_841 = (node_840) * (node_738);
        let node_846 = (node_840) * (current_base_row[13]);
        let node_762 = (node_761) * (node_740);
        let node_777 = (node_776) * (current_base_row[14]);
        let node_753 = (node_752) * (node_738);
        let node_790 = (node_761) * (current_base_row[14]);
        let node_803 = (node_794) * (node_742);
        let node_741 = (node_739) * (node_740);
        let node_766 = (node_752) * (current_base_row[13]);
        let node_3965 = (BFieldElement::new(1)) - (next_base_row[5]);
        let node_775 =
            ((((node_753) * (current_base_row[14])) * (node_742)) * (node_744)) * (node_746);
        let node_799 = (node_739) * (current_base_row[14]);
        let node_782 = (node_781) * (node_740);
        let node_778 = (node_777) * (node_742);
        let node_804 = (node_803) * (node_744);
        let node_842 = (node_841) * (node_740);
        let node_743 = (node_741) * (node_742);
        let node_763 = (node_762) * (node_742);
        let node_796 = (node_794) * (current_base_row[15]);
        let node_809 = (node_781) * (current_base_row[14]);
        let node_847 = (node_846) * (node_740);
        let node_851 = (node_841) * (current_base_row[14]);
        let node_855 = (node_846) * (current_base_row[14]);
        let node_806 = (node_762) * (current_base_row[15]);
        let node_770 = ((((node_766) * (node_740)) * (node_742)) * (node_744)) * (node_746);
        let node_816 = (node_790) * (current_base_row[15]);
        let node_813 = (node_777) * (current_base_row[15]);
        let node_391 = (next_base_row[7]) - (current_base_row[7]);
        let node_791 = (node_790) * (node_742);
        let node_393 = (next_base_row[18]) - (current_base_row[18]);
        let node_394 = (next_base_row[19]) - (current_base_row[19]);
        let node_395 = (next_base_row[20]) - (current_base_row[20]);
        let node_418 = (next_base_row[44]) - (current_base_row[44]);
        let node_419 = (next_base_row[43]) - (current_base_row[43]);
        let node_445 = (BFieldElement::new(1)) - (current_base_row[39]);
        let node_392 = (node_391) - (BFieldElement::new(1));
        let node_440 = (BFieldElement::new(1)) - (current_base_row[42]);
        let node_441 = (BFieldElement::new(1)) - (current_base_row[41]);
        let node_443 = (BFieldElement::new(1)) - (current_base_row[40]);
        let node_821 = ((node_803) * (current_base_row[16])) * (node_746);
        let node_785 = (((node_782) * (node_742)) * (node_744)) * (node_746);
        let node_838 = (((node_799) * (current_base_row[15])) * (node_744)) * (node_746);
        let node_802 = (((node_799) * (node_742)) * (node_744)) * (node_746);
        let node_805 = (node_804) * (node_746);
        let node_823 = ((node_763) * (current_base_row[16])) * (node_746);
        let node_825 = ((node_778) * (current_base_row[16])) * (node_746);
        let node_845 = (((node_842) * (node_742)) * (node_744)) * (node_746);
        let node_747 = ((node_743) * (node_744)) * (node_746);
        let node_757 = ((((node_753) * (node_740)) * (node_742)) * (node_744)) * (node_746);
        let node_780 = ((node_778) * (node_744)) * (node_746);
        let node_789 =
            ((((node_766) * (current_base_row[14])) * (node_742)) * (node_744)) * (node_746);
        let node_798 = ((node_796) * (node_744)) * (node_746);
        let node_812 = (((node_809) * (node_742)) * (node_744)) * (node_746);
        let node_830 = (((node_741) * (current_base_row[15])) * (node_744)) * (node_746);
        let node_833 = (((node_782) * (current_base_row[15])) * (node_744)) * (node_746);
        let node_835 = ((node_796) * (current_base_row[16])) * (node_746);
        let node_850 = (((node_847) * (node_742)) * (node_744)) * (node_746);
        let node_854 = (((node_851) * (node_742)) * (node_744)) * (node_746);
        let node_858 = (((node_855) * (node_742)) * (node_744)) * (node_746);
        let node_861 = (((node_842) * (current_base_row[15])) * (node_744)) * (node_746);
        let node_864 = (((node_847) * (current_base_row[15])) * (node_744)) * (node_746);
        let node_867 = (((node_851) * (current_base_row[15])) * (node_744)) * (node_746);
        let node_870 = (((node_855) * (current_base_row[15])) * (node_744)) * (node_746);
        let node_872 = ((node_806) * (current_base_row[16])) * (node_746);
        let node_874 = ((node_813) * (current_base_row[16])) * (node_746);
        let node_876 = ((node_816) * (current_base_row[16])) * (node_746);
        let node_879 = (((node_809) * (current_base_row[15])) * (node_744)) * (node_746);
        let node_881 = (node_804) * (current_base_row[17]);
        let node_883 = ((node_743) * (current_base_row[16])) * (node_746);
        let node_765 = ((node_763) * (node_744)) * (node_746);
        let node_808 = ((node_806) * (node_744)) * (node_746);
        let node_793 = ((node_791) * (node_744)) * (node_746);
        let node_818 = ((node_816) * (node_744)) * (node_746);
        let node_580 = (next_base_row[32]) - (current_base_row[32]);
        let node_583 = (next_base_row[33]) - (current_base_row[33]);
        let node_586 = (next_base_row[34]) - (current_base_row[34]);
        let node_589 = (next_base_row[35]) - (current_base_row[35]);
        let node_592 = (next_base_row[36]) - (current_base_row[36]);
        let node_594 = (next_base_row[38]) - (current_base_row[38]);
        let node_595 = (next_base_row[37]) - (current_base_row[37]);
        let node_577 = (next_base_row[31]) - (current_base_row[31]);
        let node_556 = (next_base_row[24]) - (current_base_row[24]);
        let node_559 = (next_base_row[25]) - (current_base_row[25]);
        let node_562 = (next_base_row[26]) - (current_base_row[26]);
        let node_565 = (next_base_row[27]) - (current_base_row[27]);
        let node_568 = (next_base_row[28]) - (current_base_row[28]);
        let node_571 = (next_base_row[29]) - (current_base_row[29]);
        let node_574 = (next_base_row[30]) - (current_base_row[30]);
        let node_458 = (node_440) * (current_base_row[41]);
        let node_473 = (current_base_row[42]) * (node_441);
        let node_488 = (current_base_row[42]) * (current_base_row[41]);
        let node_815 = ((node_813) * (node_744)) * (node_746);
        let node_827 = ((node_791) * (current_base_row[16])) * (node_746);
        let node_442 = (node_440) * (node_441);
        let node_553 = (next_base_row[23]) - (current_base_row[23]);
        let node_550 = (next_base_row[22]) - (current_base_row[22]);
        let node_399 = (next_base_row[24]) - (current_base_row[25]);
        let node_400 = (next_base_row[25]) - (current_base_row[26]);
        let node_401 = (next_base_row[26]) - (current_base_row[27]);
        let node_402 = (next_base_row[27]) - (current_base_row[28]);
        let node_403 = (next_base_row[28]) - (current_base_row[29]);
        let node_404 = (next_base_row[29]) - (current_base_row[30]);
        let node_405 = (next_base_row[30]) - (current_base_row[31]);
        let node_406 = (next_base_row[31]) - (current_base_row[32]);
        let node_407 = (next_base_row[32]) - (current_base_row[33]);
        let node_408 = (next_base_row[33]) - (current_base_row[34]);
        let node_409 = (next_base_row[34]) - (current_base_row[35]);
        let node_410 = (next_base_row[35]) - (current_base_row[36]);
        let node_411 = (next_base_row[36]) - (current_base_row[38]);
        let node_413 = (next_base_row[37]) - ((current_base_row[37]) - (BFieldElement::new(1)));
        let node_417 = (((current_base_row[37]) - (BFieldElement::new(16)))
            * (current_base_row[42]))
            - (BFieldElement::new(1));
        let node_397 = (next_base_row[22]) - (current_base_row[23]);
        let node_398 = (next_base_row[23]) - (current_base_row[24]);
        let node_447 = (next_base_row[21]) - (current_base_row[21]);
        let node_4349 = (next_ext_row[10]) - (current_ext_row[10]);
        let node_451 = (node_442) * (current_base_row[40]);
        let node_459 = (node_458) * (node_443);
        let node_466 = (node_458) * (current_base_row[40]);
        let node_474 = (node_473) * (node_443);
        let node_481 = (node_473) * (current_base_row[40]);
        let node_489 = (node_488) * (node_443);
        let node_496 = (node_488) * (current_base_row[40]);
        let node_4327 = (challenges.get_challenge(U32Indeterminate))
            - ((challenges.get_challenge(U32LhsWeight)) * (current_base_row[21]));
        let node_396 = (next_base_row[21]) - (current_base_row[22]);
        let node_444 = (node_442) * (node_443);
        let node_421 = (next_base_row[22]) - (current_base_row[21]);
        let node_422 = (next_base_row[23]) - (current_base_row[22]);
        let node_423 = (next_base_row[24]) - (current_base_row[23]);
        let node_424 = (next_base_row[25]) - (current_base_row[24]);
        let node_425 = (next_base_row[26]) - (current_base_row[25]);
        let node_426 = (next_base_row[27]) - (current_base_row[26]);
        let node_427 = (next_base_row[28]) - (current_base_row[27]);
        let node_428 = (next_base_row[29]) - (current_base_row[28]);
        let node_429 = (next_base_row[30]) - (current_base_row[29]);
        let node_430 = (next_base_row[31]) - (current_base_row[30]);
        let node_431 = (next_base_row[32]) - (current_base_row[31]);
        let node_432 = (next_base_row[33]) - (current_base_row[32]);
        let node_433 = (next_base_row[34]) - (current_base_row[33]);
        let node_434 = (next_base_row[35]) - (current_base_row[34]);
        let node_435 = (next_base_row[36]) - (current_base_row[35]);
        let node_436 = (next_base_row[38]) - (current_base_row[36]);
        let node_438 = (next_base_row[37]) - ((current_base_row[37]) + (BFieldElement::new(1)));
        let node_4324 = (challenges.get_challenge(U32CiWeight)) * (current_base_row[8]);
        let node_4328 = (challenges.get_challenge(U32RhsWeight)) * (current_base_row[22]);
        let node_503 = (current_base_row[39]) - (BFieldElement::new(1));
        let node_4331 = (challenges.get_challenge(U32ResultWeight)) * (next_base_row[21]);
        let node_504 = (current_base_row[39]) * (node_503);
        let node_664 = (current_base_row[22]) - (current_base_row[21]);
        let node_449 = (node_444) * (current_base_row[39]);
        let node_452 = (node_451) * (node_445);
        let node_439 = (node_391) - (BFieldElement::new(2));
        let node_455 = (node_451) * (current_base_row[39]);
        let node_460 = (node_459) * (node_445);
        let node_463 = (node_459) * (current_base_row[39]);
        let node_467 = (node_466) * (node_445);
        let node_470 = (node_466) * (current_base_row[39]);
        let node_475 = (node_474) * (node_445);
        let node_478 = (node_474) * (current_base_row[39]);
        let node_482 = (node_481) * (node_445);
        let node_485 = (node_481) * (current_base_row[39]);
        let node_490 = (node_489) * (node_445);
        let node_493 = (node_489) * (current_base_row[39]);
        let node_497 = (node_496) * (node_445);
        let node_500 = (node_496) * (current_base_row[39]);
        let node_4264 = ((((((((((challenges.get_challenge(HashStateWeight0))
            * (next_base_row[21]))
            + ((challenges.get_challenge(HashStateWeight1)) * (next_base_row[22])))
            + ((challenges.get_challenge(HashStateWeight2)) * (next_base_row[23])))
            + ((challenges.get_challenge(HashStateWeight3)) * (next_base_row[24])))
            + ((challenges.get_challenge(HashStateWeight4)) * (next_base_row[25])))
            + ((challenges.get_challenge(HashStateWeight5)) * (next_base_row[26])))
            + ((challenges.get_challenge(HashStateWeight6)) * (next_base_row[27])))
            + ((challenges.get_challenge(HashStateWeight7)) * (next_base_row[28])))
            + ((challenges.get_challenge(HashStateWeight8)) * (next_base_row[29])))
            + ((challenges.get_challenge(HashStateWeight9)) * (next_base_row[30]));
        let node_4329 = (node_4327) - (node_4328);
        let node_517 = (BFieldElement::new(2)) * (current_base_row[40]);
        let node_665 = (current_base_row[39]) * (node_664);
        let node_4304 = (((next_ext_row[9])
            - ((challenges.get_challenge(SpongeIndeterminate)) * (current_ext_row[9])))
            - ((challenges.get_challenge(HashCIWeight)) * (current_base_row[8])))
            - (node_4264);
        let node_4321 = (challenges.get_challenge(U32Indeterminate))
            - ((challenges.get_challenge(U32LhsWeight)) * (next_base_row[21]));
        let node_4322 = (challenges.get_challenge(U32RhsWeight)) * (next_base_row[22]);
        let node_4354 =
            ((node_4349) * (((node_4329) - (node_4324)) - (node_4331))) - (BFieldElement::new(1));
        let node_446 = (node_444) * (node_445);
        let node_620 = (next_base_row[8]) - (current_base_row[8]);
        let node_658 = (current_base_row[21]) + (current_base_row[22]);
        let node_661 = (next_base_row[21]) - ((current_base_row[21]) * (current_base_row[22]));
        let node_662 = (next_base_row[21]) * (current_base_row[21]);
        let node_666 = (node_665) - (BFieldElement::new(1));
        let node_680 = (current_base_row[22]) * (next_base_row[22]);
        let node_689 = (current_base_row[21]) * (current_base_row[24]);
        let node_690 = (current_base_row[23]) * (current_base_row[25]);
        let node_692 = (current_base_row[22]) * (current_base_row[26]);
        let node_710 = (current_base_row[23]) * (next_base_row[22]);
        let node_712 = (current_base_row[22]) * (next_base_row[23]);
        let node_450 = (node_449) * (node_396);
        let node_698 = (current_base_row[23]) * (current_base_row[26]);
        let node_718 = (current_base_row[23]) * (next_base_row[23]);
        let node_454 = (node_452) * ((next_base_row[21]) - (current_base_row[23]));
        let node_600 = ((current_base_row[21]) * (current_base_row[41])) - (BFieldElement::new(1));
        let node_457 = (node_455) * ((next_base_row[21]) - (current_base_row[24]));
        let node_462 = (node_460) * ((next_base_row[21]) - (current_base_row[25]));
        let node_465 = (node_463) * ((next_base_row[21]) - (current_base_row[26]));
        let node_469 = (node_467) * ((next_base_row[21]) - (current_base_row[27]));
        let node_472 = (node_470) * ((next_base_row[21]) - (current_base_row[28]));
        let node_477 = (node_475) * ((next_base_row[21]) - (current_base_row[29]));
        let node_480 = (node_478) * ((next_base_row[21]) - (current_base_row[30]));
        let node_484 = (node_482) * ((next_base_row[21]) - (current_base_row[31]));
        let node_487 = (node_485) * ((next_base_row[21]) - (current_base_row[32]));
        let node_492 = (node_490) * ((next_base_row[21]) - (current_base_row[33]));
        let node_495 = (node_493) * ((next_base_row[21]) - (current_base_row[34]));
        let node_499 = (node_497) * ((next_base_row[21]) - (current_base_row[35]));
        let node_502 = (node_500) * ((next_base_row[21]) - (current_base_row[36]));
        let node_506 = (current_base_row[40]) * ((current_base_row[40]) - (BFieldElement::new(1)));
        let node_508 = (current_base_row[41]) * ((current_base_row[41]) - (BFieldElement::new(1)));
        let node_510 = (current_base_row[42]) * ((current_base_row[42]) - (BFieldElement::new(1)));
        let node_519 = ((((current_base_row[9])
            - ((BFieldElement::new(8)) * (current_base_row[42])))
            - ((BFieldElement::new(4)) * (current_base_row[41])))
            - (node_517))
            - (current_base_row[39]);
        let node_4141 = (next_ext_row[3]) - (current_ext_row[3]);
        let node_4149 = (next_base_row[10]) - (BFieldElement::new(1));
        let node_4153 = (next_base_row[12]) - (BFieldElement::new(1));
        let node_4155 = (next_base_row[13]) - (BFieldElement::new(1));
        let node_4163 = (next_base_row[17]) - (BFieldElement::new(1));
        let node_4362 =
            ((node_4349) * (((node_4327) - (node_4324)) - (node_4331))) - (BFieldElement::new(1));
        let node_4345 = (((node_4321) - (node_4328))
            - ((challenges.get_challenge(U32CiWeight)) * (BFieldElement::new(12))))
            - (challenges.get_challenge(U32ResultWeight));
        let node_4348 = ((node_4327) - (node_4322))
            - ((challenges.get_challenge(U32CiWeight)) * (BFieldElement::new(4)));

        let base_constraints = [
            ((next_base_row[4]) - (current_base_row[4])) - (BFieldElement::new(1)),
            (current_base_row[5]) * ((next_base_row[5]) - (current_base_row[5])),
            ((next_base_row[6]) - (current_base_row[8])) * (node_3965),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_392))
                + ((node_757)
                    * ((next_base_row[21])
                        - (current_base_row[9]))))
                + ((node_765) * (node_392)))
                + ((node_770) * ((node_446) * (node_447))))
                + ((node_775) * (node_446)))
                + ((node_780) * (node_392)))
                + ((node_785)
                    * ((current_base_row[9])
                        - ((current_base_row[39]) + (node_517)))))
                + ((node_789)
                    * ((next_base_row[18])
                        - ((current_base_row[18])
                            + (BFieldElement::new(1))))))
                + ((node_793)
                    * ((next_base_row[18])
                        - ((current_base_row[18])
                            - (BFieldElement::new(1))))))
                + ((node_798)
                    * ((next_base_row[7]) - (current_base_row[20]))))
                + ((node_802)
                    * ((current_base_row[21])
                        - (BFieldElement::new(1)))))
                + ((node_805) * (node_620)))
                + ((node_808)
                    * ((next_base_row[43]) - (current_base_row[21]))))
                + ((node_812)
                    * ((next_base_row[43]) - (current_base_row[22]))))
                + ((node_815) * (node_392)))
                + ((node_818) * (node_504)))
                + ((node_821)
                    * ((current_base_row[26]) - (current_base_row[21]))))
                + ((node_823) * (node_392)))
                + ((node_825) * (node_392)))
                + ((node_827) * (node_392)))
                + ((node_830) * ((next_base_row[21]) - (node_658))))
                + ((node_833) * (node_661)))
                + ((node_835) * ((node_662) - (BFieldElement::new(1)))))
                + ((node_838) * ((current_base_row[39]) * (node_666))))
                + ((node_845)
                    * ((current_base_row[21])
                        - (((BFieldElement::new(4294967296))
                            * (next_base_row[22]))
                            + (next_base_row[21])))))
                + ((node_850) * (node_392)))
                + ((node_854) * (node_392)))
                + ((node_858) * (node_392)))
                + ((node_861) * (node_392)))
                + ((node_864) * (node_392)))
                + ((node_867)
                    * (((current_base_row[21]) - (node_680)) - (next_base_row[21]))))
                + ((node_870) * (node_392)))
                + ((node_872)
                    * ((next_base_row[21])
                        - ((current_base_row[21]) + (current_base_row[24])))))
                + ((node_874)
                    * ((next_base_row[21]) - (((node_689) - (node_690)) - (node_692)))))
                + ((node_876)
                    * ((((node_662) - (node_710)) - (node_712)) - (BFieldElement::new(1)))))
                + ((node_879) * (node_661)))
                + ((node_881) * (node_392)))
                + ((node_883) * (node_392)))
                * (node_3965))
                + ((node_391) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_393))
                + ((node_757) * (node_421)))
                + ((node_765) * (node_393)))
                + ((node_770) * (node_450)))
                + ((node_775) * ((node_449) * (node_421))))
                + ((node_780) * (node_393)))
                + ((node_785) * (node_504)))
                + ((node_789)
                    * (((next_base_row[19])
                        - (current_base_row[7]))
                        - (BFieldElement::new(2)))))
                + ((node_793)
                    * ((next_base_row[7]) - (current_base_row[19]))))
                + ((node_798) * (node_393)))
                + ((node_802) * (node_392)))
                + ((node_805) * (node_392)))
                + ((node_808)
                    * ((next_base_row[21]) - (next_base_row[44]))))
                + ((node_812)
                    * ((next_base_row[44]) - (current_base_row[21]))))
                + ((node_815) * (node_393)))
                + ((node_818)
                    * ((((next_base_row[31]) * (BFieldElement::new(2)))
                        + (current_base_row[39]))
                        - (current_base_row[31]))))
                + ((node_821)
                    * ((current_base_row[27]) - (current_base_row[22]))))
                + ((node_823) * (node_393)))
                + ((node_825) * (node_393)))
                + ((node_827) * (node_393)))
                + ((node_830) * (node_392)))
                + ((node_833) * (node_392)))
                + ((node_835) * (node_392)))
                + ((node_838) * ((node_664) * (node_666))))
                + ((node_845)
                    * ((next_base_row[21])
                        * (((current_base_row[39])
                            * ((next_base_row[22])
                                - (BFieldElement::new(4294967295))))
                            - (BFieldElement::new(1))))))
                + ((node_850) * (node_393)))
                + ((node_854) * (node_393)))
                + ((node_858) * (node_393)))
                + ((node_861) * (node_393)))
                + ((node_864) * (node_393)))
                + ((node_867) * (node_553)))
                + ((node_870) * (node_393)))
                + ((node_872)
                    * ((next_base_row[22])
                        - ((current_base_row[22]) + (current_base_row[25])))))
                + ((node_874)
                    * ((next_base_row[22])
                        - ((((((current_base_row[22]) * (current_base_row[24]))
                            + ((current_base_row[21]) * (current_base_row[25])))
                            - (node_698))
                            + (node_690))
                            + (node_692)))))
                + ((node_876)
                    * ((((((current_base_row[22]) * (next_base_row[21]))
                        + ((current_base_row[21]) * (next_base_row[22])))
                        - (node_718))
                        + (node_710))
                        + (node_712))))
                + ((node_879)
                    * ((next_base_row[22])
                        - ((current_base_row[21]) * (current_base_row[23])))))
                + ((node_881) * (node_393)))
                + ((node_883) * (node_393)))
                * (node_3965))
                + ((node_620) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_394))
                + ((node_757) * (node_422)))
                + ((node_765) * (node_394)))
                + ((node_770) * (node_454)))
                + ((node_775)
                    * ((node_452)
                        * ((next_base_row[23])
                            - (current_base_row[21])))))
                + ((node_780) * (node_394)))
                + ((node_785)
                    * ((((node_392) * (current_base_row[21]))
                        + (((node_439) * (node_600))
                            * (node_503)))
                        + ((((node_391)
                            - (BFieldElement::new(3)))
                            * (node_600))
                            * (current_base_row[39])))))
                + ((node_789)
                    * ((next_base_row[20])
                        - (current_base_row[9]))))
                + ((node_793) * (node_447)))
                + ((node_798) * (node_394)))
                + ((node_802) * (node_393)))
                + ((node_805) * (node_393)))
                + ((node_808) * (node_392)))
                + ((node_812) * (node_392)))
                + ((node_815) * (node_394)))
                + ((node_818)
                    * (((node_445)
                        * ((current_base_row[26]) - (next_base_row[21])))
                        + ((current_base_row[39])
                            * ((current_base_row[26])
                                - (next_base_row[26]))))))
                + ((node_821)
                    * ((current_base_row[28]) - (current_base_row[23]))))
                + ((node_823) * (node_394)))
                + ((node_825) * (node_394)))
                + ((node_827) * (node_394)))
                + ((node_830) * (node_393)))
                + ((node_833) * (node_393)))
                + ((node_835) * (node_393)))
                + ((node_838)
                    * ((next_base_row[21])
                        - ((BFieldElement::new(1)) - (node_665)))))
                + ((node_845) * (node_422)))
                + ((node_850) * (node_394)))
                + ((node_854) * (node_394)))
                + ((node_858) * (node_394)))
                + ((node_861) * (node_394)))
                + ((node_864) * (node_394)))
                + ((node_867) * (node_392)))
                + ((node_870) * (node_394)))
                + ((node_872)
                    * ((next_base_row[23])
                        - ((current_base_row[23]) + (current_base_row[26])))))
                + ((node_874)
                    * ((next_base_row[23])
                        - (((((current_base_row[23]) * (current_base_row[24]))
                            + ((current_base_row[22]) * (current_base_row[25])))
                            + ((current_base_row[21]) * (current_base_row[26])))
                            + (node_698)))))
                + ((node_876)
                    * (((((current_base_row[23]) * (next_base_row[21])) + (node_680))
                        + ((current_base_row[21]) * (next_base_row[23])))
                        + (node_718))))
                + ((node_879) * ((next_base_row[23]) - (node_689))))
                + ((node_881) * (node_394)))
                + ((node_883) * (node_394)))
                * (node_3965))
                + (((next_base_row[9]) - (current_base_row[9])) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_395))
                + ((node_757) * (node_423)))
                + ((node_765) * (node_395)))
                + ((node_770) * (node_457)))
                + ((node_775)
                    * ((node_455)
                        * ((next_base_row[24])
                            - (current_base_row[21])))))
                + ((node_780) * (node_395)))
                + ((node_785) * (node_393)))
                + ((node_789)
                    * ((next_base_row[7]) - (current_base_row[9]))))
                + ((node_793) * (node_550)))
                + ((node_798) * (node_395)))
                + ((node_802) * (node_394)))
                + ((node_805) * (node_394)))
                + ((node_808) * (node_393)))
                + ((node_812) * (node_393)))
                + ((node_815) * (node_395)))
                + ((node_818)
                    * (((node_445)
                        * ((current_base_row[27]) - (next_base_row[22])))
                        + ((current_base_row[39])
                            * ((current_base_row[27])
                                - (next_base_row[27]))))))
                + ((node_821)
                    * ((current_base_row[29]) - (current_base_row[24]))))
                + ((node_823) * (node_395)))
                + ((node_825) * (node_395)))
                + ((node_827) * (node_395)))
                + ((node_830) * (node_394)))
                + ((node_833) * (node_394)))
                + ((node_835) * (node_394)))
                + ((node_838) * (node_392)))
                + ((node_845) * (node_423)))
                + ((node_850) * (node_395)))
                + ((node_854) * (node_395)))
                + ((node_858) * (node_395)))
                + ((node_861) * (node_395)))
                + ((node_864) * (node_395)))
                + ((node_867) * (node_393)))
                + ((node_870) * (node_395)))
                + ((node_872) * (node_556)))
                + ((node_874) * (node_556)))
                + ((node_876) * (node_556)))
                + ((node_879) * (node_399)))
                + ((node_881) * (node_395)))
                + ((node_883) * (node_395)))
                * (node_3965))
                + ((node_393) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_396))
                + ((node_757) * (node_424)))
                + ((node_765) * (node_421)))
                + ((node_770) * (node_462)))
                + ((node_775)
                    * ((node_460)
                        * ((next_base_row[25])
                            - (current_base_row[21])))))
                + ((node_780) * (node_447)))
                + ((node_785) * (node_394)))
                + ((node_789) * (node_447)))
                + ((node_793) * (node_553)))
                + ((node_798) * (node_447)))
                + ((node_802) * (node_395)))
                + ((node_805) * (node_395)))
                + ((node_808) * (node_394)))
                + ((node_812) * (node_394)))
                + ((node_815) * (node_577)))
                + ((node_818)
                    * (((node_445)
                        * ((current_base_row[28]) - (next_base_row[23])))
                        + ((current_base_row[39])
                            * ((current_base_row[28])
                                - (next_base_row[28]))))))
                + ((node_821)
                    * ((current_base_row[30]) - (current_base_row[25]))))
                + ((node_823) * (node_447)))
                + ((node_825) * (node_447)))
                + ((node_827) * (node_577)))
                + ((node_830) * (node_395)))
                + ((node_833) * (node_395)))
                + ((node_835) * (node_395)))
                + ((node_838) * (node_393)))
                + ((node_845) * (node_424)))
                + ((node_850) * (node_397)))
                + ((node_854) * (node_397)))
                + ((node_858) * (node_397)))
                + ((node_861) * (node_550)))
                + ((node_864) * (node_397)))
                + ((node_867) * (node_394)))
                + ((node_870) * (node_550)))
                + ((node_872) * (node_559)))
                + ((node_874) * (node_559)))
                + ((node_876) * (node_559)))
                + ((node_879) * (node_400)))
                + ((node_881) * (node_421)))
                + ((node_883) * (node_396)))
                * (node_3965))
                + ((node_394) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_397))
                + ((node_757) * (node_425)))
                + ((node_765) * (node_422)))
                + ((node_770) * (node_465)))
                + ((node_775)
                    * ((node_463)
                        * ((next_base_row[26])
                            - (current_base_row[21])))))
                + ((node_780) * (node_550)))
                + ((node_785) * (node_395)))
                + ((node_789) * (node_550)))
                + ((node_793) * (node_556)))
                + ((node_798) * (node_550)))
                + ((node_802) * (node_396)))
                + ((node_805) * (node_447)))
                + ((node_808) * (node_395)))
                + ((node_812) * (node_395)))
                + ((node_815) * (node_580)))
                + ((node_818)
                    * (((node_445)
                        * ((current_base_row[29]) - (next_base_row[24])))
                        + ((current_base_row[39])
                            * ((current_base_row[29])
                                - (next_base_row[29]))))))
                + ((node_821) * (node_392)))
                + ((node_823) * (node_550)))
                + ((node_825) * (node_550)))
                + ((node_827) * (node_580)))
                + ((node_830) * (node_397)))
                + ((node_833) * (node_397)))
                + ((node_835) * (node_550)))
                + ((node_838) * (node_394)))
                + ((node_845) * (node_425)))
                + ((node_850) * (node_398)))
                + ((node_854) * (node_398)))
                + ((node_858) * (node_398)))
                + ((node_861) * (node_553)))
                + ((node_864) * (node_398)))
                + ((node_867) * (node_395)))
                + ((node_870) * (node_553)))
                + ((node_872) * (node_562)))
                + ((node_874) * (node_562)))
                + ((node_876) * (node_562)))
                + ((node_879) * (node_401)))
                + ((node_881) * (node_422)))
                + ((node_883) * (node_397)))
                * (node_3965))
                + ((node_395) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_398))
                + ((node_757) * (node_426)))
                + ((node_765) * (node_423)))
                + ((node_770) * (node_469)))
                + ((node_775)
                    * ((node_467)
                        * ((next_base_row[27])
                            - (current_base_row[21])))))
                + ((node_780) * (node_553)))
                + ((node_785) * (node_396)))
                + ((node_789) * (node_553)))
                + ((node_793) * (node_559)))
                + ((node_798) * (node_553)))
                + ((node_802) * (node_397)))
                + ((node_805) * (node_550)))
                + ((node_808) * (node_421)))
                + ((node_812) * (node_396)))
                + ((node_815) * (node_583)))
                + ((node_818)
                    * (((node_445)
                        * ((current_base_row[30]) - (next_base_row[25])))
                        + ((current_base_row[39])
                            * ((current_base_row[30])
                                - (next_base_row[30]))))))
                + ((node_821) * (node_393)))
                + ((node_823) * (node_553)))
                + ((node_825) * (node_553)))
                + ((node_827) * (node_583)))
                + ((node_830) * (node_398)))
                + ((node_833) * (node_398)))
                + ((node_835) * (node_553)))
                + ((node_838) * (node_395)))
                + ((node_845) * (node_426)))
                + ((node_850) * (node_399)))
                + ((node_854) * (node_399)))
                + ((node_858) * (node_399)))
                + ((node_861) * (node_556)))
                + ((node_864) * (node_399)))
                + ((node_867) * (node_556)))
                + ((node_870) * (node_556)))
                + ((node_872) * (node_565)))
                + ((node_874) * (node_565)))
                + ((node_876) * (node_565)))
                + ((node_879) * (node_402)))
                + ((node_881) * (node_423)))
                + ((node_883) * (node_398)))
                * (node_3965))
                + ((node_447) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_399))
                + ((node_757) * (node_427)))
                + ((node_765) * (node_424)))
                + ((node_770) * (node_472)))
                + ((node_775)
                    * ((node_470)
                        * ((next_base_row[28])
                            - (current_base_row[21])))))
                + ((node_780) * (node_556)))
                + ((node_785) * (node_397)))
                + ((node_789) * (node_556)))
                + ((node_793) * (node_562)))
                + ((node_798) * (node_556)))
                + ((node_802) * (node_398)))
                + ((node_805) * (node_553)))
                + ((node_808) * (node_422)))
                + ((node_812) * (node_397)))
                + ((node_815) * (node_586)))
                + ((node_818) * (node_580)))
                + ((node_821) * (node_394)))
                + ((node_823) * (node_556)))
                + ((node_825) * (node_556)))
                + ((node_827) * (node_586)))
                + ((node_830) * (node_399)))
                + ((node_833) * (node_399)))
                + ((node_835) * (node_556)))
                + ((node_838) * (node_397)))
                + ((node_845) * (node_427)))
                + ((node_850) * (node_400)))
                + ((node_854) * (node_400)))
                + ((node_858) * (node_400)))
                + ((node_861) * (node_559)))
                + ((node_864) * (node_400)))
                + ((node_867) * (node_559)))
                + ((node_870) * (node_559)))
                + ((node_872) * (node_568)))
                + ((node_874) * (node_568)))
                + ((node_876) * (node_568)))
                + ((node_879) * (node_403)))
                + ((node_881) * (node_424)))
                + ((node_883) * (node_399)))
                * (node_3965))
                + ((node_550) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_400))
                + ((node_757) * (node_428)))
                + ((node_765) * (node_425)))
                + ((node_770) * (node_477)))
                + ((node_775)
                    * ((node_475)
                        * ((next_base_row[29])
                            - (current_base_row[21])))))
                + ((node_780) * (node_559)))
                + ((node_785) * (node_398)))
                + ((node_789) * (node_559)))
                + ((node_793) * (node_565)))
                + ((node_798) * (node_559)))
                + ((node_802) * (node_399)))
                + ((node_805) * (node_556)))
                + ((node_808) * (node_423)))
                + ((node_812) * (node_398)))
                + ((node_815) * (node_589)))
                + ((node_818) * (node_583)))
                + ((node_821) * (node_395)))
                + ((node_823) * (node_559)))
                + ((node_825) * (node_559)))
                + ((node_827) * (node_589)))
                + ((node_830) * (node_400)))
                + ((node_833) * (node_400)))
                + ((node_835) * (node_559)))
                + ((node_838) * (node_398)))
                + ((node_845) * (node_428)))
                + ((node_850) * (node_401)))
                + ((node_854) * (node_401)))
                + ((node_858) * (node_401)))
                + ((node_861) * (node_562)))
                + ((node_864) * (node_401)))
                + ((node_867) * (node_562)))
                + ((node_870) * (node_562)))
                + ((node_872) * (node_571)))
                + ((node_874) * (node_571)))
                + ((node_876) * (node_571)))
                + ((node_879) * (node_404)))
                + ((node_881) * (node_425)))
                + ((node_883) * (node_400)))
                * (node_3965))
                + ((node_553) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_401))
                + ((node_757) * (node_429)))
                + ((node_765) * (node_426)))
                + ((node_770) * (node_480)))
                + ((node_775)
                    * ((node_478)
                        * ((next_base_row[30])
                            - (current_base_row[21])))))
                + ((node_780) * (node_562)))
                + ((node_785) * (node_399)))
                + ((node_789) * (node_562)))
                + ((node_793) * (node_568)))
                + ((node_798) * (node_562)))
                + ((node_802) * (node_400)))
                + ((node_805) * (node_559)))
                + ((node_808) * (node_424)))
                + ((node_812) * (node_399)))
                + ((node_815) * (node_592)))
                + ((node_818) * (node_586)))
                + ((node_821) * (node_447)))
                + ((node_823) * (node_562)))
                + ((node_825) * (node_562)))
                + ((node_827) * (node_592)))
                + ((node_830) * (node_401)))
                + ((node_833) * (node_401)))
                + ((node_835) * (node_562)))
                + ((node_838) * (node_399)))
                + ((node_845) * (node_429)))
                + ((node_850) * (node_402)))
                + ((node_854) * (node_402)))
                + ((node_858) * (node_402)))
                + ((node_861) * (node_565)))
                + ((node_864) * (node_402)))
                + ((node_867) * (node_565)))
                + ((node_870) * (node_565)))
                + ((node_872) * (node_574)))
                + ((node_874) * (node_574)))
                + ((node_876) * (node_574)))
                + ((node_879) * (node_405)))
                + ((node_881) * (node_426)))
                + ((node_883) * (node_401)))
                * (node_3965))
                + ((node_556) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_402))
                + ((node_757) * (node_430)))
                + ((node_765) * (node_427)))
                + ((node_770) * (node_484)))
                + ((node_775)
                    * ((node_482)
                        * ((next_base_row[31])
                            - (current_base_row[21])))))
                + ((node_780) * (node_565)))
                + ((node_785) * (node_400)))
                + ((node_789) * (node_565)))
                + ((node_793) * (node_571)))
                + ((node_798) * (node_565)))
                + ((node_802) * (node_401)))
                + ((node_805) * (node_562)))
                + ((node_808) * (node_425)))
                + ((node_812) * (node_400)))
                + ((node_815) * (node_594)))
                + ((node_818) * (node_589)))
                + ((node_821) * (node_550)))
                + ((node_823) * (node_565)))
                + ((node_825) * (node_565)))
                + ((node_827) * (node_594)))
                + ((node_830) * (node_402)))
                + ((node_833) * (node_402)))
                + ((node_835) * (node_565)))
                + ((node_838) * (node_400)))
                + ((node_845) * (node_430)))
                + ((node_850) * (node_403)))
                + ((node_854) * (node_403)))
                + ((node_858) * (node_403)))
                + ((node_861) * (node_568)))
                + ((node_864) * (node_403)))
                + ((node_867) * (node_568)))
                + ((node_870) * (node_568)))
                + ((node_872) * (node_577)))
                + ((node_874) * (node_577)))
                + ((node_876) * (node_577)))
                + ((node_879) * (node_406)))
                + ((node_881) * (node_427)))
                + ((node_883) * (node_402)))
                * (node_3965))
                + ((node_559) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_403))
                + ((node_757) * (node_431)))
                + ((node_765) * (node_428)))
                + ((node_770) * (node_487)))
                + ((node_775)
                    * ((node_485)
                        * ((next_base_row[32])
                            - (current_base_row[21])))))
                + ((node_780) * (node_568)))
                + ((node_785) * (node_401)))
                + ((node_789) * (node_568)))
                + ((node_793) * (node_574)))
                + ((node_798) * (node_568)))
                + ((node_802) * (node_402)))
                + ((node_805) * (node_565)))
                + ((node_808) * (node_426)))
                + ((node_812) * (node_401)))
                + ((node_815) * (node_595)))
                + ((node_818) * (node_592)))
                + ((node_821) * (node_553)))
                + ((node_823) * (node_568)))
                + ((node_825) * (node_568)))
                + ((node_827) * (node_595)))
                + ((node_830) * (node_403)))
                + ((node_833) * (node_403)))
                + ((node_835) * (node_568)))
                + ((node_838) * (node_401)))
                + ((node_845) * (node_431)))
                + ((node_850) * (node_404)))
                + ((node_854) * (node_404)))
                + ((node_858) * (node_404)))
                + ((node_861) * (node_571)))
                + ((node_864) * (node_404)))
                + ((node_867) * (node_571)))
                + ((node_870) * (node_571)))
                + ((node_872) * (node_580)))
                + ((node_874) * (node_580)))
                + ((node_876) * (node_580)))
                + ((node_879) * (node_407)))
                + ((node_881) * (node_428)))
                + ((node_883) * (node_403)))
                * (node_3965))
                + ((node_562) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_404))
                + ((node_757) * (node_432)))
                + ((node_765) * (node_429)))
                + ((node_770) * (node_492)))
                + ((node_775)
                    * ((node_490)
                        * ((next_base_row[33])
                            - (current_base_row[21])))))
                + ((node_780) * (node_571)))
                + ((node_785) * (node_402)))
                + ((node_789) * (node_571)))
                + ((node_793) * (node_577)))
                + ((node_798) * (node_571)))
                + ((node_802) * (node_403)))
                + ((node_805) * (node_568)))
                + ((node_808) * (node_427)))
                + ((node_812) * (node_402)))
                + ((node_815) * (node_418)))
                + ((node_818) * (node_594)))
                + ((node_821) * (node_556)))
                + ((node_823) * (node_571)))
                + ((node_825) * (node_571)))
                + ((node_827) * (node_418)))
                + ((node_830) * (node_404)))
                + ((node_833) * (node_404)))
                + ((node_835) * (node_571)))
                + ((node_838) * (node_402)))
                + ((node_845) * (node_432)))
                + ((node_850) * (node_405)))
                + ((node_854) * (node_405)))
                + ((node_858) * (node_405)))
                + ((node_861) * (node_574)))
                + ((node_864) * (node_405)))
                + ((node_867) * (node_574)))
                + ((node_870) * (node_574)))
                + ((node_872) * (node_583)))
                + ((node_874) * (node_583)))
                + ((node_876) * (node_583)))
                + ((node_879) * (node_408)))
                + ((node_881) * (node_429)))
                + ((node_883) * (node_404)))
                * (node_3965))
                + ((node_565) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((((node_747)
                * (node_405))
                + ((node_757) * (node_433)))
                + ((node_765) * (node_430)))
                + ((node_770) * (node_495)))
                + ((node_775)
                    * ((node_493)
                        * ((next_base_row[34])
                            - (current_base_row[21])))))
                + ((node_780) * (node_574)))
                + ((node_785) * (node_403)))
                + ((node_789) * (node_574)))
                + ((node_793) * (node_580)))
                + ((node_798) * (node_574)))
                + ((node_802) * (node_404)))
                + ((node_805) * (node_571)))
                + ((node_808) * (node_428)))
                + ((node_812) * (node_403)))
                + ((node_815) * (node_419)))
                + ((node_818) * (node_595)))
                + ((node_821) * (node_559)))
                + ((node_823) * (node_574)))
                + ((node_825) * (node_574)))
                + ((node_827) * (node_419)))
                + ((node_830) * (node_405)))
                + ((node_833) * (node_405)))
                + ((node_835) * (node_574)))
                + ((node_838) * (node_403)))
                + ((node_845) * (node_433)))
                + ((node_850) * (node_406)))
                + ((node_854) * (node_406)))
                + ((node_858) * (node_406)))
                + ((node_861) * (node_577)))
                + ((node_864) * (node_406)))
                + ((node_867) * (node_577)))
                + ((node_870) * (node_577)))
                + ((node_872) * (node_586)))
                + ((node_874) * (node_586)))
                + ((node_876) * (node_586)))
                + ((node_879) * (node_409)))
                + ((node_881) * (node_430)))
                + ((node_883) * (node_405)))
                * (node_3965))
                + ((node_568) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((node_747)
                * (node_406))
                + ((node_757) * (node_434)))
                + ((node_765) * (node_431)))
                + ((node_770) * (node_499)))
                + ((node_775)
                    * ((node_497)
                        * ((next_base_row[35])
                            - (current_base_row[21])))))
                + ((node_780) * (node_577)))
                + ((node_785) * (node_404)))
                + ((node_789) * (node_577)))
                + ((node_793) * (node_583)))
                + ((node_798) * (node_577)))
                + ((node_802) * (node_405)))
                + ((node_805) * (node_574)))
                + ((node_808) * (node_429)))
                + ((node_812) * (node_404)))
                + ((node_818) * (node_392)))
                + ((node_821) * (node_562)))
                + ((node_823) * (node_577)))
                + ((node_825) * (node_577)))
                + ((node_830) * (node_406)))
                + ((node_833) * (node_406)))
                + ((node_835) * (node_577)))
                + ((node_838) * (node_404)))
                + ((node_845) * (node_434)))
                + ((node_850) * (node_407)))
                + ((node_854) * (node_407)))
                + ((node_858) * (node_407)))
                + ((node_861) * (node_580)))
                + ((node_864) * (node_407)))
                + ((node_867) * (node_580)))
                + ((node_870) * (node_580)))
                + ((node_872) * (node_589)))
                + ((node_874) * (node_589)))
                + ((node_876) * (node_589)))
                + ((node_879) * (node_410)))
                + ((node_881) * (node_431)))
                + ((node_883) * (node_406)))
                * (node_3965))
                + ((node_571) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((node_747)
                * (node_407))
                + ((node_757) * (node_435)))
                + ((node_765) * (node_432)))
                + ((node_770) * (node_502)))
                + ((node_775)
                    * ((node_500)
                        * ((next_base_row[36])
                            - (current_base_row[21])))))
                + ((node_780) * (node_580)))
                + ((node_785) * (node_405)))
                + ((node_789) * (node_580)))
                + ((node_793) * (node_586)))
                + ((node_798) * (node_580)))
                + ((node_802) * (node_406)))
                + ((node_805) * (node_577)))
                + ((node_808) * (node_430)))
                + ((node_812) * (node_405)))
                + ((node_818) * (node_393)))
                + ((node_821) * (node_565)))
                + ((node_823) * (node_580)))
                + ((node_825) * (node_580)))
                + ((node_830) * (node_407)))
                + ((node_833) * (node_407)))
                + ((node_835) * (node_580)))
                + ((node_838) * (node_405)))
                + ((node_845) * (node_435)))
                + ((node_850) * (node_408)))
                + ((node_854) * (node_408)))
                + ((node_858) * (node_408)))
                + ((node_861) * (node_583)))
                + ((node_864) * (node_408)))
                + ((node_867) * (node_583)))
                + ((node_870) * (node_583)))
                + ((node_872) * (node_592)))
                + ((node_874) * (node_592)))
                + ((node_876) * (node_592)))
                + ((node_879) * (node_411)))
                + ((node_881) * (node_432)))
                + ((node_883) * (node_407)))
                * (node_3965))
                + ((node_574) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((node_747)
                * (node_408))
                + ((node_757) * (node_436)))
                + ((node_765) * (node_433)))
                + ((node_770) * (node_504)))
                + ((node_775) * (node_450)))
                + ((node_780) * (node_583)))
                + ((node_785) * (node_406)))
                + ((node_789) * (node_583)))
                + ((node_793) * (node_589)))
                + ((node_798) * (node_583)))
                + ((node_802) * (node_407)))
                + ((node_805) * (node_580)))
                + ((node_808) * (node_431)))
                + ((node_812) * (node_406)))
                + ((node_818) * (node_394)))
                + ((node_821) * (node_568)))
                + ((node_823) * (node_583)))
                + ((node_825) * (node_583)))
                + ((node_830) * (node_408)))
                + ((node_833) * (node_408)))
                + ((node_835) * (node_583)))
                + ((node_838) * (node_406)))
                + ((node_845) * (node_436)))
                + ((node_850) * (node_409)))
                + ((node_854) * (node_409)))
                + ((node_858) * (node_409)))
                + ((node_861) * (node_586)))
                + ((node_864) * (node_409)))
                + ((node_867) * (node_586)))
                + ((node_870) * (node_586)))
                + ((node_872) * (node_594)))
                + ((node_874) * (node_594)))
                + ((node_876) * (node_594)))
                + ((node_879) * (node_413)))
                + ((node_881) * (node_433)))
                + ((node_883) * (node_408)))
                * (node_3965))
                + ((node_577) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((node_747)
                * (node_409))
                + ((node_757) * (node_438)))
                + ((node_765) * (node_434)))
                + ((node_770) * (node_506)))
                + ((node_775) * (node_454)))
                + ((node_780) * (node_586)))
                + ((node_785) * (node_407)))
                + ((node_789) * (node_586)))
                + ((node_793) * (node_592)))
                + ((node_798) * (node_586)))
                + ((node_802) * (node_408)))
                + ((node_805) * (node_583)))
                + ((node_808) * (node_432)))
                + ((node_812) * (node_407)))
                + ((node_818) * (node_395)))
                + ((node_821) * (node_571)))
                + ((node_823) * (node_586)))
                + ((node_825) * (node_586)))
                + ((node_830) * (node_409)))
                + ((node_833) * (node_409)))
                + ((node_835) * (node_586)))
                + ((node_838) * (node_407)))
                + ((node_845) * (node_438)))
                + ((node_850) * (node_410)))
                + ((node_854) * (node_410)))
                + ((node_858) * (node_410)))
                + ((node_861) * (node_589)))
                + ((node_864) * (node_410)))
                + ((node_867) * (node_589)))
                + ((node_870) * (node_589)))
                + ((node_872) * (node_595)))
                + ((node_874) * (node_595)))
                + ((node_876) * (node_595)))
                + ((node_879) * (node_417)))
                + ((node_881) * (node_434)))
                + ((node_883) * (node_409)))
                * (node_3965))
                + ((node_580) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((node_747)
                * (node_410))
                + ((node_757) * (node_439)))
                + ((node_765) * (node_435)))
                + ((node_770) * (node_508)))
                + ((node_775) * (node_457)))
                + ((node_780) * (node_589)))
                + ((node_785) * (node_408)))
                + ((node_789) * (node_589)))
                + ((node_793) * (node_594)))
                + ((node_798) * (node_589)))
                + ((node_802) * (node_409)))
                + ((node_805) * (node_586)))
                + ((node_808) * (node_433)))
                + ((node_812) * (node_408)))
                + ((node_818) * (node_418)))
                + ((node_821) * (node_574)))
                + ((node_823) * (node_589)))
                + ((node_825) * (node_589)))
                + ((node_830) * (node_410)))
                + ((node_833) * (node_410)))
                + ((node_835) * (node_589)))
                + ((node_838) * (node_408)))
                + ((node_845) * (node_392)))
                + ((node_850) * (node_411)))
                + ((node_854) * (node_411)))
                + ((node_858) * (node_411)))
                + ((node_861) * (node_592)))
                + ((node_864) * (node_411)))
                + ((node_867) * (node_592)))
                + ((node_870) * (node_592)))
                + ((node_872) * (node_392)))
                + ((node_874) * (node_392)))
                + ((node_876) * (node_392)))
                + ((node_879) * (node_392)))
                + ((node_881) * (node_435)))
                + ((node_883) * (node_410)))
                * (node_3965))
                + ((node_583) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((((node_747)
                * (node_411))
                + ((node_757) * (node_393)))
                + ((node_765) * (node_436)))
                + ((node_770) * (node_510)))
                + ((node_775) * (node_462)))
                + ((node_780) * (node_592)))
                + ((node_785) * (node_409)))
                + ((node_789) * (node_592)))
                + ((node_793) * (node_595)))
                + ((node_798) * (node_592)))
                + ((node_802) * (node_410)))
                + ((node_805) * (node_589)))
                + ((node_808) * (node_434)))
                + ((node_812) * (node_409)))
                + ((node_818) * (node_419)))
                + ((node_821) * (node_577)))
                + ((node_823) * (node_592)))
                + ((node_825) * (node_592)))
                + ((node_830) * (node_411)))
                + ((node_833) * (node_411)))
                + ((node_835) * (node_592)))
                + ((node_838) * (node_409)))
                + ((node_845) * (node_393)))
                + ((node_850) * (node_413)))
                + ((node_854) * (node_413)))
                + ((node_858) * (node_413)))
                + ((node_861) * (node_594)))
                + ((node_864) * (node_413)))
                + ((node_867) * (node_594)))
                + ((node_870) * (node_594)))
                + ((node_872) * (node_393)))
                + ((node_874) * (node_393)))
                + ((node_876) * (node_393)))
                + ((node_879) * (node_393)))
                + ((node_881) * (node_436)))
                + ((node_883) * (node_411)))
                * (node_3965))
                + ((node_586) * (next_base_row[5])),
            (((((((((((((((((((((((((((((((((((((node_747)
                * (node_413))
                + ((node_757) * (node_394)))
                + ((node_765) * (node_438)))
                + ((node_770) * (node_519)))
                + ((node_775) * (node_465)))
                + ((node_780) * (node_594)))
                + ((node_785) * (node_410)))
                + ((node_789) * (node_594)))
                + ((node_793) * (node_418)))
                + ((node_798) * (node_594)))
                + ((node_802) * (node_411)))
                + ((node_805) * (node_592)))
                + ((node_808) * (node_435)))
                + ((node_812) * (node_410)))
                + ((node_821) * (node_580)))
                + ((node_823) * (node_594)))
                + ((node_825) * (node_594)))
                + ((node_830) * (node_413)))
                + ((node_833) * (node_413)))
                + ((node_835) * (node_594)))
                + ((node_838) * (node_410)))
                + ((node_845) * (node_394)))
                + ((node_850) * (node_417)))
                + ((node_854) * (node_417)))
                + ((node_858) * (node_417)))
                + ((node_861) * (node_595)))
                + ((node_864) * (node_417)))
                + ((node_867) * (node_595)))
                + ((node_870) * (node_595)))
                + ((node_872) * (node_394)))
                + ((node_874) * (node_394)))
                + ((node_876) * (node_394)))
                + ((node_879) * (node_394)))
                + ((node_881) * (node_438)))
                + ((node_883) * (node_413)))
                * (node_3965))
                + ((node_589) * (next_base_row[5])),
            (((((((((((((((((((((((((((((((((((((node_747)
                * (node_417))
                + ((node_757) * (node_395)))
                + ((node_765) * (node_418)))
                + ((node_770) * (node_439)))
                + ((node_775) * (node_469)))
                + ((node_780) * (node_595)))
                + ((node_785) * (node_411)))
                + ((node_789) * (node_595)))
                + ((node_793) * (node_419)))
                + ((node_798) * (node_595)))
                + ((node_802) * (node_413)))
                + ((node_805) * (node_594)))
                + ((node_808) * (node_436)))
                + ((node_812) * (node_411)))
                + ((node_821) * (node_583)))
                + ((node_823) * (node_595)))
                + ((node_825) * (node_595)))
                + ((node_830) * (node_417)))
                + ((node_833) * (node_417)))
                + ((node_835) * (node_595)))
                + ((node_838) * (node_411)))
                + ((node_845) * (node_395)))
                + ((node_850) * (node_418)))
                + ((node_854) * (node_418)))
                + ((node_858) * (node_418)))
                + ((node_861) * (node_418)))
                + ((node_864) * (node_418)))
                + ((node_867) * (node_418)))
                + ((node_870) * (node_418)))
                + ((node_872) * (node_395)))
                + ((node_874) * (node_395)))
                + ((node_876) * (node_395)))
                + ((node_879) * (node_395)))
                + ((node_881) * (node_418)))
                + ((node_883) * (node_417)))
                * (node_3965))
                + ((node_592) * (next_base_row[5])),
            ((((((((((((((((((((((((((((((((((((node_747)
                * (node_418))
                + ((node_757) * (node_418)))
                + ((node_765) * (node_419)))
                + ((node_770) * (node_393)))
                + ((node_775) * (node_472)))
                + ((node_780) * (node_418)))
                + ((node_785) * (node_413)))
                + ((node_789) * (node_418)))
                + ((node_798) * (node_418)))
                + ((node_802) * (node_417)))
                + ((node_805) * (node_595)))
                + ((node_808) * (node_438)))
                + ((node_812) * (node_413)))
                + ((node_821) * (node_586)))
                + ((node_823) * (node_418)))
                + ((node_825) * (node_418)))
                + ((node_830) * (node_418)))
                + ((node_833) * (node_418)))
                + ((node_835) * (node_418)))
                + ((node_838) * (node_413)))
                + ((node_845) * (node_418)))
                + ((node_850) * (node_419)))
                + ((node_854) * (node_419)))
                + ((node_858) * (node_419)))
                + ((node_861) * (node_419)))
                + ((node_864) * (node_419)))
                + ((node_867) * (node_419)))
                + ((node_870) * (node_419)))
                + ((node_872) * (node_418)))
                + ((node_874) * (node_418)))
                + ((node_876) * (node_418)))
                + ((node_879) * (node_418)))
                + ((node_881) * (node_419)))
                + ((node_883) * (node_418)))
                * (node_3965))
                + ((node_594) * (next_base_row[5])),
            ((((((((((((((((((((((((((node_747) * (node_419))
                + ((node_757) * (node_419)))
                + ((node_770) * (node_394)))
                + ((node_775) * (node_477)))
                + ((node_780) * (node_419)))
                + ((node_785) * (node_417)))
                + ((node_789) * (node_419)))
                + ((node_798) * (node_419)))
                + ((node_802) * (node_418)))
                + ((node_805) * (node_418)))
                + ((node_812) * (node_417)))
                + ((node_821) * (node_589)))
                + ((node_823) * (node_419)))
                + ((node_825) * (node_419)))
                + ((node_830) * (node_419)))
                + ((node_833) * (node_419)))
                + ((node_835) * (node_419)))
                + ((node_838) * (node_417)))
                + ((node_845) * (node_419)))
                + ((node_872) * (node_419)))
                + ((node_874) * (node_419)))
                + ((node_876) * (node_419)))
                + ((node_879) * (node_419)))
                + ((node_883) * (node_419)))
                * (node_3965))
                + ((node_595) * (next_base_row[5])),
            (((((((((node_770) * (node_395)) + ((node_775) * (node_480)))
                + ((node_785) * (node_418)))
                + ((node_802) * (node_419)))
                + ((node_805) * (node_419)))
                + ((node_821) * (node_592)))
                + ((node_838) * (node_418)))
                * (node_3965))
                + ((node_418) * (next_base_row[5])),
            (((((((node_770) * (node_421)) + ((node_775) * (node_484)))
                + ((node_785) * (node_419)))
                + ((node_821) * (node_594)))
                + ((node_838) * (node_419)))
                * (node_3965))
                + ((node_419) * (next_base_row[5])),
            ((((node_770) * (node_422)) + ((node_775) * (node_487))) + ((node_821) * (node_595)))
                * (node_3965),
            ((((node_770) * (node_423)) + ((node_775) * (node_492))) + ((node_821) * (node_418)))
                * (node_3965),
            ((((node_770) * (node_424)) + ((node_775) * (node_495))) + ((node_821) * (node_419)))
                * (node_3965),
            (((node_770) * (node_425)) + ((node_775) * (node_499))) * (node_3965),
            (((node_770) * (node_426)) + ((node_775) * (node_502))) * (node_3965),
            (((node_770) * (node_427))
                + ((node_775) * (((BFieldElement::new(1)) - (node_449)) * (node_550))))
                * (node_3965),
            (((node_770) * (node_428))
                + ((node_775) * (((BFieldElement::new(1)) - (node_452)) * (node_553))))
                * (node_3965),
            (((node_770) * (node_429))
                + ((node_775) * (((BFieldElement::new(1)) - (node_455)) * (node_556))))
                * (node_3965),
            (((node_770) * (node_430))
                + ((node_775) * (((BFieldElement::new(1)) - (node_460)) * (node_559))))
                * (node_3965),
            (((node_770) * (node_431))
                + ((node_775) * (((BFieldElement::new(1)) - (node_463)) * (node_562))))
                * (node_3965),
            (((node_770) * (node_432))
                + ((node_775) * (((BFieldElement::new(1)) - (node_467)) * (node_565))))
                * (node_3965),
            (((node_770) * (node_433))
                + ((node_775) * (((BFieldElement::new(1)) - (node_470)) * (node_568))))
                * (node_3965),
            (((node_770) * (node_434))
                + ((node_775) * (((BFieldElement::new(1)) - (node_475)) * (node_571))))
                * (node_3965),
            (((node_770) * (node_435))
                + ((node_775) * (((BFieldElement::new(1)) - (node_478)) * (node_574))))
                * (node_3965),
            (((node_770) * (node_436))
                + ((node_775) * (((BFieldElement::new(1)) - (node_482)) * (node_577))))
                * (node_3965),
            (((node_770) * (node_438))
                + ((node_775) * (((BFieldElement::new(1)) - (node_485)) * (node_580))))
                * (node_3965),
            (((node_770) * (node_418))
                + ((node_775) * (((BFieldElement::new(1)) - (node_490)) * (node_583))))
                * (node_3965),
            (((node_770) * (node_419))
                + ((node_775) * (((BFieldElement::new(1)) - (node_493)) * (node_586))))
                * (node_3965),
            ((node_775) * (((BFieldElement::new(1)) - (node_497)) * (node_589))) * (node_3965),
            ((node_775) * (((BFieldElement::new(1)) - (node_500)) * (node_592))) * (node_3965),
            ((node_775) * (node_594)) * (node_3965),
            ((node_775) * (node_595)) * (node_3965),
            ((node_775) * (node_504)) * (node_3965),
            ((node_775) * (node_506)) * (node_3965),
            ((node_775) * (node_508)) * (node_3965),
            ((node_775) * (node_510)) * (node_3965),
            ((node_775) * (node_519)) * (node_3965),
            ((node_775) * (node_439)) * (node_3965),
            ((node_775) * (node_393)) * (node_3965),
            ((node_775) * (node_394)) * (node_3965),
            ((node_775) * (node_395)) * (node_3965),
            ((node_775) * (node_418)) * (node_3965),
            ((node_775) * (node_419)) * (node_3965),
        ];
        let ext_constraints = [
            (((next_ext_row[11]) - (current_ext_row[11]))
                * ((challenges.get_challenge(ClockJumpDifferenceLookupIndeterminate))
                    - (next_base_row[4])))
                - (next_base_row[45]),
            (((current_base_row[8]) - (BFieldElement::new(128)))
                * ((next_ext_row[1]) - (current_ext_row[1])))
                + ((node_881)
                    * (((next_ext_row[1])
                        - ((challenges.get_challenge(StandardInputIndeterminate))
                            * (current_ext_row[1])))
                        - (next_base_row[21]))),
            ((node_3965)
                * (((node_4141)
                    * ((challenges.get_challenge(InstructionLookupIndeterminate))
                        - ((((challenges.get_challenge(ProgramAddressWeight))
                            * (next_base_row[7]))
                            + ((challenges.get_challenge(ProgramInstructionWeight))
                                * (next_base_row[8])))
                            + ((challenges.get_challenge(ProgramNextInstructionWeight))
                                * (next_base_row[9])))))
                    - (BFieldElement::new(1))))
                + ((next_base_row[5]) * (node_4141)),
            (((next_base_row[8]) - (BFieldElement::new(66)))
                * ((next_ext_row[2]) - (current_ext_row[2])))
                + (((((((((node_4149) * (next_base_row[11])) * (node_4153)) * (node_4155))
                    * ((next_base_row[14]) - (BFieldElement::new(1))))
                    * ((next_base_row[15]) - (BFieldElement::new(1))))
                    * (next_base_row[16]))
                    * (node_4163))
                    * (((next_ext_row[2])
                        - ((challenges.get_challenge(StandardOutputIndeterminate))
                            * (current_ext_row[2])))
                        - (next_base_row[21]))),
            (next_ext_row[4])
                - ((current_ext_row[4])
                    * ((challenges.get_challenge(OpStackIndeterminate))
                        - (((((challenges.get_challenge(OpStackClkWeight))
                            * (next_base_row[4]))
                            + ((challenges.get_challenge(OpStackIb1Weight))
                                * (next_base_row[11])))
                            + ((challenges.get_challenge(OpStackOspWeight))
                                * (next_base_row[37])))
                            + ((challenges.get_challenge(OpStackOsvWeight))
                                * (next_base_row[38]))))),
            (next_ext_row[5])
                - ((current_ext_row[5])
                    * ((challenges.get_challenge(RamIndeterminate))
                        - (((((challenges.get_challenge(RamClkWeight)) * (next_base_row[4]))
                            + ((challenges.get_challenge(RamRampWeight))
                                * (next_base_row[43])))
                            + ((challenges.get_challenge(RamRamvWeight)) * (next_base_row[44])))
                            + ((challenges.get_challenge(RamPreviousInstructionWeight))
                                * (next_base_row[6]))))),
            (next_ext_row[6])
                - ((current_ext_row[6])
                    * ((challenges.get_challenge(JumpStackIndeterminate))
                        - ((((((challenges.get_challenge(JumpStackClkWeight))
                            * (next_base_row[4]))
                            + ((challenges.get_challenge(JumpStackCiWeight))
                                * (next_base_row[8])))
                            + ((challenges.get_challenge(JumpStackJspWeight))
                                * (next_base_row[18])))
                            + ((challenges.get_challenge(JumpStackJsoWeight))
                                * (next_base_row[19])))
                            + ((challenges.get_challenge(JumpStackJsdWeight))
                                * (next_base_row[20]))))),
            (((next_base_row[8]) - (BFieldElement::new(48)))
                * ((next_ext_row[7]) - (current_ext_row[7])))
                + (((((((((node_4149) * ((next_base_row[11]) - (BFieldElement::new(1))))
                    * (node_4153))
                    * (node_4155))
                    * (next_base_row[14]))
                    * (next_base_row[15]))
                    * ((next_base_row[16]) - (BFieldElement::new(1))))
                    * (node_4163))
                    * (((next_ext_row[7])
                        - ((challenges.get_challenge(HashInputIndeterminate))
                            * (current_ext_row[7])))
                        - (node_4264))),
            (((current_base_row[8]) - (BFieldElement::new(48)))
                * ((next_ext_row[8]) - (current_ext_row[8])))
                + ((node_815)
                    * (((next_ext_row[8])
                        - ((challenges.get_challenge(HashDigestIndeterminate))
                            * (current_ext_row[8])))
                        - ((((((challenges.get_challenge(HashStateWeight0))
                            * (next_base_row[26]))
                            + ((challenges.get_challenge(HashStateWeight1))
                                * (next_base_row[27])))
                            + ((challenges.get_challenge(HashStateWeight2))
                                * (next_base_row[28])))
                            + ((challenges.get_challenge(HashStateWeight3))
                                * (next_base_row[29])))
                            + ((challenges.get_challenge(HashStateWeight4))
                                * (next_base_row[30]))))),
            (((((((current_base_row[8]) - (BFieldElement::new(72)))
                * ((current_base_row[8]) - (BFieldElement::new(80))))
                * ((current_base_row[8]) - (BFieldElement::new(88))))
                * ((next_ext_row[9]) - (current_ext_row[9])))
                + ((node_823) * (node_4304)))
                + ((node_825) * (node_4304)))
                + ((node_827) * (node_4304)),
            (((((((((node_845)
                * (((node_4349) * (((node_4321) - (node_4322)) - (node_4324)))
                    - (BFieldElement::new(1))))
                + ((node_850) * (node_4354)))
                + ((node_854) * (node_4354)))
                + ((node_858)
                    * (((node_4349)
                        * (((node_4329)
                            - ((challenges.get_challenge(U32CiWeight))
                                * (BFieldElement::new(20))))
                            - (((challenges.get_challenge(U32ResultWeight))
                                * ((node_658) - (next_base_row[21])))
                                * (BFieldElement::new(9223372034707292161)))))
                        - (BFieldElement::new(1)))))
                + ((node_864) * (node_4354)))
                + ((node_861) * (node_4362)))
                + ((node_867)
                    * (((((node_4349) * (node_4345)) * (node_4348)) - (node_4345))
                        - (node_4348))))
                + ((node_870) * (node_4362)))
                + (((BFieldElement::new(1)) - (current_base_row[12])) * (node_4349)),
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
        let base_constraints = [base_row[8]];
        let ext_constraints = [];
        base_constraints
            .into_iter()
            .chain(ext_constraints.into_iter())
            .collect()
    }
}

impl Quotientable for ExtProcessorTable {
    fn num_initial_quotients() -> usize {
        37
    }

    fn num_consistency_quotients() -> usize {
        11
    }

    fn num_transition_quotients() -> usize {
        73
    }

    fn num_terminal_quotients() -> usize {
        1
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
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 9 as Degree - zerofier_degree,
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
        [
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
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
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 14 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 11 as Degree - zerofier_degree,
            interpolant_degree * 11 as Degree - zerofier_degree,
            interpolant_degree * 11 as Degree - zerofier_degree,
            interpolant_degree * 11 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 9 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 9 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 9 as Degree - zerofier_degree,
            interpolant_degree * 9 as Degree - zerofier_degree,
            interpolant_degree * 9 as Degree - zerofier_degree,
            interpolant_degree * 11 as Degree - zerofier_degree,
        ]
        .to_vec()
    }

    #[allow(unused_variables)]
    fn terminal_quotient_degree_bounds(interpolant_degree: Degree) -> Vec<Degree> {
        let zerofier_degree = 1 as Degree;
        [interpolant_degree * 1 as Degree - zerofier_degree].to_vec()
    }
}
