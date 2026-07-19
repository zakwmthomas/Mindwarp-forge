use entity_lifecycle::AgeCohort;
use entity_lifecycle_history_binding::AmbientCohortBindingV1;

#[test]
fn ambient_cohort_binding_is_stable_strict_and_never_rerolls() {
    let binding = AmbientCohortBindingV1::new([1; 32], [2; 32], AgeCohort::Juvenile).unwrap();
    let bytes = binding.encode_canonical();
    assert_eq!(
        AmbientCohortBindingV1::decode_strict(&bytes).unwrap(),
        binding
    );
    assert_eq!(
        binding.fingerprint(),
        AmbientCohortBindingV1::decode_strict(&bytes)
            .unwrap()
            .fingerprint()
    );
    assert_ne!(
        binding,
        AmbientCohortBindingV1::new([3; 32], [2; 32], AgeCohort::Juvenile).unwrap(),
        "cohort.entity-drift"
    );
    binding
        .verify_expected([1; 32], [2; 32], AgeCohort::Juvenile)
        .unwrap();
    assert!(
        binding
            .verify_expected([3; 32], [2; 32], AgeCohort::Juvenile)
            .is_err(),
        "cohort.entity-drift"
    );
    assert!(
        binding
            .verify_expected([1; 32], [3; 32], AgeCohort::Juvenile)
            .is_err(),
        "cohort.contract-drift"
    );
    assert!(
        binding
            .verify_expected([1; 32], [2; 32], AgeCohort::Adult)
            .is_err(),
        "cohort.value-drift"
    );
    assert!(
        AmbientCohortBindingV1::new([0; 32], [2; 32], AgeCohort::Adult).is_err(),
        "cohort.zero-entity"
    );
    assert!(
        AmbientCohortBindingV1::new([1; 32], [0; 32], AgeCohort::Adult).is_err(),
        "cohort.zero-contract"
    );
    let stored = AmbientCohortBindingV1::decode_strict(&bytes).unwrap();
    assert!(
        stored
            .verify_expected([1; 32], [2; 32], AgeCohort::Adult)
            .is_err(),
        "cohort.reroll"
    );
    let mut invalid = bytes.clone();
    invalid[66] = 9;
    assert!(
        AmbientCohortBindingV1::decode_strict(&invalid).is_err(),
        "cohort.value-drift"
    );
    let mut trailing = bytes.clone();
    trailing.push(0);
    assert!(
        AmbientCohortBindingV1::decode_strict(&trailing).is_err(),
        "cohort.trailing-bytes"
    );
}
