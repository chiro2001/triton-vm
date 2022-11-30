use std::fmt::Display;

use itertools::Itertools;
use ndarray::{s, ArrayView1, ArrayView2};
use num_traits::One;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};
use twenty_first::shared_math::b_field_element::BFieldElement;
use twenty_first::shared_math::mpolynomial::Degree;
use twenty_first::shared_math::other::transpose;
use twenty_first::shared_math::traits::{FiniteField, Inverse, ModPowU32};
use twenty_first::shared_math::x_field_element::XFieldElement;

use triton_profiler::triton_profiler::TritonProfiler;
use triton_profiler::{prof_start, prof_stop};

use crate::arithmetic_domain::ArithmeticDomain;
use crate::table::table_collection::interpolant_degree;

use super::base_table::TableLike;
use super::challenges::AllChallenges;

// Generic methods specifically for tables that have been extended

pub trait ExtensionTable: TableLike<XFieldElement> + Sync {}

const ERROR_MESSAGE_GENERATE_CONSTRAINTS: &str =
    "Constraints must be in place. Run: `cargo run --bin constraint-evaluation-generator`";
const ERROR_MESSAGE_GENERATE_DEGREE_BOUNDS: &str =
    "Degree bounds must be in place. Run: `cargo run --bin constraint-evaluation-generator`";

pub trait Evaluable: ExtensionTable {
    /// The code for this method must be generated by running
    /// `cargo run --bin constraint-evaluation-generator`
    fn evaluate_initial_constraints(
        _base_row: ArrayView1<BFieldElement>,
        _ext_row: ArrayView1<XFieldElement>,
        _challenges: &AllChallenges,
    ) -> Vec<XFieldElement> {
        panic!("{ERROR_MESSAGE_GENERATE_CONSTRAINTS}")
    }

    /// The code for this method must be generated by running
    /// `cargo run --bin constraint-evaluation-generator`
    fn evaluate_consistency_constraints(
        _base_row: ArrayView1<BFieldElement>,
        _ext_row: ArrayView1<XFieldElement>,
        _challenges: &AllChallenges,
    ) -> Vec<XFieldElement> {
        panic!("{ERROR_MESSAGE_GENERATE_CONSTRAINTS}")
    }

    /// The code for this method must be generated by running
    /// `cargo run --bin constraint-evaluation-generator`
    fn evaluate_transition_constraints(
        _current_base_row: ArrayView1<BFieldElement>,
        _current_ext_row: ArrayView1<XFieldElement>,
        _next_base_row: ArrayView1<BFieldElement>,
        _next_ext_row: ArrayView1<XFieldElement>,
        _challenges: &AllChallenges,
    ) -> Vec<XFieldElement> {
        panic!("{ERROR_MESSAGE_GENERATE_CONSTRAINTS}")
    }

    /// The code for this method must be generated by running
    /// `cargo run --bin constraint-evaluation-generator`
    fn evaluate_terminal_constraints(
        _base_row: ArrayView1<BFieldElement>,
        _ext_row: ArrayView1<XFieldElement>,
        _challenges: &AllChallenges,
    ) -> Vec<XFieldElement> {
        panic!("{ERROR_MESSAGE_GENERATE_CONSTRAINTS}")
    }
}

