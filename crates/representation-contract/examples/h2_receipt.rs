use representation_contract::reference_neutral_humanoid_profile;

fn main() {
    let profile = reference_neutral_humanoid_profile().expect("reference H2 profile must validate");
    let fingerprint: String = profile
        .fingerprint()
        .expect("reference H2 profile must fingerprint")
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    println!("profile_id={}", profile.profile_id);
    println!("profile_fingerprint={fingerprint}");
    println!("scene_fingerprint={}", profile.source_scene_fingerprint);
    println!(
        "reference_suite_fingerprint={}",
        profile.reference_suite_fingerprint
    );
    println!("joints={}", profile.joints.len());
    println!("links={}", profile.link_count);
    println!("frames={}", profile.pose_frame_count);
}
