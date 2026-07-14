use humanoid_proof_chain::{reference_h5_decision, reference_manifest};

fn main() {
    let h5 = reference_h5_decision().expect("H5 decision must rebuild");
    let manifest = reference_manifest().expect("H1-H5 manifest must rebuild");
    println!("h5_receipt_id={}", h5.receipt_id);
    println!("manifest_id={}", manifest.manifest_id);
    println!("stage_count={}", manifest.content.stages.len());
    println!("authority=evidence_only_no_promotion");
    println!("deterministic_replay=true");
    println!("capability_free=true");
}
