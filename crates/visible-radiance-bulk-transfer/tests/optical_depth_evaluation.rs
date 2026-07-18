use visible_radiance_bulk_transfer::{
    BulkOpticalDepthEvaluationInputV1, FixedU128V1, TRANSMISSION_ONE_Q0_48,
    compile_bulk_optical_depth_evaluation, validate_bulk_optical_depth_evaluation,
};

fn input(lower: u128, upper: u128) -> BulkOpticalDepthEvaluationInputV1 {
    BulkOpticalDepthEvaluationInputV1 {
        schema_version: 1,
        optical_depth_lower_q64_64: FixedU128V1::from_u128(lower),
        optical_depth_upper_q64_64: FixedU128V1::from_u128(upper),
    }
}

#[test]
fn zero_depth_is_exact_identity_and_round_trips() {
    let input = input(0, 0);
    let result = compile_bulk_optical_depth_evaluation(&input).expect("zero depth compiles");
    assert_eq!(result.optical_depth_lower_q64_64.to_u128(), 0);
    assert_eq!(result.optical_depth_upper_q64_64.to_u128(), 0);
    assert_eq!(result.transfer_lower_q0_48, TRANSMISSION_ONE_Q0_48);
    assert_eq!(result.transfer_upper_q0_48, TRANSMISSION_ONE_Q0_48);
    assert_eq!(result.arithmetic_receipt.exponential_kernel_calls, 2);
    assert_eq!(result.authority_effect, "none_evidence_only");

    let input_bytes = input.to_bytes().expect("input encodes");
    assert_eq!(
        BulkOpticalDepthEvaluationInputV1::from_bytes(&input_bytes).expect("input decodes"),
        input
    );
    let result_bytes = result.to_bytes(&input).expect("result encodes");
    assert_eq!(
        visible_radiance_bulk_transfer::BulkOpticalDepthEvaluationV1::from_bytes(
            &result_bytes,
            &input,
        )
        .expect("result decodes"),
        result
    );
}

#[test]
fn finite_interval_is_directed_and_identity_bound() {
    let one = 1_u128 << 64;
    let input = input(one, 2 * one);
    let result = compile_bulk_optical_depth_evaluation(&input).expect("finite depth compiles");
    assert!(result.transfer_lower_q0_48 <= result.transfer_upper_q0_48);
    assert!(result.transfer_upper_q0_48 < TRANSMISSION_ONE_Q0_48);
    assert_eq!(result.arithmetic_receipt.observed_maximum_raw_bits, 66);
    validate_bulk_optical_depth_evaluation(&input, &result).expect("result validates");

    let mut forged = result.clone();
    forged.transfer_upper_q0_48 += 1;
    assert!(validate_bulk_optical_depth_evaluation(&input, &forged).is_err());
}

#[test]
fn ordering_width_and_canonical_codec_fail_closed() {
    let one = 1_u128 << 64;
    assert!(compile_bulk_optical_depth_evaluation(&input(2 * one, one)).is_err());
    assert!(compile_bulk_optical_depth_evaluation(&input(0, 1_u128 << 118)).is_err());

    let valid = input(one, one);
    let mut trailing = valid.to_bytes().expect("input encodes");
    trailing.push(b' ');
    assert!(BulkOpticalDepthEvaluationInputV1::from_bytes(&trailing).is_err());

    let unknown = br#"{"schema_version":1,"optical_depth_lower_q64_64":{"high_u64":1,"low_u64":0},"optical_depth_upper_q64_64":{"high_u64":1,"low_u64":0},"unknown":0}"#;
    assert!(BulkOpticalDepthEvaluationInputV1::from_bytes(unknown).is_err());
    assert!(BulkOpticalDepthEvaluationInputV1::from_bytes(&vec![b'0'; 4097]).is_err());
}

#[test]
fn projected_zero_lower_is_finite_underflow_not_opacity() {
    let depth = 64_u128 << 64;
    let result = compile_bulk_optical_depth_evaluation(&input(depth, depth))
        .expect("large finite depth compiles");
    assert_eq!(result.transfer_lower_q0_48, 0);
    assert!(result.transfer_upper_q0_48 <= 1);
    assert_eq!(result.optical_depth_lower_q64_64.to_u128(), depth);
    assert_eq!(result.optical_depth_upper_q64_64.to_u128(), depth);

    let edge = (1_u128 << 118) - 1;
    assert!(compile_bulk_optical_depth_evaluation(&input(0, edge)).is_ok());
}
