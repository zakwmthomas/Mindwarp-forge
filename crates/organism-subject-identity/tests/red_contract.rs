use organism_subject_identity::{IndividualIdentityV1, build_individual_identity};

#[test]
fn test_first_red_slice_now_round_trips_stable_identity() {
    let individual = build_individual_identity("world-a", [7; 32], 1).unwrap();
    let bytes = individual.encode_canonical().unwrap();
    assert_eq!(
        IndividualIdentityV1::decode_strict(&bytes).unwrap(),
        individual
    );
}
