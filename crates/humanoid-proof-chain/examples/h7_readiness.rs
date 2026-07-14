use humanoid_proof_chain::{reference_promotion_package, simulated_candidate};

fn main() {
    let package = reference_promotion_package().expect("H7 package must rebuild");
    let candidate = simulated_candidate(&package).expect("H7 simulated candidate must bind");
    println!("package_id={}", package.package_id);
    println!("simulated_candidate_id={}", candidate.candidate_id);
    println!("candidate_name={}", package.content.candidate_name);
    println!("claim_count={}", package.content.claims.len());
    println!("non_claim_count={}", package.content.non_claims.len());
    println!("authority=evidence_only_no_promotion");
    println!("kernel_candidate_created=false");
}
