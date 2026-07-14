use forge_kernel::{ActorKind, ForgeKernel};
use humanoid_proof_chain::reference_promotion_package;

fn main() {
    let package = reference_promotion_package().expect("H7 package must rebuild");
    let package_bytes = package.to_bytes().expect("H7 package must encode");
    let mut kernel = ForgeKernel::default();
    let evidence_id = kernel
        .register_evidence(
            ActorKind::Assistant,
            &package_bytes,
            "h7:disposable-candidate-proof",
        )
        .expect("H7 evidence must register");
    let candidate_id = kernel
        .propose_candidate(&evidence_id, "h7:disposable-candidate-proof")
        .expect("H7 candidate must propose");

    println!("package_id={}", package.package_id);
    println!("package_bytes={}", package_bytes.len());
    println!("kernel_evidence_id={evidence_id}");
    println!("kernel_candidate_id={candidate_id}");
    println!("kernel_candidate_state=proposed");
    println!("kernel_event_count={}", kernel.events().len());
    println!("authority=evidence_only_no_approval_or_promotion");
    println!("persistent_forge_mutated=false");
}
