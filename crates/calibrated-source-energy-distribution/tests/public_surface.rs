use calibrated_source_energy_distribution::{
    CalibratedSourceEnergyDistributionQueryV1, CalibratedSourceEnergyDistributionV1,
    SourceEnergyDistributionError, compile_calibrated_source_energy_distribution,
};

#[test]
fn frozen_public_surface_exists() {
    let _: fn(
        CalibratedSourceEnergyDistributionQueryV1,
    ) -> Result<CalibratedSourceEnergyDistributionV1, SourceEnergyDistributionError> =
        compile_calibrated_source_energy_distribution;
}
