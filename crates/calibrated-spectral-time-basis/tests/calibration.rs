use calibrated_spectral_time_basis::{
    CalibratedBandV1, CalibratedSpectralIntervalV1, CalibratedSpectralTimeBasisError,
    CalibratedSpectralTimeBasisInputV1, CalibratedSpectralTimeBasisV1, CalibratedTimeCellV1,
    ExactUnsignedRationalV1, MAX_INPUT_BYTES, MAX_RESULT_BYTES,
    compile_calibrated_spectral_time_basis,
};
use optical_phase_space_dimensionless_transfer::compile_optical_band_time_binding;
use sha2::{Digest, Sha256};
use visible_radiance_bulk_transfer::VisibleRadianceBandV1;

fn rational(numerator: &str, denominator: &str) -> ExactUnsignedRationalV1 {
    ExactUnsignedRationalV1 {
        denominator: denominator.to_owned(),
        numerator: numerator.to_owned(),
    }
}

fn fixture() -> CalibratedSpectralTimeBasisInputV1 {
    CalibratedSpectralTimeBasisInputV1 {
        basis_version: 1,
        calibration_provenance_id: [17; 32],
        quantity_kind: "radiant_energy".to_owned(),
        schema_version: 1,
        spectral_coordinate: "vacuum_wavelength_metre".to_owned(),
        spectral_intervals: [
            CalibratedSpectralIntervalV1 {
                band: CalibratedBandV1::Blue,
                lower: rational("1", "2500000"),
                upper: rational("1", "2000000"),
            },
            CalibratedSpectralIntervalV1 {
                band: CalibratedBandV1::Green,
                lower: rational("1", "2000000"),
                upper: rational("3", "5000000"),
            },
            CalibratedSpectralIntervalV1 {
                band: CalibratedBandV1::Red,
                lower: rational("3", "5000000"),
                upper: rational("7", "10000000"),
            },
        ],
        spectral_weighting: "unit_energy_integral".to_owned(),
        time_cell: CalibratedTimeCellV1 {
            clock_origin_id: [34; 32],
            end_tick: 116,
            seconds_per_tick: rational("1", "1000"),
            start_tick: 100,
        },
        unit: "joule".to_owned(),
    }
}

fn hex(bytes: [u8; 32]) -> String {
    bytes.iter().map(|value| format!("{value:02x}")).collect()
}

fn sha(bytes: &[u8]) -> String {
    let digest: [u8; 32] = Sha256::digest(bytes).into();
    hex(digest)
}

#[test]
fn exact_identity_codec_and_legacy_owner_lock() {
    let lock: serde_json::Value = serde_json::from_str(include_str!(
        "../fixtures/calibration_v1_identity_lock.json"
    ))
    .unwrap();
    let input = fixture();
    let input_bytes = input.to_bytes().unwrap();
    let result = compile_calibrated_spectral_time_basis(&input).unwrap();
    let result_bytes = result.to_bytes().unwrap();
    assert_eq!(input_bytes.len(), 896);
    assert_eq!(result_bytes.len(), 1_786);
    assert_eq!(sha(&input_bytes), lock["input"]["sha256"].as_str().unwrap());
    assert_eq!(
        sha(&result_bytes),
        lock["result"]["sha256"].as_str().unwrap()
    );
    assert_eq!(
        hex(result.calibrated_basis_id),
        lock["calibrated_basis_id"].as_str().unwrap()
    );
    assert_eq!(
        hex(result.derived_legacy_time_basis_id),
        lock["derived_legacy_time_basis_id"].as_str().unwrap()
    );
    let legacy = [
        (
            VisibleRadianceBandV1::Blue,
            result.derived_legacy_band_time_ids.blue,
        ),
        (
            VisibleRadianceBandV1::Green,
            result.derived_legacy_band_time_ids.green,
        ),
        (
            VisibleRadianceBandV1::Red,
            result.derived_legacy_band_time_ids.red,
        ),
    ];
    for (band, expected) in legacy {
        let binding =
            compile_optical_band_time_binding(band, result.derived_legacy_time_basis_id).unwrap();
        assert_eq!(binding.band_time_id, expected);
    }
    assert_eq!(
        hex(result.derived_legacy_band_time_ids.blue),
        lock["derived_legacy_band_time_ids"]["blue"]
            .as_str()
            .unwrap()
    );
    assert_eq!(
        hex(result.derived_legacy_band_time_ids.green),
        lock["derived_legacy_band_time_ids"]["green"]
            .as_str()
            .unwrap()
    );
    assert_eq!(
        hex(result.derived_legacy_band_time_ids.red),
        lock["derived_legacy_band_time_ids"]["red"]
            .as_str()
            .unwrap()
    );
    assert_eq!(
        CalibratedSpectralTimeBasisInputV1::from_bytes(&input_bytes).unwrap(),
        input
    );
    assert_eq!(
        CalibratedSpectralTimeBasisV1::from_bytes(&result_bytes).unwrap(),
        result
    );
}

