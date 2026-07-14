use control_calibration::reference_calibration;

fn main() {
    let receipt = reference_calibration().expect("reference H4 calibration must validate");
    println!("calibration_fingerprint={}", receipt.fingerprint().unwrap());
    println!(
        "h3_candidate_fingerprint={}",
        receipt.h3_candidate_fingerprint
    );
    println!(
        "reference_snapshot_fingerprint={}",
        receipt.reference_snapshot_fingerprint
    );
    for sample in receipt.samples {
        println!(
            "control={:?};fingerprint={};edge_deficit={};rest_span_loss={};hand_vertical_displacement={}",
            sample.control,
            sample.snapshot_fingerprint,
            sample.edge_deficit,
            sample.rest_front_span_loss,
            sample.frame_one_hand_vertical_displacement
        );
    }
}
