use humanoid_generation::reference_receipt;

fn main() {
    let receipt = reference_receipt().expect("reference H3 generation must validate");
    println!("input_fingerprint={}", receipt.input_fingerprint);
    println!("candidate_fingerprint={}", receipt.candidate_fingerprint);
    println!(
        "replay_candidate_fingerprint={}",
        receipt.replay_candidate_fingerprint
    );
    println!("joints={}", receipt.joint_count);
    println!("links={}", receipt.link_count);
    println!("deterministic_replay={}", receipt.deterministic_replay);
    println!("inputs_unchanged={}", receipt.inputs_unchanged);
    println!("capability_free={}", receipt.capability_free);
}