#[test]
fn substitutions_change_basis_time_and_band_identities() {
    let base = fixture();
    let baseline = compile_calibrated_spectral_time_basis(&base).unwrap();
    let mut variants = Vec::new();
    let mut value = base.clone();
    value.basis_version = 2;
    variants.push(value);
    let mut value = base.clone();
    value.calibration_provenance_id = [18; 32];
    variants.push(value);
    let mut value = base.clone();
    value.spectral_intervals[0].lower = rational("1", "3000000");
    variants.push(value);
    let mut value = base.clone();
    value.time_cell.end_tick = 117;
    variants.push(value);
    let mut value = base.clone();
    value.time_cell.clock_origin_id = [35; 32];
    variants.push(value);
    let mut value = base;
    value.time_cell.seconds_per_tick = rational("1", "2000");
    variants.push(value);
    for variant in variants {
        let result = compile_calibrated_spectral_time_basis(&variant).unwrap();
        assert_ne!(result.calibrated_basis_id, baseline.calibrated_basis_id);
        assert_ne!(
            result.derived_legacy_time_basis_id,
            baseline.derived_legacy_time_basis_id
        );
        assert_ne!(
            result.derived_legacy_band_time_ids,
            baseline.derived_legacy_band_time_ids
        );
    }
}

#[test]
fn rational_interval_time_and_identity_hostiles_fail_typed() {
    let invalid = [
        rational("-1", "2"),
        rational("+1", "2"),
        rational("01", "2"),
        rational("1", "0"),
        rational("2", "4"),
        rational("0", "2"),
        rational("340282366920938463463374607431768211456", "1"),
    ];
    for bad in invalid {
        let mut input = fixture();
        input.spectral_intervals[0].lower = bad;
        assert!(compile_calibrated_spectral_time_basis(&input).is_err());
    }
    let mut cases = Vec::new();
    let mut value = fixture();
    value.basis_version = 0;
    cases.push(value);
    let mut value = fixture();
    value.calibration_provenance_id = [0; 32];
    cases.push(value);
    let mut value = fixture();
    value.spectral_intervals[0].band = CalibratedBandV1::Red;
    cases.push(value);
    let mut value = fixture();
    value.spectral_intervals[0].upper = rational("1", "2500000");
    cases.push(value);
    let mut value = fixture();
    value.spectral_intervals[1].lower = rational("11", "20000000");
    cases.push(value);
    let mut value = fixture();
    value.spectral_intervals[1].lower = rational("9", "20000000");
    cases.push(value);
    let mut value = fixture();
    value.time_cell.clock_origin_id = [0; 32];
    cases.push(value);
    let mut value = fixture();
    value.time_cell.end_tick = 100;
    cases.push(value);
    let mut value = fixture();
    value.time_cell.seconds_per_tick = rational("0", "1");
    cases.push(value);
    for input in cases {
        assert!(compile_calibrated_spectral_time_basis(&input).is_err());
    }
}

#[test]
fn strict_raw_codec_ceiling_and_forgery_shields() {
    let input = fixture();
    let input_bytes = input.to_bytes().unwrap();
    let result = compile_calibrated_spectral_time_basis(&input).unwrap();
    let result_bytes = result.to_bytes().unwrap();
    let mut trailing = input_bytes.clone();
    trailing.push(b' ');
    assert_eq!(
        CalibratedSpectralTimeBasisInputV1::from_bytes(&trailing),
        Err(CalibratedSpectralTimeBasisError::CodecDefect)
    );
    let duplicate =
        String::from_utf8(input_bytes.clone())
            .unwrap()
            .replacen("{", "{\"basis_version\":1,", 1);
    assert!(CalibratedSpectralTimeBasisInputV1::from_bytes(duplicate.as_bytes()).is_err());
    assert!(CalibratedSpectralTimeBasisInputV1::from_bytes(&[0xff]).is_err());
    let unknown = String::from_utf8(input_bytes)
        .unwrap()
        .replacen("{", "{\"unknown\":1,", 1);
    assert!(CalibratedSpectralTimeBasisInputV1::from_bytes(unknown.as_bytes()).is_err());
    assert_eq!(
        CalibratedSpectralTimeBasisInputV1::from_bytes(&vec![b' '; MAX_INPUT_BYTES + 1]),
        Err(CalibratedSpectralTimeBasisError::ByteCeiling)
    );
    assert_eq!(
        CalibratedSpectralTimeBasisV1::from_bytes(&vec![b' '; MAX_RESULT_BYTES + 1]),
        Err(CalibratedSpectralTimeBasisError::ByteCeiling)
    );
    let mut forged = result;
    forged.calibrated_basis_id = [99; 32];
    assert_eq!(
        forged.to_bytes(),
        Err(CalibratedSpectralTimeBasisError::IdentityMismatch)
    );
    let mut trailing_result = result_bytes;
    trailing_result.push(b'\n');
    assert_eq!(
        CalibratedSpectralTimeBasisV1::from_bytes(&trailing_result),
        Err(CalibratedSpectralTimeBasisError::CodecDefect)
    );
}
