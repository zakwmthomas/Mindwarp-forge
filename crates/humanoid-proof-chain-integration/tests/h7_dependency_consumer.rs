use forge_kernel::{ActorKind, AuthorityBasis, CandidateState, ForgeKernel};
use humanoid_proof_chain::reference_promotion_package;
use humanoid_proof_chain_integration::{H7_CANDIDATE_ID, H7DependencyError, verify_h7_dependency};

fn exact_candidate(state: CandidateState) -> ForgeKernel {
    let package = reference_promotion_package().unwrap();
    let mut kernel = ForgeKernel::default();
    let evidence = kernel
        .register_evidence(ActorKind::Assistant, package.to_bytes().unwrap(), "test:h7")
        .unwrap();
    let candidate = kernel.propose_candidate(&evidence, "test:h7").unwrap();
    assert_eq!(candidate, H7_CANDIDATE_ID);
    if matches!(
        state,
        CandidateState::Approved | CandidateState::Promoted | CandidateState::Superseded
    ) {
        kernel
            .approve_candidate(
                ActorKind::DirectProjectUser,
                AuthorityBasis::ExplicitUserAuthorization,
                H7_CANDIDATE_ID,
                "test:h7-approval",
            )
            .unwrap();
    }
    if matches!(state, CandidateState::Promoted | CandidateState::Superseded) {
        kernel
            .promote_candidate(
                ActorKind::DirectProjectUser,
                AuthorityBasis::ExplicitUserAuthorization,
                H7_CANDIDATE_ID,
                "test:h7-promotion",
            )
            .unwrap();
    }
    if state == CandidateState::Superseded {
        let correction = kernel
            .register_evidence(
                ActorKind::Assistant,
                b"replacement evidence",
                "test:correction",
            )
            .unwrap();
        kernel
            .supersede_candidate(
                ActorKind::DirectProjectUser,
                AuthorityBasis::ExplicitUserAuthorization,
                H7_CANDIDATE_ID,
                &correction,
                None,
                "test:h7-supersession",
            )
            .unwrap();
    }
    kernel
}

#[test]
fn exact_promoted_non_superseded_package_passes_with_scope_retained() {
    let kernel = exact_candidate(CandidateState::Promoted);
    let receipt = verify_h7_dependency(&kernel).unwrap();
    assert_eq!(receipt.lifecycle_state, CandidateState::Promoted);
    assert!(!receipt.superseded);
    assert_eq!(receipt.claims.len(), 6);
    assert_eq!(receipt.retained_blockers.len(), 8);
    assert!(
        receipt
            .retained_blockers
            .contains(&"no_device_runtime_engine_or_production_fitness".into())
    );
    assert_eq!(
        receipt.authority_effect,
        "read_only_dependency_verification"
    );
}

#[test]
fn generic_promoted_candidate_does_not_substitute_for_exact_h7() {
    let mut kernel = ForgeKernel::default();
    let evidence = kernel
        .register_evidence(ActorKind::Assistant, b"generic", "test:generic")
        .unwrap();
    let candidate = kernel.propose_candidate(&evidence, "test:generic").unwrap();
    kernel
        .approve_candidate(
            ActorKind::DirectProjectUser,
            AuthorityBasis::ExplicitUserAuthorization,
            &candidate,
            "test:generic-approval",
        )
        .unwrap();
    kernel
        .promote_candidate(
            ActorKind::DirectProjectUser,
            AuthorityBasis::ExplicitUserAuthorization,
            &candidate,
            "test:generic-promotion",
        )
        .unwrap();
    assert_eq!(
        verify_h7_dependency(&kernel),
        Err(H7DependencyError::MissingExactCandidate)
    );
}

#[test]
fn proposed_approved_and_superseded_h7_fail_closed() {
    for state in [
        CandidateState::Proposed,
        CandidateState::Approved,
        CandidateState::Superseded,
    ] {
        let kernel = exact_candidate(state.clone());
        assert_eq!(
            verify_h7_dependency(&kernel),
            Err(H7DependencyError::WrongLifecycleState(state))
        );
    }
}
