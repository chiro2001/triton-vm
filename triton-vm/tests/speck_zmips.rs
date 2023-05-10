use triton_profiler::triton_profiler::{Report, TritonProfiler};
use triton_profiler::{prof_start, prof_stop};
use triton_program::AbstractProgram;
use triton_vm::shared_tests::SPECK128_ZMIPS;
use triton_vm::stark::Stark;
use triton_vm::table::master_table::MasterBaseTable;
use triton_vm::vm::simulate;
use triton_vm::{Claim, StarkParameters};
use triton_zmips::program::Program;
use twenty_first::shared_math::b_field_element::BFieldElement;
use twenty_first::shared_math::tip5::Tip5;
use twenty_first::util_types::algebraic_hasher::AlgebraicHasher;

#[test]
fn speck() {
    let mut maybe_profiler = Some(TritonProfiler::new("Speck 128 zMIPS"));
    let mut report: Report = Report::placeholder();

    // stark object
    prof_start!(maybe_profiler, "parse program");
    let program = match Program::from_code(SPECK128_ZMIPS) {
        Err(e) => panic!("Cannot compile source code into program: {e}"),
        Ok(p) => Box::new(p) as Box<dyn AbstractProgram>,
    };
    prof_stop!(maybe_profiler, "parse program");
    let input = vec![100];
    let public_input = input.iter().map(|&e| BFieldElement::new(e)).collect();
    prof_start!(maybe_profiler, "generate AET");
    let (aet, output, err) = simulate(program.clone(), public_input, vec![]);
    prof_stop!(maybe_profiler, "generate AET");
    if let Some(error) = err {
        panic!("The VM encountered the following problem: {error}");
    }

    let output = output.iter().map(|x| x.value()).collect();
    let padded_height = MasterBaseTable::padded_height(&aet);
    let claim = Claim {
        input,
        program_digest: Tip5::hash(&program),
        output,
        padded_height,
    };
    let parameters = StarkParameters::default();
    let _proof = Stark::prove(&parameters, &claim, &aet, &mut maybe_profiler);

    let max_degree =
        Stark::derive_max_degree(claim.padded_height, parameters.num_trace_randomizers);
    let fri = Stark::derive_fri(&parameters, max_degree);

    if let Some(profiler) = maybe_profiler.as_mut() {
        profiler.finish();
        report = profiler.report(
            Some(aet.processor_trace.nrows()),
            Some(claim.padded_height),
            Some(fri.domain.length),
        );
    }
    Stark::prove(&parameters, &claim, &aet, &mut None);
    println!("Writing report ...");
    println!("{report}");
}
