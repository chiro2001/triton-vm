use ndarray::ArrayView1;
use twenty_first::shared_math::b_field_element::BFieldElement;
use twenty_first::shared_math::mpolynomial::Degree;
use twenty_first::shared_math::x_field_element::XFieldElement;

use crate::table::challenges::ChallengeId::*;
use crate::table::challenges::Challenges;
use crate::table::extension_table::Evaluable;
use crate::table::extension_table::Quotientable;
use crate::table::hash_table::ExtHashTable;

// This file has been auto-generated. Any modifications _will_ be lost.
// To re-generate, execute:
// `cargo run --bin constraint-evaluation-generator`
impl Evaluable<BFieldElement> for ExtHashTable {
    #[inline]
    #[allow(unused_variables)]
    fn evaluate_initial_constraints(
        base_row: ArrayView1<BFieldElement>,
        ext_row: ArrayView1<XFieldElement>,
        challenges: &Challenges,
    ) -> Vec<XFieldElement> {
        let node_89 = (base_row[62]) + (BFieldElement::new(1));
        let node_94 = (base_row[63]) - (BFieldElement::new(72));
        let node_92 = (base_row[63]) - (BFieldElement::new(48));
        let node_88 = ((((((((((challenges.get_challenge(HashStateWeight0))
            * ((((((base_row[64]) * (BFieldElement::new(281474976710656)))
                + ((base_row[65]) * (BFieldElement::new(4294967296))))
                + ((base_row[66]) * (BFieldElement::new(65536))))
                + (base_row[67]))
                * (BFieldElement::new(18446744065119617025))))
            + ((challenges.get_challenge(HashStateWeight1))
                * ((((((base_row[68]) * (BFieldElement::new(281474976710656)))
                    + ((base_row[69]) * (BFieldElement::new(4294967296))))
                    + ((base_row[70]) * (BFieldElement::new(65536))))
                    + (base_row[71]))
                    * (BFieldElement::new(18446744065119617025)))))
            + ((challenges.get_challenge(HashStateWeight2))
                * ((((((base_row[72]) * (BFieldElement::new(281474976710656)))
                    + ((base_row[73]) * (BFieldElement::new(4294967296))))
                    + ((base_row[74]) * (BFieldElement::new(65536))))
                    + (base_row[75]))
                    * (BFieldElement::new(18446744065119617025)))))
            + ((challenges.get_challenge(HashStateWeight3))
                * ((((((base_row[76]) * (BFieldElement::new(281474976710656)))
                    + ((base_row[77]) * (BFieldElement::new(4294967296))))
                    + ((base_row[78]) * (BFieldElement::new(65536))))
                    + (base_row[79]))
                    * (BFieldElement::new(18446744065119617025)))))
            + ((challenges.get_challenge(HashStateWeight4)) * (base_row[96])))
            + ((challenges.get_challenge(HashStateWeight5)) * (base_row[97])))
            + ((challenges.get_challenge(HashStateWeight6)) * (base_row[98])))
            + ((challenges.get_challenge(HashStateWeight7)) * (base_row[99])))
            + ((challenges.get_challenge(HashStateWeight8)) * (base_row[100])))
            + ((challenges.get_challenge(HashStateWeight9)) * (base_row[101]));
        let node_97 = (ext_row[22]) - (BFieldElement::new(1));

        let base_constraints = [(node_89) * (base_row[62]), (node_94) * (node_92)];
        let ext_constraints = [
            ((((node_89) * (node_94))
                * (((ext_row[22]) - (challenges.get_challenge(HashInputIndeterminate)))
                    - (node_88)))
                + ((node_92) * (node_97)))
                + ((base_row[62]) * (node_97)),
            (ext_row[23]) - (BFieldElement::new(1)),
            ((node_92)
                * ((((ext_row[24]) - (challenges.get_challenge(SpongeIndeterminate)))
                    - ((challenges.get_challenge(HashCIWeight)) * (BFieldElement::new(72))))
                    - (node_88)))
                + ((node_94) * ((ext_row[24]) - (BFieldElement::new(1)))),
            ((node_89)
                * (((ext_row[25])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[64]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[80])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[25])),
            ((node_89)
                * (((ext_row[26])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[65]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[81])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[26])),
            ((node_89)
                * (((ext_row[27])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[66]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[82])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[27])),
            ((node_89)
                * (((ext_row[28])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[67]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[83])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[28])),
            ((node_89)
                * (((ext_row[29])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[68]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[84])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[29])),
            ((node_89)
                * (((ext_row[30])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[69]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[85])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[30])),
            ((node_89)
                * (((ext_row[31])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[70]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[86])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[31])),
            ((node_89)
                * (((ext_row[32])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[71]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[87])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[32])),
            ((node_89)
                * (((ext_row[33])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[72]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[88])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[33])),
            ((node_89)
                * (((ext_row[34])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[73]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[89])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[34])),
            ((node_89)
                * (((ext_row[35])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[74]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[90])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[35])),
            ((node_89)
                * (((ext_row[36])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[75]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[91])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[36])),
            ((node_89)
                * (((ext_row[37])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[76]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[92])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[37])),
            ((node_89)
                * (((ext_row[38])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[77]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[93])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[38])),
            ((node_89)
                * (((ext_row[39])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[78]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[94])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[39])),
            ((node_89)
                * (((ext_row[40])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[79]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[95])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[40])),
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
        let node_32 = (base_row[62]) - (BFieldElement::new(5));
        let node_34 = (base_row[62]) + (BFieldElement::new(1));
        let node_20 = (base_row[62]) - (BFieldElement::new(1));
        let node_23 = (base_row[62]) - (BFieldElement::new(2));
        let node_26 = (base_row[62]) - (BFieldElement::new(3));
        let node_29 = (base_row[62]) - (BFieldElement::new(4));
        let node_134 = (node_34) * (base_row[62]);
        let node_143 = (node_134) * (node_20);
        let node_151 = (node_143) * (node_23);
        let node_39 = (((((node_34) * (node_20)) * (node_23)) * (node_26)) * (node_29)) * (node_32);
        let node_138 = ((((node_134) * (node_23)) * (node_26)) * (node_29)) * (node_32);
        let node_146 = (((node_143) * (node_26)) * (node_29)) * (node_32);
        let node_153 = ((node_151) * (node_29)) * (node_32);
        let node_159 = ((node_151) * (node_26)) * (node_32);
        let node_13 = (base_row[63]) - (BFieldElement::new(80));
        let node_15 = (base_row[63]) - (BFieldElement::new(88));
        let node_9 = (base_row[63]) - (BFieldElement::new(48));
        let node_43 =
            (((node_39) * ((base_row[63]) - (BFieldElement::new(72)))) * (node_13)) * (node_15);
        let node_58 = (((node_39) * (node_9)) * (node_13)) * (node_15);
        let node_72 = ((BFieldElement::new(4294967295))
            - ((base_row[64]) * (BFieldElement::new(65536))))
            - (base_row[65]);
        let node_77 = ((BFieldElement::new(4294967295))
            - ((base_row[68]) * (BFieldElement::new(65536))))
            - (base_row[69]);
        let node_82 = ((BFieldElement::new(4294967295))
            - ((base_row[72]) * (BFieldElement::new(65536))))
            - (base_row[73]);
        let node_87 = ((BFieldElement::new(4294967295))
            - ((base_row[76]) * (BFieldElement::new(65536))))
            - (base_row[77]);
        let node_93 = ((node_72) * (base_row[108])) - (BFieldElement::new(1));
        let node_95 = ((node_77) * (base_row[109])) - (BFieldElement::new(1));
        let node_97 = ((node_82) * (base_row[110])) - (BFieldElement::new(1));
        let node_99 = ((node_87) * (base_row[111])) - (BFieldElement::new(1));

        let base_constraints = [
            ((((((base_row[62]) * (node_20)) * (node_23)) * (node_26)) * (node_29)) * (node_32))
                * (node_9),
            (node_43) * ((base_row[102]) - (BFieldElement::new(1))),
            (node_43) * ((base_row[103]) - (BFieldElement::new(1))),
            (node_43) * ((base_row[104]) - (BFieldElement::new(1))),
            (node_43) * ((base_row[105]) - (BFieldElement::new(1))),
            (node_43) * ((base_row[106]) - (BFieldElement::new(1))),
            (node_43) * ((base_row[107]) - (BFieldElement::new(1))),
            (node_58) * (base_row[102]),
            (node_58) * (base_row[103]),
            (node_58) * (base_row[104]),
            (node_58) * (base_row[105]),
            (node_58) * (base_row[106]),
            (node_58) * (base_row[107]),
            (node_93) * (base_row[108]),
            (node_95) * (base_row[109]),
            (node_97) * (base_row[110]),
            (node_99) * (base_row[111]),
            (node_93) * (node_72),
            (node_95) * (node_77),
            (node_97) * (node_82),
            (node_99) * (node_87),
            (node_93) * (((base_row[66]) * (BFieldElement::new(65536))) + (base_row[67])),
            (node_95) * (((base_row[70]) * (BFieldElement::new(65536))) + (base_row[71])),
            (node_97) * (((base_row[74]) * (BFieldElement::new(65536))) + (base_row[75])),
            (node_99) * (((base_row[78]) * (BFieldElement::new(65536))) + (base_row[79])),
            (((((node_39) * ((base_row[112]) - (BFieldElement::new(13630775303355457758))))
                + ((node_138) * ((base_row[112]) - (BFieldElement::new(17532528648579384106)))))
                + ((node_146) * ((base_row[112]) - (BFieldElement::new(13835756199368269249)))))
                + ((node_153) * ((base_row[112]) - (BFieldElement::new(549990724933663297)))))
                + ((node_159) * ((base_row[112]) - (BFieldElement::new(3350107164315270407)))),
            (((((node_39) * ((base_row[113]) - (BFieldElement::new(16896927574093233874))))
                + ((node_138) * ((base_row[113]) - (BFieldElement::new(5216785850422679555)))))
                + ((node_146) * ((base_row[113]) - (BFieldElement::new(1648753455944344172)))))
                + ((node_153) * ((base_row[113]) - (BFieldElement::new(4901984846118077401)))))
                + ((node_159) * ((base_row[113]) - (BFieldElement::new(17715942834299349177)))),
            (((((node_39) * ((base_row[114]) - (BFieldElement::new(10379449653650130495))))
                + ((node_138) * ((base_row[114]) - (BFieldElement::new(15418071332095031847)))))
                + ((node_146) * ((base_row[114]) - (BFieldElement::new(9836124473569258483)))))
                + ((node_153) * ((base_row[114]) - (BFieldElement::new(11458643033696775769)))))
                + ((node_159) * ((base_row[114]) - (BFieldElement::new(9600609149219873996)))),
            (((((node_39) * ((base_row[115]) - (BFieldElement::new(1965408364413093495))))
                + ((node_138) * ((base_row[115]) - (BFieldElement::new(11921929762955146258)))))
                + ((node_146) * ((base_row[115]) - (BFieldElement::new(12867641597107932229)))))
                + ((node_153) * ((base_row[115]) - (BFieldElement::new(8706785264119212710)))))
                + ((node_159) * ((base_row[115]) - (BFieldElement::new(12894357635820003949)))),
            (((((node_39) * ((base_row[116]) - (BFieldElement::new(15232538947090185111))))
                + ((node_138) * ((base_row[116]) - (BFieldElement::new(9738718993677019874)))))
                + ((node_146) * ((base_row[116]) - (BFieldElement::new(11254152636692960595)))))
                + ((node_153) * ((base_row[116]) - (BFieldElement::new(12521758138015724072)))))
                + ((node_159) * ((base_row[116]) - (BFieldElement::new(4597649658040514631)))),
            (((((node_39) * ((base_row[117]) - (BFieldElement::new(15892634398091747074))))
                + ((node_138) * ((base_row[117]) - (BFieldElement::new(3464580399432997147)))))
                + ((node_146) * ((base_row[117]) - (BFieldElement::new(16550832737139861108)))))
                + ((node_153) * ((base_row[117]) - (BFieldElement::new(11877914062416978196)))))
                + ((node_159) * ((base_row[117]) - (BFieldElement::new(7735563950920491847)))),
            (((((node_39) * ((base_row[118]) - (BFieldElement::new(3989134140024871768))))
                + ((node_138) * ((base_row[118]) - (BFieldElement::new(13408434769117164050)))))
                + ((node_146) * ((base_row[118]) - (BFieldElement::new(11861573970480733262)))))
                + ((node_153) * ((base_row[118]) - (BFieldElement::new(11333318251134523752)))))
                + ((node_159) * ((base_row[118]) - (BFieldElement::new(1663379455870887181)))),
            (((((node_39) * ((base_row[119]) - (BFieldElement::new(2851411912127730865))))
                + ((node_138) * ((base_row[119]) - (BFieldElement::new(264428218649616431)))))
                + ((node_146) * ((base_row[119]) - (BFieldElement::new(1256660473588673495)))))
                + ((node_153) * ((base_row[119]) - (BFieldElement::new(3933899631278608623)))))
                + ((node_159) * ((base_row[119]) - (BFieldElement::new(13889298103638829706)))),
            (((((node_39) * ((base_row[120]) - (BFieldElement::new(8709136439293758776))))
                + ((node_138) * ((base_row[120]) - (BFieldElement::new(4436247869008081381)))))
                + ((node_146) * ((base_row[120]) - (BFieldElement::new(13879506000676455136)))))
                + ((node_153) * ((base_row[120]) - (BFieldElement::new(16635128972021157924)))))
                + ((node_159) * ((base_row[120]) - (BFieldElement::new(7375530351220884434)))),
            (((((node_39) * ((base_row[121]) - (BFieldElement::new(3694858669662939734))))
                + ((node_138) * ((base_row[121]) - (BFieldElement::new(4063129435850804221)))))
                + ((node_146) * ((base_row[121]) - (BFieldElement::new(10564103842682358721)))))
                + ((node_153) * ((base_row[121]) - (BFieldElement::new(10291337173108950450)))))
                + ((node_159) * ((base_row[121]) - (BFieldElement::new(3502022433285269151)))),
            (((((node_39) * ((base_row[122]) - (BFieldElement::new(12692440244315327141))))
                + ((node_138) * ((base_row[122]) - (BFieldElement::new(2865073155741120117)))))
                + ((node_146) * ((base_row[122]) - (BFieldElement::new(16142842524796397521)))))
                + ((node_153) * ((base_row[122]) - (BFieldElement::new(4142107155024199350)))))
                + ((node_159) * ((base_row[122]) - (BFieldElement::new(9231805330431056952)))),
            (((((node_39) * ((base_row[123]) - (BFieldElement::new(10722316166358076749))))
                + ((node_138) * ((base_row[123]) - (BFieldElement::new(5749834437609765994)))))
                + ((node_146) * ((base_row[123]) - (BFieldElement::new(3287098591948630584)))))
                + ((node_153) * ((base_row[123]) - (BFieldElement::new(16973934533787743537)))))
                + ((node_159) * ((base_row[123]) - (BFieldElement::new(9252272755288523725)))),
            (((((node_39) * ((base_row[124]) - (BFieldElement::new(12745429320441639448))))
                + ((node_138) * ((base_row[124]) - (BFieldElement::new(6804196764189408435)))))
                + ((node_146) * ((base_row[124]) - (BFieldElement::new(685911471061284805)))))
                + ((node_153) * ((base_row[124]) - (BFieldElement::new(11068111539125175221)))))
                + ((node_159) * ((base_row[124]) - (BFieldElement::new(10014268662326746219)))),
            (((((node_39) * ((base_row[125]) - (BFieldElement::new(17932424223723990421))))
                + ((node_138) * ((base_row[125]) - (BFieldElement::new(17060469201292988508)))))
                + ((node_146) * ((base_row[125]) - (BFieldElement::new(5285298776918878023)))))
                + ((node_153) * ((base_row[125]) - (BFieldElement::new(17546769694830203606)))))
                + ((node_159) * ((base_row[125]) - (BFieldElement::new(15565031632950843234)))),
            (((((node_39) * ((base_row[126]) - (BFieldElement::new(7558102534867937463))))
                + ((node_138) * ((base_row[126]) - (BFieldElement::new(9475383556737206708)))))
                + ((node_146) * ((base_row[126]) - (BFieldElement::new(18310953571768047354)))))
                + ((node_153) * ((base_row[126]) - (BFieldElement::new(5315217744825068993)))))
                + ((node_159) * ((base_row[126]) - (BFieldElement::new(1209725273521819323)))),
            (((((node_39) * ((base_row[127]) - (BFieldElement::new(15551047435855531404))))
                + ((node_138) * ((base_row[127]) - (BFieldElement::new(12876344085611465020)))))
                + ((node_146) * ((base_row[127]) - (BFieldElement::new(3142266350630002035)))))
                + ((node_153) * ((base_row[127]) - (BFieldElement::new(4609594252909613081)))))
                + ((node_159) * ((base_row[127]) - (BFieldElement::new(6024642864597845108)))),
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
        let node_882 = (next_base_row[62]) + (BFieldElement::new(1));
        let node_898 = (next_base_row[62]) - (BFieldElement::new(1));
        let node_900 = (next_base_row[62]) - (BFieldElement::new(2));
        let node_902 = (next_base_row[62]) - (BFieldElement::new(3));
        let node_904 = (next_base_row[62]) - (BFieldElement::new(4));
        let node_906 = (next_base_row[62]) - (BFieldElement::new(5));
        let node_813 = (current_base_row[62]) - (BFieldElement::new(5));
        let node_811 = (current_base_row[62]) + (BFieldElement::new(1));
        let node_814 = (node_811) * (node_813);
        let node_87 = (((((current_base_row[80]) * (BFieldElement::new(281474976710656)))
            + ((current_base_row[81]) * (BFieldElement::new(4294967296))))
            + ((current_base_row[82]) * (BFieldElement::new(65536))))
            + (current_base_row[83]))
            * (BFieldElement::new(18446744065119617025));
        let node_98 = (((((current_base_row[84]) * (BFieldElement::new(281474976710656)))
            + ((current_base_row[85]) * (BFieldElement::new(4294967296))))
            + ((current_base_row[86]) * (BFieldElement::new(65536))))
            + (current_base_row[87]))
            * (BFieldElement::new(18446744065119617025));
        let node_109 = (((((current_base_row[88]) * (BFieldElement::new(281474976710656)))
            + ((current_base_row[89]) * (BFieldElement::new(4294967296))))
            + ((current_base_row[90]) * (BFieldElement::new(65536))))
            + (current_base_row[91]))
            * (BFieldElement::new(18446744065119617025));
        let node_120 = (((((current_base_row[92]) * (BFieldElement::new(281474976710656)))
            + ((current_base_row[93]) * (BFieldElement::new(4294967296))))
            + ((current_base_row[94]) * (BFieldElement::new(65536))))
            + (current_base_row[95]))
            * (BFieldElement::new(18446744065119617025));
        let node_181 = ((((((current_base_row[96]) * (current_base_row[96]))
            * (current_base_row[96]))
            * (current_base_row[96]))
            * (current_base_row[96]))
            * (current_base_row[96]))
            * (current_base_row[96]);
        let node_182 = ((((((current_base_row[97]) * (current_base_row[97]))
            * (current_base_row[97]))
            * (current_base_row[97]))
            * (current_base_row[97]))
            * (current_base_row[97]))
            * (current_base_row[97]);
        let node_183 = ((((((current_base_row[98]) * (current_base_row[98]))
            * (current_base_row[98]))
            * (current_base_row[98]))
            * (current_base_row[98]))
            * (current_base_row[98]))
            * (current_base_row[98]);
        let node_184 = ((((((current_base_row[99]) * (current_base_row[99]))
            * (current_base_row[99]))
            * (current_base_row[99]))
            * (current_base_row[99]))
            * (current_base_row[99]))
            * (current_base_row[99]);
        let node_185 = ((((((current_base_row[100]) * (current_base_row[100]))
            * (current_base_row[100]))
            * (current_base_row[100]))
            * (current_base_row[100]))
            * (current_base_row[100]))
            * (current_base_row[100]);
        let node_186 = ((((((current_base_row[101]) * (current_base_row[101]))
            * (current_base_row[101]))
            * (current_base_row[101]))
            * (current_base_row[101]))
            * (current_base_row[101]))
            * (current_base_row[101]);
        let node_187 = ((((((current_base_row[102]) * (current_base_row[102]))
            * (current_base_row[102]))
            * (current_base_row[102]))
            * (current_base_row[102]))
            * (current_base_row[102]))
            * (current_base_row[102]);
        let node_188 = ((((((current_base_row[103]) * (current_base_row[103]))
            * (current_base_row[103]))
            * (current_base_row[103]))
            * (current_base_row[103]))
            * (current_base_row[103]))
            * (current_base_row[103]);
        let node_189 = ((((((current_base_row[104]) * (current_base_row[104]))
            * (current_base_row[104]))
            * (current_base_row[104]))
            * (current_base_row[104]))
            * (current_base_row[104]))
            * (current_base_row[104]);
        let node_190 = ((((((current_base_row[105]) * (current_base_row[105]))
            * (current_base_row[105]))
            * (current_base_row[105]))
            * (current_base_row[105]))
            * (current_base_row[105]))
            * (current_base_row[105]);
        let node_191 = ((((((current_base_row[106]) * (current_base_row[106]))
            * (current_base_row[106]))
            * (current_base_row[106]))
            * (current_base_row[106]))
            * (current_base_row[106]))
            * (current_base_row[106]);
        let node_192 = ((((((current_base_row[107]) * (current_base_row[107]))
            * (current_base_row[107]))
            * (current_base_row[107]))
            * (current_base_row[107]))
            * (current_base_row[107]))
            * (current_base_row[107]);
        let node_1041 = (node_882) * (node_906);
        let node_1045 =
            ((((next_base_row[62]) * (node_898)) * (node_900)) * (node_902)) * (node_904);
        let node_894 = (next_base_row[63]) - (BFieldElement::new(48));
        let node_764 = (((((next_base_row[64]) * (BFieldElement::new(281474976710656)))
            + ((next_base_row[65]) * (BFieldElement::new(4294967296))))
            + ((next_base_row[66]) * (BFieldElement::new(65536))))
            + (next_base_row[67]))
            * (BFieldElement::new(18446744065119617025));
        let node_775 = (((((next_base_row[68]) * (BFieldElement::new(281474976710656)))
            + ((next_base_row[69]) * (BFieldElement::new(4294967296))))
            + ((next_base_row[70]) * (BFieldElement::new(65536))))
            + (next_base_row[71]))
            * (BFieldElement::new(18446744065119617025));
        let node_786 = (((((next_base_row[72]) * (BFieldElement::new(281474976710656)))
            + ((next_base_row[73]) * (BFieldElement::new(4294967296))))
            + ((next_base_row[74]) * (BFieldElement::new(65536))))
            + (next_base_row[75]))
            * (BFieldElement::new(18446744065119617025));
        let node_797 = (((((next_base_row[76]) * (BFieldElement::new(281474976710656)))
            + ((next_base_row[77]) * (BFieldElement::new(4294967296))))
            + ((next_base_row[78]) * (BFieldElement::new(65536))))
            + (next_base_row[79]))
            * (BFieldElement::new(18446744065119617025));
        let node_926 = (next_base_row[63]) - (BFieldElement::new(72));
        let node_907 =
            (((((node_882) * (node_898)) * (node_900)) * (node_902)) * (node_904)) * (node_906);
        let node_928 = (next_base_row[63]) - (BFieldElement::new(88));
        let node_966 = (next_base_row[63]) - (BFieldElement::new(80));
        let node_925 = (node_907) * (node_894);
        let node_970 = ((node_926) * (node_966)) * (node_928);
        let node_980 = (((((challenges.get_challenge(HashStateWeight0)) * (node_764))
            + ((challenges.get_challenge(HashStateWeight1)) * (node_775)))
            + ((challenges.get_challenge(HashStateWeight2)) * (node_786)))
            + ((challenges.get_challenge(HashStateWeight3)) * (node_797)))
            + ((challenges.get_challenge(HashStateWeight4)) * (next_base_row[96]));
        let node_865 = (current_base_row[62]) - (BFieldElement::new(1));
        let node_868 = (current_base_row[62]) - (BFieldElement::new(2));
        let node_871 = (current_base_row[62]) - (BFieldElement::new(3));
        let node_874 = (current_base_row[62]) - (BFieldElement::new(4));
        let node_927 = (node_925) * (node_926);
        let node_914 = (challenges.get_challenge(HashStateWeight10))
            * ((next_base_row[102]) - (current_base_row[102]));
        let node_915 = (challenges.get_challenge(HashStateWeight11))
            * ((next_base_row[103]) - (current_base_row[103]));
        let node_917 = (challenges.get_challenge(HashStateWeight12))
            * ((next_base_row[104]) - (current_base_row[104]));
        let node_919 = (challenges.get_challenge(HashStateWeight13))
            * ((next_base_row[105]) - (current_base_row[105]));
        let node_921 = (challenges.get_challenge(HashStateWeight14))
            * ((next_base_row[106]) - (current_base_row[106]));
        let node_923 = (challenges.get_challenge(HashStateWeight15))
            * ((next_base_row[107]) - (current_base_row[107]));
        let node_990 = (((((node_980)
            + ((challenges.get_challenge(HashStateWeight5)) * (next_base_row[97])))
            + ((challenges.get_challenge(HashStateWeight6)) * (next_base_row[98])))
            + ((challenges.get_challenge(HashStateWeight7)) * (next_base_row[99])))
            + ((challenges.get_challenge(HashStateWeight8)) * (next_base_row[100])))
            + ((challenges.get_challenge(HashStateWeight9)) * (next_base_row[101]));
        let node_971 = (next_ext_row[22]) - (current_ext_row[22]);
        let node_1006 = (next_ext_row[23]) - (current_ext_row[23]);
        let node_1023 = (next_ext_row[24]) - (current_ext_row[24]);
        let node_1037 = (next_ext_row[25]) - (current_ext_row[25]);
        let node_1055 = (next_ext_row[26]) - (current_ext_row[26]);
        let node_1068 = (next_ext_row[27]) - (current_ext_row[27]);
        let node_1081 = (next_ext_row[28]) - (current_ext_row[28]);
        let node_1094 = (next_ext_row[29]) - (current_ext_row[29]);
        let node_1107 = (next_ext_row[30]) - (current_ext_row[30]);
        let node_1120 = (next_ext_row[31]) - (current_ext_row[31]);
        let node_1133 = (next_ext_row[32]) - (current_ext_row[32]);
        let node_1146 = (next_ext_row[33]) - (current_ext_row[33]);
        let node_1159 = (next_ext_row[34]) - (current_ext_row[34]);
        let node_1172 = (next_ext_row[35]) - (current_ext_row[35]);
        let node_1185 = (next_ext_row[36]) - (current_ext_row[36]);
        let node_1198 = (next_ext_row[37]) - (current_ext_row[37]);
        let node_1211 = (next_ext_row[38]) - (current_ext_row[38]);
        let node_1224 = (next_ext_row[39]) - (current_ext_row[39]);
        let node_1237 = (next_ext_row[40]) - (current_ext_row[40]);

        let base_constraints = [
            ((((((current_base_row[62]) * (node_865)) * (node_868)) * (node_871)) * (node_874))
                * (node_813))
                * (node_882),
            (((((((node_811) * (current_base_row[62])) * (node_865)) * (node_868)) * (node_871))
                * (node_874))
                * (next_base_row[62]))
                * (node_882),
            (node_814) * (((next_base_row[62]) - (current_base_row[62])) - (BFieldElement::new(1))),
            ((((current_base_row[63]) - (BFieldElement::new(72)))
                * ((current_base_row[63]) - (BFieldElement::new(80))))
                * ((current_base_row[63]) - (BFieldElement::new(88))))
                * (node_894),
            (node_813) * ((next_base_row[63]) - (current_base_row[63])),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(61402)) * (node_87))
                    + ((BFieldElement::new(17845)) * (node_98)))
                    + ((BFieldElement::new(26798)) * (node_109)))
                    + ((BFieldElement::new(59689)) * (node_120)))
                    + ((BFieldElement::new(12021)) * (node_181)))
                    + ((BFieldElement::new(40901)) * (node_182)))
                    + ((BFieldElement::new(41351)) * (node_183)))
                    + ((BFieldElement::new(27521)) * (node_184)))
                    + ((BFieldElement::new(56951)) * (node_185)))
                    + ((BFieldElement::new(12034)) * (node_186)))
                    + ((BFieldElement::new(53865)) * (node_187)))
                    + ((BFieldElement::new(43244)) * (node_188)))
                    + ((BFieldElement::new(7454)) * (node_189)))
                    + ((BFieldElement::new(33823)) * (node_190)))
                    + ((BFieldElement::new(28750)) * (node_191)))
                    + ((BFieldElement::new(1108)) * (node_192)))
                    + (current_base_row[112]))
                    - (node_764)),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(1108)) * (node_87))
                    + ((BFieldElement::new(61402)) * (node_98)))
                    + ((BFieldElement::new(17845)) * (node_109)))
                    + ((BFieldElement::new(26798)) * (node_120)))
                    + ((BFieldElement::new(59689)) * (node_181)))
                    + ((BFieldElement::new(12021)) * (node_182)))
                    + ((BFieldElement::new(40901)) * (node_183)))
                    + ((BFieldElement::new(41351)) * (node_184)))
                    + ((BFieldElement::new(27521)) * (node_185)))
                    + ((BFieldElement::new(56951)) * (node_186)))
                    + ((BFieldElement::new(12034)) * (node_187)))
                    + ((BFieldElement::new(53865)) * (node_188)))
                    + ((BFieldElement::new(43244)) * (node_189)))
                    + ((BFieldElement::new(7454)) * (node_190)))
                    + ((BFieldElement::new(33823)) * (node_191)))
                    + ((BFieldElement::new(28750)) * (node_192)))
                    + (current_base_row[113]))
                    - (node_775)),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(28750)) * (node_87))
                    + ((BFieldElement::new(1108)) * (node_98)))
                    + ((BFieldElement::new(61402)) * (node_109)))
                    + ((BFieldElement::new(17845)) * (node_120)))
                    + ((BFieldElement::new(26798)) * (node_181)))
                    + ((BFieldElement::new(59689)) * (node_182)))
                    + ((BFieldElement::new(12021)) * (node_183)))
                    + ((BFieldElement::new(40901)) * (node_184)))
                    + ((BFieldElement::new(41351)) * (node_185)))
                    + ((BFieldElement::new(27521)) * (node_186)))
                    + ((BFieldElement::new(56951)) * (node_187)))
                    + ((BFieldElement::new(12034)) * (node_188)))
                    + ((BFieldElement::new(53865)) * (node_189)))
                    + ((BFieldElement::new(43244)) * (node_190)))
                    + ((BFieldElement::new(7454)) * (node_191)))
                    + ((BFieldElement::new(33823)) * (node_192)))
                    + (current_base_row[114]))
                    - (node_786)),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(33823)) * (node_87))
                    + ((BFieldElement::new(28750)) * (node_98)))
                    + ((BFieldElement::new(1108)) * (node_109)))
                    + ((BFieldElement::new(61402)) * (node_120)))
                    + ((BFieldElement::new(17845)) * (node_181)))
                    + ((BFieldElement::new(26798)) * (node_182)))
                    + ((BFieldElement::new(59689)) * (node_183)))
                    + ((BFieldElement::new(12021)) * (node_184)))
                    + ((BFieldElement::new(40901)) * (node_185)))
                    + ((BFieldElement::new(41351)) * (node_186)))
                    + ((BFieldElement::new(27521)) * (node_187)))
                    + ((BFieldElement::new(56951)) * (node_188)))
                    + ((BFieldElement::new(12034)) * (node_189)))
                    + ((BFieldElement::new(53865)) * (node_190)))
                    + ((BFieldElement::new(43244)) * (node_191)))
                    + ((BFieldElement::new(7454)) * (node_192)))
                    + (current_base_row[115]))
                    - (node_797)),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(7454)) * (node_87))
                    + ((BFieldElement::new(33823)) * (node_98)))
                    + ((BFieldElement::new(28750)) * (node_109)))
                    + ((BFieldElement::new(1108)) * (node_120)))
                    + ((BFieldElement::new(61402)) * (node_181)))
                    + ((BFieldElement::new(17845)) * (node_182)))
                    + ((BFieldElement::new(26798)) * (node_183)))
                    + ((BFieldElement::new(59689)) * (node_184)))
                    + ((BFieldElement::new(12021)) * (node_185)))
                    + ((BFieldElement::new(40901)) * (node_186)))
                    + ((BFieldElement::new(41351)) * (node_187)))
                    + ((BFieldElement::new(27521)) * (node_188)))
                    + ((BFieldElement::new(56951)) * (node_189)))
                    + ((BFieldElement::new(12034)) * (node_190)))
                    + ((BFieldElement::new(53865)) * (node_191)))
                    + ((BFieldElement::new(43244)) * (node_192)))
                    + (current_base_row[116]))
                    - (next_base_row[96])),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(43244)) * (node_87))
                    + ((BFieldElement::new(7454)) * (node_98)))
                    + ((BFieldElement::new(33823)) * (node_109)))
                    + ((BFieldElement::new(28750)) * (node_120)))
                    + ((BFieldElement::new(1108)) * (node_181)))
                    + ((BFieldElement::new(61402)) * (node_182)))
                    + ((BFieldElement::new(17845)) * (node_183)))
                    + ((BFieldElement::new(26798)) * (node_184)))
                    + ((BFieldElement::new(59689)) * (node_185)))
                    + ((BFieldElement::new(12021)) * (node_186)))
                    + ((BFieldElement::new(40901)) * (node_187)))
                    + ((BFieldElement::new(41351)) * (node_188)))
                    + ((BFieldElement::new(27521)) * (node_189)))
                    + ((BFieldElement::new(56951)) * (node_190)))
                    + ((BFieldElement::new(12034)) * (node_191)))
                    + ((BFieldElement::new(53865)) * (node_192)))
                    + (current_base_row[117]))
                    - (next_base_row[97])),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(53865)) * (node_87))
                    + ((BFieldElement::new(43244)) * (node_98)))
                    + ((BFieldElement::new(7454)) * (node_109)))
                    + ((BFieldElement::new(33823)) * (node_120)))
                    + ((BFieldElement::new(28750)) * (node_181)))
                    + ((BFieldElement::new(1108)) * (node_182)))
                    + ((BFieldElement::new(61402)) * (node_183)))
                    + ((BFieldElement::new(17845)) * (node_184)))
                    + ((BFieldElement::new(26798)) * (node_185)))
                    + ((BFieldElement::new(59689)) * (node_186)))
                    + ((BFieldElement::new(12021)) * (node_187)))
                    + ((BFieldElement::new(40901)) * (node_188)))
                    + ((BFieldElement::new(41351)) * (node_189)))
                    + ((BFieldElement::new(27521)) * (node_190)))
                    + ((BFieldElement::new(56951)) * (node_191)))
                    + ((BFieldElement::new(12034)) * (node_192)))
                    + (current_base_row[118]))
                    - (next_base_row[98])),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(12034)) * (node_87))
                    + ((BFieldElement::new(53865)) * (node_98)))
                    + ((BFieldElement::new(43244)) * (node_109)))
                    + ((BFieldElement::new(7454)) * (node_120)))
                    + ((BFieldElement::new(33823)) * (node_181)))
                    + ((BFieldElement::new(28750)) * (node_182)))
                    + ((BFieldElement::new(1108)) * (node_183)))
                    + ((BFieldElement::new(61402)) * (node_184)))
                    + ((BFieldElement::new(17845)) * (node_185)))
                    + ((BFieldElement::new(26798)) * (node_186)))
                    + ((BFieldElement::new(59689)) * (node_187)))
                    + ((BFieldElement::new(12021)) * (node_188)))
                    + ((BFieldElement::new(40901)) * (node_189)))
                    + ((BFieldElement::new(41351)) * (node_190)))
                    + ((BFieldElement::new(27521)) * (node_191)))
                    + ((BFieldElement::new(56951)) * (node_192)))
                    + (current_base_row[119]))
                    - (next_base_row[99])),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(56951)) * (node_87))
                    + ((BFieldElement::new(12034)) * (node_98)))
                    + ((BFieldElement::new(53865)) * (node_109)))
                    + ((BFieldElement::new(43244)) * (node_120)))
                    + ((BFieldElement::new(7454)) * (node_181)))
                    + ((BFieldElement::new(33823)) * (node_182)))
                    + ((BFieldElement::new(28750)) * (node_183)))
                    + ((BFieldElement::new(1108)) * (node_184)))
                    + ((BFieldElement::new(61402)) * (node_185)))
                    + ((BFieldElement::new(17845)) * (node_186)))
                    + ((BFieldElement::new(26798)) * (node_187)))
                    + ((BFieldElement::new(59689)) * (node_188)))
                    + ((BFieldElement::new(12021)) * (node_189)))
                    + ((BFieldElement::new(40901)) * (node_190)))
                    + ((BFieldElement::new(41351)) * (node_191)))
                    + ((BFieldElement::new(27521)) * (node_192)))
                    + (current_base_row[120]))
                    - (next_base_row[100])),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(27521)) * (node_87))
                    + ((BFieldElement::new(56951)) * (node_98)))
                    + ((BFieldElement::new(12034)) * (node_109)))
                    + ((BFieldElement::new(53865)) * (node_120)))
                    + ((BFieldElement::new(43244)) * (node_181)))
                    + ((BFieldElement::new(7454)) * (node_182)))
                    + ((BFieldElement::new(33823)) * (node_183)))
                    + ((BFieldElement::new(28750)) * (node_184)))
                    + ((BFieldElement::new(1108)) * (node_185)))
                    + ((BFieldElement::new(61402)) * (node_186)))
                    + ((BFieldElement::new(17845)) * (node_187)))
                    + ((BFieldElement::new(26798)) * (node_188)))
                    + ((BFieldElement::new(59689)) * (node_189)))
                    + ((BFieldElement::new(12021)) * (node_190)))
                    + ((BFieldElement::new(40901)) * (node_191)))
                    + ((BFieldElement::new(41351)) * (node_192)))
                    + (current_base_row[121]))
                    - (next_base_row[101])),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(41351)) * (node_87))
                    + ((BFieldElement::new(27521)) * (node_98)))
                    + ((BFieldElement::new(56951)) * (node_109)))
                    + ((BFieldElement::new(12034)) * (node_120)))
                    + ((BFieldElement::new(53865)) * (node_181)))
                    + ((BFieldElement::new(43244)) * (node_182)))
                    + ((BFieldElement::new(7454)) * (node_183)))
                    + ((BFieldElement::new(33823)) * (node_184)))
                    + ((BFieldElement::new(28750)) * (node_185)))
                    + ((BFieldElement::new(1108)) * (node_186)))
                    + ((BFieldElement::new(61402)) * (node_187)))
                    + ((BFieldElement::new(17845)) * (node_188)))
                    + ((BFieldElement::new(26798)) * (node_189)))
                    + ((BFieldElement::new(59689)) * (node_190)))
                    + ((BFieldElement::new(12021)) * (node_191)))
                    + ((BFieldElement::new(40901)) * (node_192)))
                    + (current_base_row[122]))
                    - (next_base_row[102])),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(40901)) * (node_87))
                    + ((BFieldElement::new(41351)) * (node_98)))
                    + ((BFieldElement::new(27521)) * (node_109)))
                    + ((BFieldElement::new(56951)) * (node_120)))
                    + ((BFieldElement::new(12034)) * (node_181)))
                    + ((BFieldElement::new(53865)) * (node_182)))
                    + ((BFieldElement::new(43244)) * (node_183)))
                    + ((BFieldElement::new(7454)) * (node_184)))
                    + ((BFieldElement::new(33823)) * (node_185)))
                    + ((BFieldElement::new(28750)) * (node_186)))
                    + ((BFieldElement::new(1108)) * (node_187)))
                    + ((BFieldElement::new(61402)) * (node_188)))
                    + ((BFieldElement::new(17845)) * (node_189)))
                    + ((BFieldElement::new(26798)) * (node_190)))
                    + ((BFieldElement::new(59689)) * (node_191)))
                    + ((BFieldElement::new(12021)) * (node_192)))
                    + (current_base_row[123]))
                    - (next_base_row[103])),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(12021)) * (node_87))
                    + ((BFieldElement::new(40901)) * (node_98)))
                    + ((BFieldElement::new(41351)) * (node_109)))
                    + ((BFieldElement::new(27521)) * (node_120)))
                    + ((BFieldElement::new(56951)) * (node_181)))
                    + ((BFieldElement::new(12034)) * (node_182)))
                    + ((BFieldElement::new(53865)) * (node_183)))
                    + ((BFieldElement::new(43244)) * (node_184)))
                    + ((BFieldElement::new(7454)) * (node_185)))
                    + ((BFieldElement::new(33823)) * (node_186)))
                    + ((BFieldElement::new(28750)) * (node_187)))
                    + ((BFieldElement::new(1108)) * (node_188)))
                    + ((BFieldElement::new(61402)) * (node_189)))
                    + ((BFieldElement::new(17845)) * (node_190)))
                    + ((BFieldElement::new(26798)) * (node_191)))
                    + ((BFieldElement::new(59689)) * (node_192)))
                    + (current_base_row[124]))
                    - (next_base_row[104])),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(59689)) * (node_87))
                    + ((BFieldElement::new(12021)) * (node_98)))
                    + ((BFieldElement::new(40901)) * (node_109)))
                    + ((BFieldElement::new(41351)) * (node_120)))
                    + ((BFieldElement::new(27521)) * (node_181)))
                    + ((BFieldElement::new(56951)) * (node_182)))
                    + ((BFieldElement::new(12034)) * (node_183)))
                    + ((BFieldElement::new(53865)) * (node_184)))
                    + ((BFieldElement::new(43244)) * (node_185)))
                    + ((BFieldElement::new(7454)) * (node_186)))
                    + ((BFieldElement::new(33823)) * (node_187)))
                    + ((BFieldElement::new(28750)) * (node_188)))
                    + ((BFieldElement::new(1108)) * (node_189)))
                    + ((BFieldElement::new(61402)) * (node_190)))
                    + ((BFieldElement::new(17845)) * (node_191)))
                    + ((BFieldElement::new(26798)) * (node_192)))
                    + (current_base_row[125]))
                    - (next_base_row[105])),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(26798)) * (node_87))
                    + ((BFieldElement::new(59689)) * (node_98)))
                    + ((BFieldElement::new(12021)) * (node_109)))
                    + ((BFieldElement::new(40901)) * (node_120)))
                    + ((BFieldElement::new(41351)) * (node_181)))
                    + ((BFieldElement::new(27521)) * (node_182)))
                    + ((BFieldElement::new(56951)) * (node_183)))
                    + ((BFieldElement::new(12034)) * (node_184)))
                    + ((BFieldElement::new(53865)) * (node_185)))
                    + ((BFieldElement::new(43244)) * (node_186)))
                    + ((BFieldElement::new(7454)) * (node_187)))
                    + ((BFieldElement::new(33823)) * (node_188)))
                    + ((BFieldElement::new(28750)) * (node_189)))
                    + ((BFieldElement::new(1108)) * (node_190)))
                    + ((BFieldElement::new(61402)) * (node_191)))
                    + ((BFieldElement::new(17845)) * (node_192)))
                    + (current_base_row[126]))
                    - (next_base_row[106])),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(17845)) * (node_87))
                    + ((BFieldElement::new(26798)) * (node_98)))
                    + ((BFieldElement::new(59689)) * (node_109)))
                    + ((BFieldElement::new(12021)) * (node_120)))
                    + ((BFieldElement::new(40901)) * (node_181)))
                    + ((BFieldElement::new(41351)) * (node_182)))
                    + ((BFieldElement::new(27521)) * (node_183)))
                    + ((BFieldElement::new(56951)) * (node_184)))
                    + ((BFieldElement::new(12034)) * (node_185)))
                    + ((BFieldElement::new(53865)) * (node_186)))
                    + ((BFieldElement::new(43244)) * (node_187)))
                    + ((BFieldElement::new(7454)) * (node_188)))
                    + ((BFieldElement::new(33823)) * (node_189)))
                    + ((BFieldElement::new(28750)) * (node_190)))
                    + ((BFieldElement::new(1108)) * (node_191)))
                    + ((BFieldElement::new(61402)) * (node_192)))
                    + (current_base_row[127]))
                    - (next_base_row[107])),
        ];
        let ext_constraints = [
            ((node_927) * (node_928))
                * ((((((node_914) + (node_915)) + (node_917)) + (node_919)) + (node_921))
                    + (node_923)),
            ((node_927) * (node_966))
                * (((((((((((((((((challenges.get_challenge(HashStateWeight0))
                    * ((node_764)
                        - ((((((current_base_row[64])
                            * (BFieldElement::new(281474976710656)))
                            + ((current_base_row[65])
                                * (BFieldElement::new(4294967296))))
                            + ((current_base_row[66])
                                * (BFieldElement::new(65536))))
                            + (current_base_row[67]))
                            * (BFieldElement::new(18446744065119617025)))))
                    + ((challenges.get_challenge(HashStateWeight1))
                        * ((node_775)
                            - ((((((current_base_row[68])
                                * (BFieldElement::new(281474976710656)))
                                + ((current_base_row[69])
                                    * (BFieldElement::new(4294967296))))
                                + ((current_base_row[70])
                                    * (BFieldElement::new(65536))))
                                + (current_base_row[71]))
                                * (BFieldElement::new(18446744065119617025))))))
                    + ((challenges.get_challenge(HashStateWeight2))
                        * ((node_786)
                            - ((((((current_base_row[72])
                                * (BFieldElement::new(281474976710656)))
                                + ((current_base_row[73])
                                    * (BFieldElement::new(4294967296))))
                                + ((current_base_row[74])
                                    * (BFieldElement::new(65536))))
                                + (current_base_row[75]))
                                * (BFieldElement::new(18446744065119617025))))))
                    + ((challenges.get_challenge(HashStateWeight3))
                        * ((node_797)
                            - ((((((current_base_row[76])
                                * (BFieldElement::new(281474976710656)))
                                + ((current_base_row[77])
                                    * (BFieldElement::new(4294967296))))
                                + ((current_base_row[78])
                                    * (BFieldElement::new(65536))))
                                + (current_base_row[79]))
                                * (BFieldElement::new(18446744065119617025))))))
                    + ((challenges.get_challenge(HashStateWeight4))
                        * ((next_base_row[96]) - (current_base_row[96]))))
                    + ((challenges.get_challenge(HashStateWeight5))
                        * ((next_base_row[97]) - (current_base_row[97]))))
                    + ((challenges.get_challenge(HashStateWeight6))
                        * ((next_base_row[98]) - (current_base_row[98]))))
                    + ((challenges.get_challenge(HashStateWeight7))
                        * ((next_base_row[99]) - (current_base_row[99]))))
                    + ((challenges.get_challenge(HashStateWeight8))
                        * ((next_base_row[100]) - (current_base_row[100]))))
                    + ((challenges.get_challenge(HashStateWeight9))
                        * ((next_base_row[101]) - (current_base_row[101]))))
                    + (node_914))
                    + (node_915))
                    + (node_917))
                    + (node_919))
                    + (node_921))
                    + (node_923)),
            ((((node_907) * (node_970))
                * (((next_ext_row[22])
                    - ((challenges.get_challenge(HashInputIndeterminate))
                        * (current_ext_row[22])))
                    - (node_990)))
                + ((next_base_row[62]) * (node_971)))
                + ((node_894) * (node_971)),
            (((((((((node_882) * (next_base_row[62])) * (node_898)) * (node_900)) * (node_902))
                * (node_904))
                * (node_970))
                * (((next_ext_row[23])
                    - ((challenges.get_challenge(HashDigestIndeterminate))
                        * (current_ext_row[23])))
                    - (node_980)))
                + ((node_906) * (node_1006)))
                + ((node_894) * (node_1006)),
            (((node_925)
                * ((((next_ext_row[24])
                    - ((challenges.get_challenge(SpongeIndeterminate))
                        * (current_ext_row[24])))
                    - ((challenges.get_challenge(HashCIWeight)) * (next_base_row[63])))
                    - (node_990)))
                + ((next_base_row[62]) * (node_1023)))
                + ((node_970) * (node_1023)),
            ((node_1041)
                * (((node_1037)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[64]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[80])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1037)),
            ((node_1041)
                * (((node_1055)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[65]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[81])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1055)),
            ((node_1041)
                * (((node_1068)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[66]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[82])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1068)),
            ((node_1041)
                * (((node_1081)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[67]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[83])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1081)),
            ((node_1041)
                * (((node_1094)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[68]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[84])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1094)),
            ((node_1041)
                * (((node_1107)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[69]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[85])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1107)),
            ((node_1041)
                * (((node_1120)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[70]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[86])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1120)),
            ((node_1041)
                * (((node_1133)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[71]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[87])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1133)),
            ((node_1041)
                * (((node_1146)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[72]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[88])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1146)),
            ((node_1041)
                * (((node_1159)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[73]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[89])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1159)),
            ((node_1041)
                * (((node_1172)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[74]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[90])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1172)),
            ((node_1041)
                * (((node_1185)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[75]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[91])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1185)),
            ((node_1041)
                * (((node_1198)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[76]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[92])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1198)),
            ((node_1041)
                * (((node_1211)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[77]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[93])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1211)),
            ((node_1041)
                * (((node_1224)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[78]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[94])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1224)),
            ((node_1041)
                * (((node_1237)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[79]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[95])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1237)),
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

impl Evaluable<XFieldElement> for ExtHashTable {
    #[inline]
    #[allow(unused_variables)]
    fn evaluate_initial_constraints(
        base_row: ArrayView1<XFieldElement>,
        ext_row: ArrayView1<XFieldElement>,
        challenges: &Challenges,
    ) -> Vec<XFieldElement> {
        let node_89 = (base_row[62]) + (BFieldElement::new(1));
        let node_94 = (base_row[63]) - (BFieldElement::new(72));
        let node_92 = (base_row[63]) - (BFieldElement::new(48));
        let node_88 = ((((((((((challenges.get_challenge(HashStateWeight0))
            * ((((((base_row[64]) * (BFieldElement::new(281474976710656)))
                + ((base_row[65]) * (BFieldElement::new(4294967296))))
                + ((base_row[66]) * (BFieldElement::new(65536))))
                + (base_row[67]))
                * (BFieldElement::new(18446744065119617025))))
            + ((challenges.get_challenge(HashStateWeight1))
                * ((((((base_row[68]) * (BFieldElement::new(281474976710656)))
                    + ((base_row[69]) * (BFieldElement::new(4294967296))))
                    + ((base_row[70]) * (BFieldElement::new(65536))))
                    + (base_row[71]))
                    * (BFieldElement::new(18446744065119617025)))))
            + ((challenges.get_challenge(HashStateWeight2))
                * ((((((base_row[72]) * (BFieldElement::new(281474976710656)))
                    + ((base_row[73]) * (BFieldElement::new(4294967296))))
                    + ((base_row[74]) * (BFieldElement::new(65536))))
                    + (base_row[75]))
                    * (BFieldElement::new(18446744065119617025)))))
            + ((challenges.get_challenge(HashStateWeight3))
                * ((((((base_row[76]) * (BFieldElement::new(281474976710656)))
                    + ((base_row[77]) * (BFieldElement::new(4294967296))))
                    + ((base_row[78]) * (BFieldElement::new(65536))))
                    + (base_row[79]))
                    * (BFieldElement::new(18446744065119617025)))))
            + ((challenges.get_challenge(HashStateWeight4)) * (base_row[96])))
            + ((challenges.get_challenge(HashStateWeight5)) * (base_row[97])))
            + ((challenges.get_challenge(HashStateWeight6)) * (base_row[98])))
            + ((challenges.get_challenge(HashStateWeight7)) * (base_row[99])))
            + ((challenges.get_challenge(HashStateWeight8)) * (base_row[100])))
            + ((challenges.get_challenge(HashStateWeight9)) * (base_row[101]));
        let node_97 = (ext_row[22]) - (BFieldElement::new(1));

        let base_constraints = [(node_89) * (base_row[62]), (node_94) * (node_92)];
        let ext_constraints = [
            ((((node_89) * (node_94))
                * (((ext_row[22]) - (challenges.get_challenge(HashInputIndeterminate)))
                    - (node_88)))
                + ((node_92) * (node_97)))
                + ((base_row[62]) * (node_97)),
            (ext_row[23]) - (BFieldElement::new(1)),
            ((node_92)
                * ((((ext_row[24]) - (challenges.get_challenge(SpongeIndeterminate)))
                    - ((challenges.get_challenge(HashCIWeight)) * (BFieldElement::new(72))))
                    - (node_88)))
                + ((node_94) * ((ext_row[24]) - (BFieldElement::new(1)))),
            ((node_89)
                * (((ext_row[25])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[64]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[80])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[25])),
            ((node_89)
                * (((ext_row[26])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[65]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[81])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[26])),
            ((node_89)
                * (((ext_row[27])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[66]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[82])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[27])),
            ((node_89)
                * (((ext_row[28])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[67]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[83])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[28])),
            ((node_89)
                * (((ext_row[29])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[68]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[84])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[29])),
            ((node_89)
                * (((ext_row[30])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[69]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[85])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[30])),
            ((node_89)
                * (((ext_row[31])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[70]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[86])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[31])),
            ((node_89)
                * (((ext_row[32])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[71]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[87])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[32])),
            ((node_89)
                * (((ext_row[33])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[72]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[88])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[33])),
            ((node_89)
                * (((ext_row[34])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[73]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[89])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[34])),
            ((node_89)
                * (((ext_row[35])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[74]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[90])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[35])),
            ((node_89)
                * (((ext_row[36])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[75]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[91])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[36])),
            ((node_89)
                * (((ext_row[37])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[76]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[92])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[37])),
            ((node_89)
                * (((ext_row[38])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[77]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[93])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[38])),
            ((node_89)
                * (((ext_row[39])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[78]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[94])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[39])),
            ((node_89)
                * (((ext_row[40])
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (base_row[79]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (base_row[95])))))
                    - (BFieldElement::new(1))))
                + ((base_row[62]) * (ext_row[40])),
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
        let node_32 = (base_row[62]) - (BFieldElement::new(5));
        let node_34 = (base_row[62]) + (BFieldElement::new(1));
        let node_20 = (base_row[62]) - (BFieldElement::new(1));
        let node_23 = (base_row[62]) - (BFieldElement::new(2));
        let node_26 = (base_row[62]) - (BFieldElement::new(3));
        let node_29 = (base_row[62]) - (BFieldElement::new(4));
        let node_134 = (node_34) * (base_row[62]);
        let node_143 = (node_134) * (node_20);
        let node_151 = (node_143) * (node_23);
        let node_39 = (((((node_34) * (node_20)) * (node_23)) * (node_26)) * (node_29)) * (node_32);
        let node_138 = ((((node_134) * (node_23)) * (node_26)) * (node_29)) * (node_32);
        let node_146 = (((node_143) * (node_26)) * (node_29)) * (node_32);
        let node_153 = ((node_151) * (node_29)) * (node_32);
        let node_159 = ((node_151) * (node_26)) * (node_32);
        let node_13 = (base_row[63]) - (BFieldElement::new(80));
        let node_15 = (base_row[63]) - (BFieldElement::new(88));
        let node_9 = (base_row[63]) - (BFieldElement::new(48));
        let node_43 =
            (((node_39) * ((base_row[63]) - (BFieldElement::new(72)))) * (node_13)) * (node_15);
        let node_58 = (((node_39) * (node_9)) * (node_13)) * (node_15);
        let node_72 = ((BFieldElement::new(4294967295))
            - ((base_row[64]) * (BFieldElement::new(65536))))
            - (base_row[65]);
        let node_77 = ((BFieldElement::new(4294967295))
            - ((base_row[68]) * (BFieldElement::new(65536))))
            - (base_row[69]);
        let node_82 = ((BFieldElement::new(4294967295))
            - ((base_row[72]) * (BFieldElement::new(65536))))
            - (base_row[73]);
        let node_87 = ((BFieldElement::new(4294967295))
            - ((base_row[76]) * (BFieldElement::new(65536))))
            - (base_row[77]);
        let node_93 = ((node_72) * (base_row[108])) - (BFieldElement::new(1));
        let node_95 = ((node_77) * (base_row[109])) - (BFieldElement::new(1));
        let node_97 = ((node_82) * (base_row[110])) - (BFieldElement::new(1));
        let node_99 = ((node_87) * (base_row[111])) - (BFieldElement::new(1));

        let base_constraints = [
            ((((((base_row[62]) * (node_20)) * (node_23)) * (node_26)) * (node_29)) * (node_32))
                * (node_9),
            (node_43) * ((base_row[102]) - (BFieldElement::new(1))),
            (node_43) * ((base_row[103]) - (BFieldElement::new(1))),
            (node_43) * ((base_row[104]) - (BFieldElement::new(1))),
            (node_43) * ((base_row[105]) - (BFieldElement::new(1))),
            (node_43) * ((base_row[106]) - (BFieldElement::new(1))),
            (node_43) * ((base_row[107]) - (BFieldElement::new(1))),
            (node_58) * (base_row[102]),
            (node_58) * (base_row[103]),
            (node_58) * (base_row[104]),
            (node_58) * (base_row[105]),
            (node_58) * (base_row[106]),
            (node_58) * (base_row[107]),
            (node_93) * (base_row[108]),
            (node_95) * (base_row[109]),
            (node_97) * (base_row[110]),
            (node_99) * (base_row[111]),
            (node_93) * (node_72),
            (node_95) * (node_77),
            (node_97) * (node_82),
            (node_99) * (node_87),
            (node_93) * (((base_row[66]) * (BFieldElement::new(65536))) + (base_row[67])),
            (node_95) * (((base_row[70]) * (BFieldElement::new(65536))) + (base_row[71])),
            (node_97) * (((base_row[74]) * (BFieldElement::new(65536))) + (base_row[75])),
            (node_99) * (((base_row[78]) * (BFieldElement::new(65536))) + (base_row[79])),
            (((((node_39) * ((base_row[112]) - (BFieldElement::new(13630775303355457758))))
                + ((node_138) * ((base_row[112]) - (BFieldElement::new(17532528648579384106)))))
                + ((node_146) * ((base_row[112]) - (BFieldElement::new(13835756199368269249)))))
                + ((node_153) * ((base_row[112]) - (BFieldElement::new(549990724933663297)))))
                + ((node_159) * ((base_row[112]) - (BFieldElement::new(3350107164315270407)))),
            (((((node_39) * ((base_row[113]) - (BFieldElement::new(16896927574093233874))))
                + ((node_138) * ((base_row[113]) - (BFieldElement::new(5216785850422679555)))))
                + ((node_146) * ((base_row[113]) - (BFieldElement::new(1648753455944344172)))))
                + ((node_153) * ((base_row[113]) - (BFieldElement::new(4901984846118077401)))))
                + ((node_159) * ((base_row[113]) - (BFieldElement::new(17715942834299349177)))),
            (((((node_39) * ((base_row[114]) - (BFieldElement::new(10379449653650130495))))
                + ((node_138) * ((base_row[114]) - (BFieldElement::new(15418071332095031847)))))
                + ((node_146) * ((base_row[114]) - (BFieldElement::new(9836124473569258483)))))
                + ((node_153) * ((base_row[114]) - (BFieldElement::new(11458643033696775769)))))
                + ((node_159) * ((base_row[114]) - (BFieldElement::new(9600609149219873996)))),
            (((((node_39) * ((base_row[115]) - (BFieldElement::new(1965408364413093495))))
                + ((node_138) * ((base_row[115]) - (BFieldElement::new(11921929762955146258)))))
                + ((node_146) * ((base_row[115]) - (BFieldElement::new(12867641597107932229)))))
                + ((node_153) * ((base_row[115]) - (BFieldElement::new(8706785264119212710)))))
                + ((node_159) * ((base_row[115]) - (BFieldElement::new(12894357635820003949)))),
            (((((node_39) * ((base_row[116]) - (BFieldElement::new(15232538947090185111))))
                + ((node_138) * ((base_row[116]) - (BFieldElement::new(9738718993677019874)))))
                + ((node_146) * ((base_row[116]) - (BFieldElement::new(11254152636692960595)))))
                + ((node_153) * ((base_row[116]) - (BFieldElement::new(12521758138015724072)))))
                + ((node_159) * ((base_row[116]) - (BFieldElement::new(4597649658040514631)))),
            (((((node_39) * ((base_row[117]) - (BFieldElement::new(15892634398091747074))))
                + ((node_138) * ((base_row[117]) - (BFieldElement::new(3464580399432997147)))))
                + ((node_146) * ((base_row[117]) - (BFieldElement::new(16550832737139861108)))))
                + ((node_153) * ((base_row[117]) - (BFieldElement::new(11877914062416978196)))))
                + ((node_159) * ((base_row[117]) - (BFieldElement::new(7735563950920491847)))),
            (((((node_39) * ((base_row[118]) - (BFieldElement::new(3989134140024871768))))
                + ((node_138) * ((base_row[118]) - (BFieldElement::new(13408434769117164050)))))
                + ((node_146) * ((base_row[118]) - (BFieldElement::new(11861573970480733262)))))
                + ((node_153) * ((base_row[118]) - (BFieldElement::new(11333318251134523752)))))
                + ((node_159) * ((base_row[118]) - (BFieldElement::new(1663379455870887181)))),
            (((((node_39) * ((base_row[119]) - (BFieldElement::new(2851411912127730865))))
                + ((node_138) * ((base_row[119]) - (BFieldElement::new(264428218649616431)))))
                + ((node_146) * ((base_row[119]) - (BFieldElement::new(1256660473588673495)))))
                + ((node_153) * ((base_row[119]) - (BFieldElement::new(3933899631278608623)))))
                + ((node_159) * ((base_row[119]) - (BFieldElement::new(13889298103638829706)))),
            (((((node_39) * ((base_row[120]) - (BFieldElement::new(8709136439293758776))))
                + ((node_138) * ((base_row[120]) - (BFieldElement::new(4436247869008081381)))))
                + ((node_146) * ((base_row[120]) - (BFieldElement::new(13879506000676455136)))))
                + ((node_153) * ((base_row[120]) - (BFieldElement::new(16635128972021157924)))))
                + ((node_159) * ((base_row[120]) - (BFieldElement::new(7375530351220884434)))),
            (((((node_39) * ((base_row[121]) - (BFieldElement::new(3694858669662939734))))
                + ((node_138) * ((base_row[121]) - (BFieldElement::new(4063129435850804221)))))
                + ((node_146) * ((base_row[121]) - (BFieldElement::new(10564103842682358721)))))
                + ((node_153) * ((base_row[121]) - (BFieldElement::new(10291337173108950450)))))
                + ((node_159) * ((base_row[121]) - (BFieldElement::new(3502022433285269151)))),
            (((((node_39) * ((base_row[122]) - (BFieldElement::new(12692440244315327141))))
                + ((node_138) * ((base_row[122]) - (BFieldElement::new(2865073155741120117)))))
                + ((node_146) * ((base_row[122]) - (BFieldElement::new(16142842524796397521)))))
                + ((node_153) * ((base_row[122]) - (BFieldElement::new(4142107155024199350)))))
                + ((node_159) * ((base_row[122]) - (BFieldElement::new(9231805330431056952)))),
            (((((node_39) * ((base_row[123]) - (BFieldElement::new(10722316166358076749))))
                + ((node_138) * ((base_row[123]) - (BFieldElement::new(5749834437609765994)))))
                + ((node_146) * ((base_row[123]) - (BFieldElement::new(3287098591948630584)))))
                + ((node_153) * ((base_row[123]) - (BFieldElement::new(16973934533787743537)))))
                + ((node_159) * ((base_row[123]) - (BFieldElement::new(9252272755288523725)))),
            (((((node_39) * ((base_row[124]) - (BFieldElement::new(12745429320441639448))))
                + ((node_138) * ((base_row[124]) - (BFieldElement::new(6804196764189408435)))))
                + ((node_146) * ((base_row[124]) - (BFieldElement::new(685911471061284805)))))
                + ((node_153) * ((base_row[124]) - (BFieldElement::new(11068111539125175221)))))
                + ((node_159) * ((base_row[124]) - (BFieldElement::new(10014268662326746219)))),
            (((((node_39) * ((base_row[125]) - (BFieldElement::new(17932424223723990421))))
                + ((node_138) * ((base_row[125]) - (BFieldElement::new(17060469201292988508)))))
                + ((node_146) * ((base_row[125]) - (BFieldElement::new(5285298776918878023)))))
                + ((node_153) * ((base_row[125]) - (BFieldElement::new(17546769694830203606)))))
                + ((node_159) * ((base_row[125]) - (BFieldElement::new(15565031632950843234)))),
            (((((node_39) * ((base_row[126]) - (BFieldElement::new(7558102534867937463))))
                + ((node_138) * ((base_row[126]) - (BFieldElement::new(9475383556737206708)))))
                + ((node_146) * ((base_row[126]) - (BFieldElement::new(18310953571768047354)))))
                + ((node_153) * ((base_row[126]) - (BFieldElement::new(5315217744825068993)))))
                + ((node_159) * ((base_row[126]) - (BFieldElement::new(1209725273521819323)))),
            (((((node_39) * ((base_row[127]) - (BFieldElement::new(15551047435855531404))))
                + ((node_138) * ((base_row[127]) - (BFieldElement::new(12876344085611465020)))))
                + ((node_146) * ((base_row[127]) - (BFieldElement::new(3142266350630002035)))))
                + ((node_153) * ((base_row[127]) - (BFieldElement::new(4609594252909613081)))))
                + ((node_159) * ((base_row[127]) - (BFieldElement::new(6024642864597845108)))),
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
        let node_882 = (next_base_row[62]) + (BFieldElement::new(1));
        let node_898 = (next_base_row[62]) - (BFieldElement::new(1));
        let node_900 = (next_base_row[62]) - (BFieldElement::new(2));
        let node_902 = (next_base_row[62]) - (BFieldElement::new(3));
        let node_904 = (next_base_row[62]) - (BFieldElement::new(4));
        let node_906 = (next_base_row[62]) - (BFieldElement::new(5));
        let node_813 = (current_base_row[62]) - (BFieldElement::new(5));
        let node_811 = (current_base_row[62]) + (BFieldElement::new(1));
        let node_814 = (node_811) * (node_813);
        let node_87 = (((((current_base_row[80]) * (BFieldElement::new(281474976710656)))
            + ((current_base_row[81]) * (BFieldElement::new(4294967296))))
            + ((current_base_row[82]) * (BFieldElement::new(65536))))
            + (current_base_row[83]))
            * (BFieldElement::new(18446744065119617025));
        let node_98 = (((((current_base_row[84]) * (BFieldElement::new(281474976710656)))
            + ((current_base_row[85]) * (BFieldElement::new(4294967296))))
            + ((current_base_row[86]) * (BFieldElement::new(65536))))
            + (current_base_row[87]))
            * (BFieldElement::new(18446744065119617025));
        let node_109 = (((((current_base_row[88]) * (BFieldElement::new(281474976710656)))
            + ((current_base_row[89]) * (BFieldElement::new(4294967296))))
            + ((current_base_row[90]) * (BFieldElement::new(65536))))
            + (current_base_row[91]))
            * (BFieldElement::new(18446744065119617025));
        let node_120 = (((((current_base_row[92]) * (BFieldElement::new(281474976710656)))
            + ((current_base_row[93]) * (BFieldElement::new(4294967296))))
            + ((current_base_row[94]) * (BFieldElement::new(65536))))
            + (current_base_row[95]))
            * (BFieldElement::new(18446744065119617025));
        let node_181 = ((((((current_base_row[96]) * (current_base_row[96]))
            * (current_base_row[96]))
            * (current_base_row[96]))
            * (current_base_row[96]))
            * (current_base_row[96]))
            * (current_base_row[96]);
        let node_182 = ((((((current_base_row[97]) * (current_base_row[97]))
            * (current_base_row[97]))
            * (current_base_row[97]))
            * (current_base_row[97]))
            * (current_base_row[97]))
            * (current_base_row[97]);
        let node_183 = ((((((current_base_row[98]) * (current_base_row[98]))
            * (current_base_row[98]))
            * (current_base_row[98]))
            * (current_base_row[98]))
            * (current_base_row[98]))
            * (current_base_row[98]);
        let node_184 = ((((((current_base_row[99]) * (current_base_row[99]))
            * (current_base_row[99]))
            * (current_base_row[99]))
            * (current_base_row[99]))
            * (current_base_row[99]))
            * (current_base_row[99]);
        let node_185 = ((((((current_base_row[100]) * (current_base_row[100]))
            * (current_base_row[100]))
            * (current_base_row[100]))
            * (current_base_row[100]))
            * (current_base_row[100]))
            * (current_base_row[100]);
        let node_186 = ((((((current_base_row[101]) * (current_base_row[101]))
            * (current_base_row[101]))
            * (current_base_row[101]))
            * (current_base_row[101]))
            * (current_base_row[101]))
            * (current_base_row[101]);
        let node_187 = ((((((current_base_row[102]) * (current_base_row[102]))
            * (current_base_row[102]))
            * (current_base_row[102]))
            * (current_base_row[102]))
            * (current_base_row[102]))
            * (current_base_row[102]);
        let node_188 = ((((((current_base_row[103]) * (current_base_row[103]))
            * (current_base_row[103]))
            * (current_base_row[103]))
            * (current_base_row[103]))
            * (current_base_row[103]))
            * (current_base_row[103]);
        let node_189 = ((((((current_base_row[104]) * (current_base_row[104]))
            * (current_base_row[104]))
            * (current_base_row[104]))
            * (current_base_row[104]))
            * (current_base_row[104]))
            * (current_base_row[104]);
        let node_190 = ((((((current_base_row[105]) * (current_base_row[105]))
            * (current_base_row[105]))
            * (current_base_row[105]))
            * (current_base_row[105]))
            * (current_base_row[105]))
            * (current_base_row[105]);
        let node_191 = ((((((current_base_row[106]) * (current_base_row[106]))
            * (current_base_row[106]))
            * (current_base_row[106]))
            * (current_base_row[106]))
            * (current_base_row[106]))
            * (current_base_row[106]);
        let node_192 = ((((((current_base_row[107]) * (current_base_row[107]))
            * (current_base_row[107]))
            * (current_base_row[107]))
            * (current_base_row[107]))
            * (current_base_row[107]))
            * (current_base_row[107]);
        let node_1041 = (node_882) * (node_906);
        let node_1045 =
            ((((next_base_row[62]) * (node_898)) * (node_900)) * (node_902)) * (node_904);
        let node_894 = (next_base_row[63]) - (BFieldElement::new(48));
        let node_764 = (((((next_base_row[64]) * (BFieldElement::new(281474976710656)))
            + ((next_base_row[65]) * (BFieldElement::new(4294967296))))
            + ((next_base_row[66]) * (BFieldElement::new(65536))))
            + (next_base_row[67]))
            * (BFieldElement::new(18446744065119617025));
        let node_775 = (((((next_base_row[68]) * (BFieldElement::new(281474976710656)))
            + ((next_base_row[69]) * (BFieldElement::new(4294967296))))
            + ((next_base_row[70]) * (BFieldElement::new(65536))))
            + (next_base_row[71]))
            * (BFieldElement::new(18446744065119617025));
        let node_786 = (((((next_base_row[72]) * (BFieldElement::new(281474976710656)))
            + ((next_base_row[73]) * (BFieldElement::new(4294967296))))
            + ((next_base_row[74]) * (BFieldElement::new(65536))))
            + (next_base_row[75]))
            * (BFieldElement::new(18446744065119617025));
        let node_797 = (((((next_base_row[76]) * (BFieldElement::new(281474976710656)))
            + ((next_base_row[77]) * (BFieldElement::new(4294967296))))
            + ((next_base_row[78]) * (BFieldElement::new(65536))))
            + (next_base_row[79]))
            * (BFieldElement::new(18446744065119617025));
        let node_926 = (next_base_row[63]) - (BFieldElement::new(72));
        let node_907 =
            (((((node_882) * (node_898)) * (node_900)) * (node_902)) * (node_904)) * (node_906);
        let node_928 = (next_base_row[63]) - (BFieldElement::new(88));
        let node_966 = (next_base_row[63]) - (BFieldElement::new(80));
        let node_925 = (node_907) * (node_894);
        let node_970 = ((node_926) * (node_966)) * (node_928);
        let node_980 = (((((challenges.get_challenge(HashStateWeight0)) * (node_764))
            + ((challenges.get_challenge(HashStateWeight1)) * (node_775)))
            + ((challenges.get_challenge(HashStateWeight2)) * (node_786)))
            + ((challenges.get_challenge(HashStateWeight3)) * (node_797)))
            + ((challenges.get_challenge(HashStateWeight4)) * (next_base_row[96]));
        let node_865 = (current_base_row[62]) - (BFieldElement::new(1));
        let node_868 = (current_base_row[62]) - (BFieldElement::new(2));
        let node_871 = (current_base_row[62]) - (BFieldElement::new(3));
        let node_874 = (current_base_row[62]) - (BFieldElement::new(4));
        let node_927 = (node_925) * (node_926);
        let node_914 = (challenges.get_challenge(HashStateWeight10))
            * ((next_base_row[102]) - (current_base_row[102]));
        let node_915 = (challenges.get_challenge(HashStateWeight11))
            * ((next_base_row[103]) - (current_base_row[103]));
        let node_917 = (challenges.get_challenge(HashStateWeight12))
            * ((next_base_row[104]) - (current_base_row[104]));
        let node_919 = (challenges.get_challenge(HashStateWeight13))
            * ((next_base_row[105]) - (current_base_row[105]));
        let node_921 = (challenges.get_challenge(HashStateWeight14))
            * ((next_base_row[106]) - (current_base_row[106]));
        let node_923 = (challenges.get_challenge(HashStateWeight15))
            * ((next_base_row[107]) - (current_base_row[107]));
        let node_990 = (((((node_980)
            + ((challenges.get_challenge(HashStateWeight5)) * (next_base_row[97])))
            + ((challenges.get_challenge(HashStateWeight6)) * (next_base_row[98])))
            + ((challenges.get_challenge(HashStateWeight7)) * (next_base_row[99])))
            + ((challenges.get_challenge(HashStateWeight8)) * (next_base_row[100])))
            + ((challenges.get_challenge(HashStateWeight9)) * (next_base_row[101]));
        let node_971 = (next_ext_row[22]) - (current_ext_row[22]);
        let node_1006 = (next_ext_row[23]) - (current_ext_row[23]);
        let node_1023 = (next_ext_row[24]) - (current_ext_row[24]);
        let node_1037 = (next_ext_row[25]) - (current_ext_row[25]);
        let node_1055 = (next_ext_row[26]) - (current_ext_row[26]);
        let node_1068 = (next_ext_row[27]) - (current_ext_row[27]);
        let node_1081 = (next_ext_row[28]) - (current_ext_row[28]);
        let node_1094 = (next_ext_row[29]) - (current_ext_row[29]);
        let node_1107 = (next_ext_row[30]) - (current_ext_row[30]);
        let node_1120 = (next_ext_row[31]) - (current_ext_row[31]);
        let node_1133 = (next_ext_row[32]) - (current_ext_row[32]);
        let node_1146 = (next_ext_row[33]) - (current_ext_row[33]);
        let node_1159 = (next_ext_row[34]) - (current_ext_row[34]);
        let node_1172 = (next_ext_row[35]) - (current_ext_row[35]);
        let node_1185 = (next_ext_row[36]) - (current_ext_row[36]);
        let node_1198 = (next_ext_row[37]) - (current_ext_row[37]);
        let node_1211 = (next_ext_row[38]) - (current_ext_row[38]);
        let node_1224 = (next_ext_row[39]) - (current_ext_row[39]);
        let node_1237 = (next_ext_row[40]) - (current_ext_row[40]);

        let base_constraints = [
            ((((((current_base_row[62]) * (node_865)) * (node_868)) * (node_871)) * (node_874))
                * (node_813))
                * (node_882),
            (((((((node_811) * (current_base_row[62])) * (node_865)) * (node_868)) * (node_871))
                * (node_874))
                * (next_base_row[62]))
                * (node_882),
            (node_814) * (((next_base_row[62]) - (current_base_row[62])) - (BFieldElement::new(1))),
            ((((current_base_row[63]) - (BFieldElement::new(72)))
                * ((current_base_row[63]) - (BFieldElement::new(80))))
                * ((current_base_row[63]) - (BFieldElement::new(88))))
                * (node_894),
            (node_813) * ((next_base_row[63]) - (current_base_row[63])),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(61402)) * (node_87))
                    + ((BFieldElement::new(17845)) * (node_98)))
                    + ((BFieldElement::new(26798)) * (node_109)))
                    + ((BFieldElement::new(59689)) * (node_120)))
                    + ((BFieldElement::new(12021)) * (node_181)))
                    + ((BFieldElement::new(40901)) * (node_182)))
                    + ((BFieldElement::new(41351)) * (node_183)))
                    + ((BFieldElement::new(27521)) * (node_184)))
                    + ((BFieldElement::new(56951)) * (node_185)))
                    + ((BFieldElement::new(12034)) * (node_186)))
                    + ((BFieldElement::new(53865)) * (node_187)))
                    + ((BFieldElement::new(43244)) * (node_188)))
                    + ((BFieldElement::new(7454)) * (node_189)))
                    + ((BFieldElement::new(33823)) * (node_190)))
                    + ((BFieldElement::new(28750)) * (node_191)))
                    + ((BFieldElement::new(1108)) * (node_192)))
                    + (current_base_row[112]))
                    - (node_764)),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(1108)) * (node_87))
                    + ((BFieldElement::new(61402)) * (node_98)))
                    + ((BFieldElement::new(17845)) * (node_109)))
                    + ((BFieldElement::new(26798)) * (node_120)))
                    + ((BFieldElement::new(59689)) * (node_181)))
                    + ((BFieldElement::new(12021)) * (node_182)))
                    + ((BFieldElement::new(40901)) * (node_183)))
                    + ((BFieldElement::new(41351)) * (node_184)))
                    + ((BFieldElement::new(27521)) * (node_185)))
                    + ((BFieldElement::new(56951)) * (node_186)))
                    + ((BFieldElement::new(12034)) * (node_187)))
                    + ((BFieldElement::new(53865)) * (node_188)))
                    + ((BFieldElement::new(43244)) * (node_189)))
                    + ((BFieldElement::new(7454)) * (node_190)))
                    + ((BFieldElement::new(33823)) * (node_191)))
                    + ((BFieldElement::new(28750)) * (node_192)))
                    + (current_base_row[113]))
                    - (node_775)),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(28750)) * (node_87))
                    + ((BFieldElement::new(1108)) * (node_98)))
                    + ((BFieldElement::new(61402)) * (node_109)))
                    + ((BFieldElement::new(17845)) * (node_120)))
                    + ((BFieldElement::new(26798)) * (node_181)))
                    + ((BFieldElement::new(59689)) * (node_182)))
                    + ((BFieldElement::new(12021)) * (node_183)))
                    + ((BFieldElement::new(40901)) * (node_184)))
                    + ((BFieldElement::new(41351)) * (node_185)))
                    + ((BFieldElement::new(27521)) * (node_186)))
                    + ((BFieldElement::new(56951)) * (node_187)))
                    + ((BFieldElement::new(12034)) * (node_188)))
                    + ((BFieldElement::new(53865)) * (node_189)))
                    + ((BFieldElement::new(43244)) * (node_190)))
                    + ((BFieldElement::new(7454)) * (node_191)))
                    + ((BFieldElement::new(33823)) * (node_192)))
                    + (current_base_row[114]))
                    - (node_786)),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(33823)) * (node_87))
                    + ((BFieldElement::new(28750)) * (node_98)))
                    + ((BFieldElement::new(1108)) * (node_109)))
                    + ((BFieldElement::new(61402)) * (node_120)))
                    + ((BFieldElement::new(17845)) * (node_181)))
                    + ((BFieldElement::new(26798)) * (node_182)))
                    + ((BFieldElement::new(59689)) * (node_183)))
                    + ((BFieldElement::new(12021)) * (node_184)))
                    + ((BFieldElement::new(40901)) * (node_185)))
                    + ((BFieldElement::new(41351)) * (node_186)))
                    + ((BFieldElement::new(27521)) * (node_187)))
                    + ((BFieldElement::new(56951)) * (node_188)))
                    + ((BFieldElement::new(12034)) * (node_189)))
                    + ((BFieldElement::new(53865)) * (node_190)))
                    + ((BFieldElement::new(43244)) * (node_191)))
                    + ((BFieldElement::new(7454)) * (node_192)))
                    + (current_base_row[115]))
                    - (node_797)),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(7454)) * (node_87))
                    + ((BFieldElement::new(33823)) * (node_98)))
                    + ((BFieldElement::new(28750)) * (node_109)))
                    + ((BFieldElement::new(1108)) * (node_120)))
                    + ((BFieldElement::new(61402)) * (node_181)))
                    + ((BFieldElement::new(17845)) * (node_182)))
                    + ((BFieldElement::new(26798)) * (node_183)))
                    + ((BFieldElement::new(59689)) * (node_184)))
                    + ((BFieldElement::new(12021)) * (node_185)))
                    + ((BFieldElement::new(40901)) * (node_186)))
                    + ((BFieldElement::new(41351)) * (node_187)))
                    + ((BFieldElement::new(27521)) * (node_188)))
                    + ((BFieldElement::new(56951)) * (node_189)))
                    + ((BFieldElement::new(12034)) * (node_190)))
                    + ((BFieldElement::new(53865)) * (node_191)))
                    + ((BFieldElement::new(43244)) * (node_192)))
                    + (current_base_row[116]))
                    - (next_base_row[96])),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(43244)) * (node_87))
                    + ((BFieldElement::new(7454)) * (node_98)))
                    + ((BFieldElement::new(33823)) * (node_109)))
                    + ((BFieldElement::new(28750)) * (node_120)))
                    + ((BFieldElement::new(1108)) * (node_181)))
                    + ((BFieldElement::new(61402)) * (node_182)))
                    + ((BFieldElement::new(17845)) * (node_183)))
                    + ((BFieldElement::new(26798)) * (node_184)))
                    + ((BFieldElement::new(59689)) * (node_185)))
                    + ((BFieldElement::new(12021)) * (node_186)))
                    + ((BFieldElement::new(40901)) * (node_187)))
                    + ((BFieldElement::new(41351)) * (node_188)))
                    + ((BFieldElement::new(27521)) * (node_189)))
                    + ((BFieldElement::new(56951)) * (node_190)))
                    + ((BFieldElement::new(12034)) * (node_191)))
                    + ((BFieldElement::new(53865)) * (node_192)))
                    + (current_base_row[117]))
                    - (next_base_row[97])),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(53865)) * (node_87))
                    + ((BFieldElement::new(43244)) * (node_98)))
                    + ((BFieldElement::new(7454)) * (node_109)))
                    + ((BFieldElement::new(33823)) * (node_120)))
                    + ((BFieldElement::new(28750)) * (node_181)))
                    + ((BFieldElement::new(1108)) * (node_182)))
                    + ((BFieldElement::new(61402)) * (node_183)))
                    + ((BFieldElement::new(17845)) * (node_184)))
                    + ((BFieldElement::new(26798)) * (node_185)))
                    + ((BFieldElement::new(59689)) * (node_186)))
                    + ((BFieldElement::new(12021)) * (node_187)))
                    + ((BFieldElement::new(40901)) * (node_188)))
                    + ((BFieldElement::new(41351)) * (node_189)))
                    + ((BFieldElement::new(27521)) * (node_190)))
                    + ((BFieldElement::new(56951)) * (node_191)))
                    + ((BFieldElement::new(12034)) * (node_192)))
                    + (current_base_row[118]))
                    - (next_base_row[98])),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(12034)) * (node_87))
                    + ((BFieldElement::new(53865)) * (node_98)))
                    + ((BFieldElement::new(43244)) * (node_109)))
                    + ((BFieldElement::new(7454)) * (node_120)))
                    + ((BFieldElement::new(33823)) * (node_181)))
                    + ((BFieldElement::new(28750)) * (node_182)))
                    + ((BFieldElement::new(1108)) * (node_183)))
                    + ((BFieldElement::new(61402)) * (node_184)))
                    + ((BFieldElement::new(17845)) * (node_185)))
                    + ((BFieldElement::new(26798)) * (node_186)))
                    + ((BFieldElement::new(59689)) * (node_187)))
                    + ((BFieldElement::new(12021)) * (node_188)))
                    + ((BFieldElement::new(40901)) * (node_189)))
                    + ((BFieldElement::new(41351)) * (node_190)))
                    + ((BFieldElement::new(27521)) * (node_191)))
                    + ((BFieldElement::new(56951)) * (node_192)))
                    + (current_base_row[119]))
                    - (next_base_row[99])),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(56951)) * (node_87))
                    + ((BFieldElement::new(12034)) * (node_98)))
                    + ((BFieldElement::new(53865)) * (node_109)))
                    + ((BFieldElement::new(43244)) * (node_120)))
                    + ((BFieldElement::new(7454)) * (node_181)))
                    + ((BFieldElement::new(33823)) * (node_182)))
                    + ((BFieldElement::new(28750)) * (node_183)))
                    + ((BFieldElement::new(1108)) * (node_184)))
                    + ((BFieldElement::new(61402)) * (node_185)))
                    + ((BFieldElement::new(17845)) * (node_186)))
                    + ((BFieldElement::new(26798)) * (node_187)))
                    + ((BFieldElement::new(59689)) * (node_188)))
                    + ((BFieldElement::new(12021)) * (node_189)))
                    + ((BFieldElement::new(40901)) * (node_190)))
                    + ((BFieldElement::new(41351)) * (node_191)))
                    + ((BFieldElement::new(27521)) * (node_192)))
                    + (current_base_row[120]))
                    - (next_base_row[100])),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(27521)) * (node_87))
                    + ((BFieldElement::new(56951)) * (node_98)))
                    + ((BFieldElement::new(12034)) * (node_109)))
                    + ((BFieldElement::new(53865)) * (node_120)))
                    + ((BFieldElement::new(43244)) * (node_181)))
                    + ((BFieldElement::new(7454)) * (node_182)))
                    + ((BFieldElement::new(33823)) * (node_183)))
                    + ((BFieldElement::new(28750)) * (node_184)))
                    + ((BFieldElement::new(1108)) * (node_185)))
                    + ((BFieldElement::new(61402)) * (node_186)))
                    + ((BFieldElement::new(17845)) * (node_187)))
                    + ((BFieldElement::new(26798)) * (node_188)))
                    + ((BFieldElement::new(59689)) * (node_189)))
                    + ((BFieldElement::new(12021)) * (node_190)))
                    + ((BFieldElement::new(40901)) * (node_191)))
                    + ((BFieldElement::new(41351)) * (node_192)))
                    + (current_base_row[121]))
                    - (next_base_row[101])),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(41351)) * (node_87))
                    + ((BFieldElement::new(27521)) * (node_98)))
                    + ((BFieldElement::new(56951)) * (node_109)))
                    + ((BFieldElement::new(12034)) * (node_120)))
                    + ((BFieldElement::new(53865)) * (node_181)))
                    + ((BFieldElement::new(43244)) * (node_182)))
                    + ((BFieldElement::new(7454)) * (node_183)))
                    + ((BFieldElement::new(33823)) * (node_184)))
                    + ((BFieldElement::new(28750)) * (node_185)))
                    + ((BFieldElement::new(1108)) * (node_186)))
                    + ((BFieldElement::new(61402)) * (node_187)))
                    + ((BFieldElement::new(17845)) * (node_188)))
                    + ((BFieldElement::new(26798)) * (node_189)))
                    + ((BFieldElement::new(59689)) * (node_190)))
                    + ((BFieldElement::new(12021)) * (node_191)))
                    + ((BFieldElement::new(40901)) * (node_192)))
                    + (current_base_row[122]))
                    - (next_base_row[102])),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(40901)) * (node_87))
                    + ((BFieldElement::new(41351)) * (node_98)))
                    + ((BFieldElement::new(27521)) * (node_109)))
                    + ((BFieldElement::new(56951)) * (node_120)))
                    + ((BFieldElement::new(12034)) * (node_181)))
                    + ((BFieldElement::new(53865)) * (node_182)))
                    + ((BFieldElement::new(43244)) * (node_183)))
                    + ((BFieldElement::new(7454)) * (node_184)))
                    + ((BFieldElement::new(33823)) * (node_185)))
                    + ((BFieldElement::new(28750)) * (node_186)))
                    + ((BFieldElement::new(1108)) * (node_187)))
                    + ((BFieldElement::new(61402)) * (node_188)))
                    + ((BFieldElement::new(17845)) * (node_189)))
                    + ((BFieldElement::new(26798)) * (node_190)))
                    + ((BFieldElement::new(59689)) * (node_191)))
                    + ((BFieldElement::new(12021)) * (node_192)))
                    + (current_base_row[123]))
                    - (next_base_row[103])),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(12021)) * (node_87))
                    + ((BFieldElement::new(40901)) * (node_98)))
                    + ((BFieldElement::new(41351)) * (node_109)))
                    + ((BFieldElement::new(27521)) * (node_120)))
                    + ((BFieldElement::new(56951)) * (node_181)))
                    + ((BFieldElement::new(12034)) * (node_182)))
                    + ((BFieldElement::new(53865)) * (node_183)))
                    + ((BFieldElement::new(43244)) * (node_184)))
                    + ((BFieldElement::new(7454)) * (node_185)))
                    + ((BFieldElement::new(33823)) * (node_186)))
                    + ((BFieldElement::new(28750)) * (node_187)))
                    + ((BFieldElement::new(1108)) * (node_188)))
                    + ((BFieldElement::new(61402)) * (node_189)))
                    + ((BFieldElement::new(17845)) * (node_190)))
                    + ((BFieldElement::new(26798)) * (node_191)))
                    + ((BFieldElement::new(59689)) * (node_192)))
                    + (current_base_row[124]))
                    - (next_base_row[104])),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(59689)) * (node_87))
                    + ((BFieldElement::new(12021)) * (node_98)))
                    + ((BFieldElement::new(40901)) * (node_109)))
                    + ((BFieldElement::new(41351)) * (node_120)))
                    + ((BFieldElement::new(27521)) * (node_181)))
                    + ((BFieldElement::new(56951)) * (node_182)))
                    + ((BFieldElement::new(12034)) * (node_183)))
                    + ((BFieldElement::new(53865)) * (node_184)))
                    + ((BFieldElement::new(43244)) * (node_185)))
                    + ((BFieldElement::new(7454)) * (node_186)))
                    + ((BFieldElement::new(33823)) * (node_187)))
                    + ((BFieldElement::new(28750)) * (node_188)))
                    + ((BFieldElement::new(1108)) * (node_189)))
                    + ((BFieldElement::new(61402)) * (node_190)))
                    + ((BFieldElement::new(17845)) * (node_191)))
                    + ((BFieldElement::new(26798)) * (node_192)))
                    + (current_base_row[125]))
                    - (next_base_row[105])),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(26798)) * (node_87))
                    + ((BFieldElement::new(59689)) * (node_98)))
                    + ((BFieldElement::new(12021)) * (node_109)))
                    + ((BFieldElement::new(40901)) * (node_120)))
                    + ((BFieldElement::new(41351)) * (node_181)))
                    + ((BFieldElement::new(27521)) * (node_182)))
                    + ((BFieldElement::new(56951)) * (node_183)))
                    + ((BFieldElement::new(12034)) * (node_184)))
                    + ((BFieldElement::new(53865)) * (node_185)))
                    + ((BFieldElement::new(43244)) * (node_186)))
                    + ((BFieldElement::new(7454)) * (node_187)))
                    + ((BFieldElement::new(33823)) * (node_188)))
                    + ((BFieldElement::new(28750)) * (node_189)))
                    + ((BFieldElement::new(1108)) * (node_190)))
                    + ((BFieldElement::new(61402)) * (node_191)))
                    + ((BFieldElement::new(17845)) * (node_192)))
                    + (current_base_row[126]))
                    - (next_base_row[106])),
            (node_814)
                * (((((((((((((((((((BFieldElement::new(17845)) * (node_87))
                    + ((BFieldElement::new(26798)) * (node_98)))
                    + ((BFieldElement::new(59689)) * (node_109)))
                    + ((BFieldElement::new(12021)) * (node_120)))
                    + ((BFieldElement::new(40901)) * (node_181)))
                    + ((BFieldElement::new(41351)) * (node_182)))
                    + ((BFieldElement::new(27521)) * (node_183)))
                    + ((BFieldElement::new(56951)) * (node_184)))
                    + ((BFieldElement::new(12034)) * (node_185)))
                    + ((BFieldElement::new(53865)) * (node_186)))
                    + ((BFieldElement::new(43244)) * (node_187)))
                    + ((BFieldElement::new(7454)) * (node_188)))
                    + ((BFieldElement::new(33823)) * (node_189)))
                    + ((BFieldElement::new(28750)) * (node_190)))
                    + ((BFieldElement::new(1108)) * (node_191)))
                    + ((BFieldElement::new(61402)) * (node_192)))
                    + (current_base_row[127]))
                    - (next_base_row[107])),
        ];
        let ext_constraints = [
            ((node_927) * (node_928))
                * ((((((node_914) + (node_915)) + (node_917)) + (node_919)) + (node_921))
                    + (node_923)),
            ((node_927) * (node_966))
                * (((((((((((((((((challenges.get_challenge(HashStateWeight0))
                    * ((node_764)
                        - ((((((current_base_row[64])
                            * (BFieldElement::new(281474976710656)))
                            + ((current_base_row[65])
                                * (BFieldElement::new(4294967296))))
                            + ((current_base_row[66])
                                * (BFieldElement::new(65536))))
                            + (current_base_row[67]))
                            * (BFieldElement::new(18446744065119617025)))))
                    + ((challenges.get_challenge(HashStateWeight1))
                        * ((node_775)
                            - ((((((current_base_row[68])
                                * (BFieldElement::new(281474976710656)))
                                + ((current_base_row[69])
                                    * (BFieldElement::new(4294967296))))
                                + ((current_base_row[70])
                                    * (BFieldElement::new(65536))))
                                + (current_base_row[71]))
                                * (BFieldElement::new(18446744065119617025))))))
                    + ((challenges.get_challenge(HashStateWeight2))
                        * ((node_786)
                            - ((((((current_base_row[72])
                                * (BFieldElement::new(281474976710656)))
                                + ((current_base_row[73])
                                    * (BFieldElement::new(4294967296))))
                                + ((current_base_row[74])
                                    * (BFieldElement::new(65536))))
                                + (current_base_row[75]))
                                * (BFieldElement::new(18446744065119617025))))))
                    + ((challenges.get_challenge(HashStateWeight3))
                        * ((node_797)
                            - ((((((current_base_row[76])
                                * (BFieldElement::new(281474976710656)))
                                + ((current_base_row[77])
                                    * (BFieldElement::new(4294967296))))
                                + ((current_base_row[78])
                                    * (BFieldElement::new(65536))))
                                + (current_base_row[79]))
                                * (BFieldElement::new(18446744065119617025))))))
                    + ((challenges.get_challenge(HashStateWeight4))
                        * ((next_base_row[96]) - (current_base_row[96]))))
                    + ((challenges.get_challenge(HashStateWeight5))
                        * ((next_base_row[97]) - (current_base_row[97]))))
                    + ((challenges.get_challenge(HashStateWeight6))
                        * ((next_base_row[98]) - (current_base_row[98]))))
                    + ((challenges.get_challenge(HashStateWeight7))
                        * ((next_base_row[99]) - (current_base_row[99]))))
                    + ((challenges.get_challenge(HashStateWeight8))
                        * ((next_base_row[100]) - (current_base_row[100]))))
                    + ((challenges.get_challenge(HashStateWeight9))
                        * ((next_base_row[101]) - (current_base_row[101]))))
                    + (node_914))
                    + (node_915))
                    + (node_917))
                    + (node_919))
                    + (node_921))
                    + (node_923)),
            ((((node_907) * (node_970))
                * (((next_ext_row[22])
                    - ((challenges.get_challenge(HashInputIndeterminate))
                        * (current_ext_row[22])))
                    - (node_990)))
                + ((next_base_row[62]) * (node_971)))
                + ((node_894) * (node_971)),
            (((((((((node_882) * (next_base_row[62])) * (node_898)) * (node_900)) * (node_902))
                * (node_904))
                * (node_970))
                * (((next_ext_row[23])
                    - ((challenges.get_challenge(HashDigestIndeterminate))
                        * (current_ext_row[23])))
                    - (node_980)))
                + ((node_906) * (node_1006)))
                + ((node_894) * (node_1006)),
            (((node_925)
                * ((((next_ext_row[24])
                    - ((challenges.get_challenge(SpongeIndeterminate))
                        * (current_ext_row[24])))
                    - ((challenges.get_challenge(HashCIWeight)) * (next_base_row[63])))
                    - (node_990)))
                + ((next_base_row[62]) * (node_1023)))
                + ((node_970) * (node_1023)),
            ((node_1041)
                * (((node_1037)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[64]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[80])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1037)),
            ((node_1041)
                * (((node_1055)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[65]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[81])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1055)),
            ((node_1041)
                * (((node_1068)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[66]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[82])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1068)),
            ((node_1041)
                * (((node_1081)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[67]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[83])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1081)),
            ((node_1041)
                * (((node_1094)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[68]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[84])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1094)),
            ((node_1041)
                * (((node_1107)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[69]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[85])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1107)),
            ((node_1041)
                * (((node_1120)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[70]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[86])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1120)),
            ((node_1041)
                * (((node_1133)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[71]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[87])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1133)),
            ((node_1041)
                * (((node_1146)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[72]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[88])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1146)),
            ((node_1041)
                * (((node_1159)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[73]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[89])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1159)),
            ((node_1041)
                * (((node_1172)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[74]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[90])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1172)),
            ((node_1041)
                * (((node_1185)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[75]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[91])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1185)),
            ((node_1041)
                * (((node_1198)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[76]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[92])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1198)),
            ((node_1041)
                * (((node_1211)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[77]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[93])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1211)),
            ((node_1041)
                * (((node_1224)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[78]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[94])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1224)),
            ((node_1041)
                * (((node_1237)
                    * ((challenges.get_challenge(HashCascadeLookupIndeterminate))
                        - (((challenges.get_challenge(HashCascadeLookInWeight))
                            * (next_base_row[79]))
                            + ((challenges.get_challenge(HashCascadeLookOutWeight))
                                * (next_base_row[95])))))
                    - (BFieldElement::new(1))))
                + ((node_1045) * (node_1237)),
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

impl Quotientable for ExtHashTable {
    fn num_initial_quotients() -> usize {
        21
    }

    fn num_consistency_quotients() -> usize {
        41
    }

    fn num_transition_quotients() -> usize {
        42
    }

    fn num_terminal_quotients() -> usize {
        0
    }

    #[allow(unused_variables)]
    fn initial_quotient_degree_bounds(interpolant_degree: Degree) -> Vec<Degree> {
        let zerofier_degree = 1 as Degree;
        [
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 1 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
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
            interpolant_degree * 7 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 7 as Degree - zerofier_degree,
            interpolant_degree * 7 as Degree - zerofier_degree,
            interpolant_degree * 7 as Degree - zerofier_degree,
            interpolant_degree * 7 as Degree - zerofier_degree,
            interpolant_degree * 7 as Degree - zerofier_degree,
            interpolant_degree * 7 as Degree - zerofier_degree,
            interpolant_degree * 7 as Degree - zerofier_degree,
            interpolant_degree * 7 as Degree - zerofier_degree,
            interpolant_degree * 7 as Degree - zerofier_degree,
            interpolant_degree * 7 as Degree - zerofier_degree,
            interpolant_degree * 7 as Degree - zerofier_degree,
            interpolant_degree * 7 as Degree - zerofier_degree,
            interpolant_degree * 7 as Degree - zerofier_degree,
            interpolant_degree * 7 as Degree - zerofier_degree,
            interpolant_degree * 7 as Degree - zerofier_degree,
            interpolant_degree * 7 as Degree - zerofier_degree,
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
            interpolant_degree * 7 as Degree - zerofier_degree,
            interpolant_degree * 8 as Degree - zerofier_degree,
            interpolant_degree * 3 as Degree - zerofier_degree,
            interpolant_degree * 4 as Degree - zerofier_degree,
            interpolant_degree * 2 as Degree - zerofier_degree,
            interpolant_degree * 9 as Degree - zerofier_degree,
            interpolant_degree * 9 as Degree - zerofier_degree,
            interpolant_degree * 9 as Degree - zerofier_degree,
            interpolant_degree * 9 as Degree - zerofier_degree,
            interpolant_degree * 9 as Degree - zerofier_degree,
            interpolant_degree * 9 as Degree - zerofier_degree,
            interpolant_degree * 9 as Degree - zerofier_degree,
            interpolant_degree * 9 as Degree - zerofier_degree,
            interpolant_degree * 9 as Degree - zerofier_degree,
            interpolant_degree * 9 as Degree - zerofier_degree,
            interpolant_degree * 9 as Degree - zerofier_degree,
            interpolant_degree * 9 as Degree - zerofier_degree,
            interpolant_degree * 9 as Degree - zerofier_degree,
            interpolant_degree * 9 as Degree - zerofier_degree,
            interpolant_degree * 9 as Degree - zerofier_degree,
            interpolant_degree * 9 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 10 as Degree - zerofier_degree,
            interpolant_degree * 8 as Degree - zerofier_degree,
            interpolant_degree * 6 as Degree - zerofier_degree,
            interpolant_degree * 6 as Degree - zerofier_degree,
            interpolant_degree * 6 as Degree - zerofier_degree,
            interpolant_degree * 6 as Degree - zerofier_degree,
            interpolant_degree * 6 as Degree - zerofier_degree,
            interpolant_degree * 6 as Degree - zerofier_degree,
            interpolant_degree * 6 as Degree - zerofier_degree,
            interpolant_degree * 6 as Degree - zerofier_degree,
            interpolant_degree * 6 as Degree - zerofier_degree,
            interpolant_degree * 6 as Degree - zerofier_degree,
            interpolant_degree * 6 as Degree - zerofier_degree,
            interpolant_degree * 6 as Degree - zerofier_degree,
            interpolant_degree * 6 as Degree - zerofier_degree,
            interpolant_degree * 6 as Degree - zerofier_degree,
            interpolant_degree * 6 as Degree - zerofier_degree,
            interpolant_degree * 6 as Degree - zerofier_degree,
        ]
        .to_vec()
    }

    #[allow(unused_variables)]
    fn terminal_quotient_degree_bounds(interpolant_degree: Degree) -> Vec<Degree> {
        let zerofier_degree = 1 as Degree;
        [].to_vec()
    }
}