pub trait Quotientable: ExtensionTable + Evaluable {
    /// Compute the degrees of the quotients from all AIR constraints that apply to the table.
    fn all_degrees_with_origin(
        table_name: &str,
        padded_height: usize,
        num_trace_randomizers: usize,
    ) -> Vec<DegreeWithOrigin> {
        let initial_degrees_with_origin =
            Self::initial_quotient_degree_bounds(padded_height, num_trace_randomizers)
                .into_iter()
                .enumerate()
                .map(|(i, d)| DegreeWithOrigin {
                    degree: d,
                    zerofier_degree: 1,
                    origin_table_name: table_name.to_owned(),
                    origin_index: i,
                    origin_table_height: padded_height,
                    origin_num_trace_randomizers: num_trace_randomizers,
                    origin_constraint_type: "initial constraint".to_string(),
                })
                .collect_vec();

        let consistency_degrees_with_origin =
            Self::consistency_quotient_degree_bounds(padded_height, num_trace_randomizers)
                .into_iter()
                .enumerate()
                .map(|(i, d)| DegreeWithOrigin {
                    degree: d,
                    zerofier_degree: padded_height as Degree,
                    origin_table_name: table_name.to_owned(),
                    origin_index: i,
                    origin_table_height: padded_height,
                    origin_num_trace_randomizers: num_trace_randomizers,
                    origin_constraint_type: "consistency constraint".to_string(),
                })
                .collect();

        let transition_degrees_with_origin =
            Self::transition_quotient_degree_bounds(padded_height, num_trace_randomizers)
                .into_iter()
                .enumerate()
                .map(|(i, d)| DegreeWithOrigin {
                    degree: d,
                    zerofier_degree: padded_height as Degree - 1,
                    origin_table_name: table_name.to_owned(),
                    origin_index: i,
                    origin_table_height: padded_height,
                    origin_num_trace_randomizers: num_trace_randomizers,
                    origin_constraint_type: "transition constraint".to_string(),
                })
                .collect();

        let terminal_degrees_with_origin =
            Self::terminal_quotient_degree_bounds(padded_height, num_trace_randomizers)
                .into_iter()
                .enumerate()
                .map(|(i, d)| DegreeWithOrigin {
                    degree: d,
                    zerofier_degree: 1,
                    origin_table_name: table_name.to_owned(),
                    origin_index: i,
                    origin_table_height: padded_height,
                    origin_num_trace_randomizers: num_trace_randomizers,
                    origin_constraint_type: "terminal constraint".to_string(),
                })
                .collect();

        [
            initial_degrees_with_origin,
            consistency_degrees_with_origin,
            transition_degrees_with_origin,
            terminal_degrees_with_origin,
        ]
        .concat()
    }

    fn initial_quotients(
        master_base_table: ArrayView2<BFieldElement>,
        master_ext_table: ArrayView2<XFieldElement>,
        quotient_domain: ArithmeticDomain,
        challenges: &AllChallenges,
    ) -> Vec<Vec<XFieldElement>> {
        debug_assert_eq!(quotient_domain.length, master_base_table.nrows());
        debug_assert_eq!(quotient_domain.length, master_ext_table.nrows());

        let zerofier_codeword = quotient_domain
            .domain_values()
            .into_iter()
            .map(|x| x - BFieldElement::one())
            .collect();
        let zerofier_inverse = BFieldElement::batch_inversion(zerofier_codeword);

        let transposed_quotient_codewords: Vec<_> = zerofier_inverse
            .par_iter()
            .enumerate()
            .map(|(domain_index, &z_inv)| {
                let base_row = master_base_table.slice(s![domain_index, ..]);
                let ext_row = master_ext_table.slice(s![domain_index, ..]);
                let evaluated_bcs =
                    Self::evaluate_initial_constraints(base_row, ext_row, challenges);
                evaluated_bcs.iter().map(|&ebc| ebc * z_inv).collect()
            })
            .collect();
        transpose(&transposed_quotient_codewords)
    }

    fn consistency_quotients(
        master_base_table: ArrayView2<BFieldElement>,
        master_ext_table: ArrayView2<XFieldElement>,
        quotient_domain: ArithmeticDomain,
        challenges: &AllChallenges,
        padded_height: usize,
    ) -> Vec<Vec<XFieldElement>> {
        debug_assert_eq!(quotient_domain.length, master_base_table.nrows());
        debug_assert_eq!(quotient_domain.length, master_ext_table.nrows());

        let zerofier_codeword = quotient_domain
            .domain_values()
            .iter()
            .map(|x| x.mod_pow_u32(padded_height as u32) - BFieldElement::one())
            .collect();
        let zerofier_inverse = BFieldElement::batch_inversion(zerofier_codeword);

        let transposed_quotient_codewords: Vec<_> = zerofier_inverse
            .par_iter()
            .enumerate()
            .map(|(domain_index, &z_inv)| {
                let base_row = master_base_table.slice(s![domain_index, ..]);
                let ext_row = master_ext_table.slice(s![domain_index, ..]);
                let evaluated_ccs =
                    Self::evaluate_consistency_constraints(base_row, ext_row, challenges);
                evaluated_ccs.iter().map(|&ecc| ecc * z_inv).collect()
            })
            .collect();
        transpose(&transposed_quotient_codewords)
    }

    fn transition_quotients(
        master_base_table: ArrayView2<BFieldElement>,
        master_ext_table: ArrayView2<XFieldElement>,
        quotient_domain: ArithmeticDomain,
        challenges: &AllChallenges,
        trace_domain_generator: BFieldElement,
        padded_height: usize,
    ) -> Vec<Vec<XFieldElement>> {
        debug_assert_eq!(quotient_domain.length, master_base_table.nrows());
        debug_assert_eq!(quotient_domain.length, master_ext_table.nrows());

        let one = XFieldElement::one();
        let trace_domain_generator_inverse = trace_domain_generator.inverse();
        let domain_values = quotient_domain.domain_values();

        let subgroup_zerofier: Vec<_> = domain_values
            .par_iter()
            .map(|domain_value| domain_value.mod_pow_u32(padded_height as u32) - one)
            .collect();
        let subgroup_zerofier_inverse = XFieldElement::batch_inversion(subgroup_zerofier);
        let zerofier_inverse: Vec<_> = domain_values
            .into_par_iter()
            .zip_eq(subgroup_zerofier_inverse.into_par_iter())
            .map(|(domain_value, sub_z_inv)| {
                (domain_value - trace_domain_generator_inverse) * sub_z_inv
            })
            .collect();
        // the relation between the quotient domain and the trace domain
        let unit_distance = quotient_domain.length / padded_height;

        let domain_length_bit_mask = quotient_domain.length - 1;
        let transposed_quotient_codewords: Vec<_> = zerofier_inverse
            .par_iter()
            .enumerate()
            .map(|(current_row_index, &z_inv)| {
                // `&domain_length_bit_mask` performs the modulo operation cheaply:
                // `domain.length - 1` is a bit-mask with all 1s because `domain.length` is 2^k
                // for some k.
                let next_row_index = (current_row_index + unit_distance) & domain_length_bit_mask;
                let current_base_row = master_base_table.slice(s![current_row_index, ..]);
                let current_ext_row = master_ext_table.slice(s![current_row_index, ..]);
                let next_base_row = master_base_table.slice(s![next_row_index, ..]);
                let next_ext_row = master_ext_table.slice(s![next_row_index, ..]);
                let evaluated_tcs = Self::evaluate_transition_constraints(
                    current_base_row,
                    current_ext_row,
                    next_base_row,
                    next_ext_row,
                    challenges,
                );
                evaluated_tcs.iter().map(|&etc| etc * z_inv).collect()
            })
            .collect();
        transpose(&transposed_quotient_codewords)
    }

    fn terminal_quotients(
        master_base_table: ArrayView2<BFieldElement>,
        master_ext_table: ArrayView2<XFieldElement>,
        quotient_domain: ArithmeticDomain,
        challenges: &AllChallenges,
        trace_domain_generator: BFieldElement,
    ) -> Vec<Vec<XFieldElement>> {
        debug_assert_eq!(quotient_domain.length, master_base_table.nrows());
        debug_assert_eq!(quotient_domain.length, master_ext_table.nrows());

        // The zerofier for the terminal quotient has a root in the last
        // value in the cyclical group generated from the trace domain's generator.
        let zerofier_codeword = quotient_domain
            .domain_values()
            .into_iter()
            .map(|x| x - trace_domain_generator.inverse())
            .collect_vec();
        let zerofier_inverse = BFieldElement::batch_inversion(zerofier_codeword);

        let transposed_quotient_codewords: Vec<_> = zerofier_inverse
            .par_iter()
            .enumerate()
            .map(|(domain_index, &z_inv)| {
                let base_row = master_base_table.slice(s![domain_index, ..]);
                let ext_row = master_ext_table.slice(s![domain_index, ..]);
                let evaluated_termcs =
                    Self::evaluate_terminal_constraints(base_row, ext_row, challenges);
                evaluated_termcs.iter().map(|&etc| etc * z_inv).collect()
            })
            .collect();
        transpose(&transposed_quotient_codewords)
    }

    // todo get quotient degree bounds by type, not by table
    fn all_quotients(
        master_base_table: ArrayView2<BFieldElement>,
        master_ext_table: ArrayView2<XFieldElement>,
        quotient_domain: ArithmeticDomain,
        challenges: &AllChallenges,
        trace_domain_generator: BFieldElement,
        padded_height: usize,
        maybe_profiler: &mut Option<TritonProfiler>,
    ) -> Vec<Vec<XFieldElement>> {
        prof_start!(maybe_profiler, "initial quotients");
        let initial_quotients = Self::initial_quotients(
            master_base_table,
            master_ext_table,
            quotient_domain,
            challenges,
        );
        prof_stop!(maybe_profiler, "initial quotients");

        prof_start!(maybe_profiler, "consistency quotients");
        let consistency_quotients = Self::consistency_quotients(
            master_base_table,
            master_ext_table,
            quotient_domain,
            challenges,
            padded_height,
        );
        prof_stop!(maybe_profiler, "consistency quotients");

        prof_start!(maybe_profiler, "transition quotients");
        let transition_quotients = Self::transition_quotients(
            master_base_table,
            master_ext_table,
            quotient_domain,
            challenges,
            trace_domain_generator,
            padded_height,
        );
        prof_stop!(maybe_profiler, "transition quotients");

        prof_start!(maybe_profiler, "terminal quotients");
        let terminal_quotients = Self::terminal_quotients(
            master_base_table,
            master_ext_table,
            quotient_domain,
            challenges,
            trace_domain_generator,
        );
        prof_stop!(maybe_profiler, "terminal quotients");

        vec![
            initial_quotients,
            consistency_quotients,
            transition_quotients,
            terminal_quotients,
        ]
        .concat()
    }

    // todo get quotient degree bounds by type, not by table
    fn all_quotient_degree_bounds(
        padded_height: usize,
        num_trace_randomizers: usize,
    ) -> Vec<Degree> {
        vec![
            Self::initial_quotient_degree_bounds(padded_height, num_trace_randomizers),
            Self::consistency_quotient_degree_bounds(padded_height, num_trace_randomizers),
            Self::transition_quotient_degree_bounds(padded_height, num_trace_randomizers),
            Self::terminal_quotient_degree_bounds(padded_height, num_trace_randomizers),
        ]
        .concat()
    }

    fn initial_quotient_degree_bounds(
        _padded_height: usize,
        _num_trace_randomizers: usize,
    ) -> Vec<Degree> {
        panic!("{ERROR_MESSAGE_GENERATE_DEGREE_BOUNDS}")
    }

    fn consistency_quotient_degree_bounds(
        _padded_height: usize,
        _num_trace_randomizers: usize,
    ) -> Vec<Degree> {
        panic!("{ERROR_MESSAGE_GENERATE_DEGREE_BOUNDS}")
    }

    fn transition_quotient_degree_bounds(
        _padded_height: usize,
        _num_trace_randomizers: usize,
    ) -> Vec<Degree> {
        panic!("{ERROR_MESSAGE_GENERATE_DEGREE_BOUNDS}")
    }

    fn terminal_quotient_degree_bounds(
        _padded_height: usize,
        _num_trace_randomizers: usize,
    ) -> Vec<Degree> {
        panic!("{ERROR_MESSAGE_GENERATE_DEGREE_BOUNDS}")
    }
}
pub trait QuotientableExtensionTable: ExtensionTable + Quotientable {}

/// Helps debugging and benchmarking. The maximal degree achieved in any table dictates the length
/// of the FRI domain, which in turn is responsible for the main performance bottleneck.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct DegreeWithOrigin {
    pub degree: Degree,
    pub zerofier_degree: Degree,
    pub origin_table_name: String,
    pub origin_index: usize,
    pub origin_table_height: usize,
    pub origin_num_trace_randomizers: usize,
    pub origin_constraint_type: String,
}

impl Default for DegreeWithOrigin {
    fn default() -> Self {
        DegreeWithOrigin {
            degree: -1,
            zerofier_degree: -1,
            origin_table_name: "NoTable".to_string(),
            origin_index: usize::MAX,
            origin_table_height: 0,
            origin_num_trace_randomizers: 0,
            origin_constraint_type: "NoType".to_string(),
        }
    }
}

impl Display for DegreeWithOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let interpolant_degree =
            interpolant_degree(self.origin_table_height, self.origin_num_trace_randomizers);
        let zerofier_corrected_degree = self.degree + self.zerofier_degree;
        assert_eq!(0, zerofier_corrected_degree % interpolant_degree); // todo: wtf – in Display?!
        let degree = zerofier_corrected_degree / interpolant_degree as Degree;
        write!(
            f,
            "Degree of poly for table {} (index {:02}) of type {} is {}.",
            self.origin_table_name, self.origin_index, self.origin_constraint_type, degree,
        )
    }
}
