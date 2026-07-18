//! Capability-free G1-C4 integration proof: the first "addressable world
//! package" seam named by the master program.
//!
//! `hierarchy-history` deliberately accepts only an opaque
//! `world_conditions_contract`/`world_conditions_fingerprint` pair on
//! `HierarchyDescriptor` and does not implement or import derived-world
//! rules (see `HIERARCHY_HISTORY_DESIGN_GATE.md`). This crate proves that a
//! real `derived_world_rules::CausalWorldPacket` can fill that seam, without
//! either module depending on the other's internals. It is evidence only:
//! no save, storage engine, runtime residency, or Kernel authority is
//! implemented here.

use derived_world_rules::{CausalWorldPacket, WorldGenerationInput, validate_world_packet};
use hierarchy_history::{DescriptorOrigin, HierarchyDescriptor, HierarchyHistoryError};
use sha2::{Digest, Sha256};

pub const BOUND_DERIVED_WORLD_SCHEMA: u16 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindingError {
    UnsupportedSchema(u16),
    Descriptor(HierarchyHistoryError),
    MalformedPacketId,
    InvalidCausalPacket,
}

/// Fingerprint of *which* opaque world-conditions contract/version is bound.
/// This never depends on packet content; it only names the accepted schema.
pub fn world_conditions_contract_fingerprint(schema_version: u16) -> [u8; 32] {
    Sha256::digest(
        [
            b"mindwarp.hierarchy.world-conditions-contract.derived-world-rules.v1\0".as_slice(),
            &schema_version.to_be_bytes(),
        ]
        .concat(),
    )
    .into()
}

/// The exact whole-packet fingerprint used as `world_conditions_fingerprint`.
/// This is provenance-sensitive: it changes whenever the packet's `input_id`
/// changes, even if the physical palette/signal content is identical. A
/// physical-only (provenance-independent) fingerprint is not implemented by
/// this crate; see the retained limitation below.
pub fn world_conditions_packet_fingerprint(
    packet: &CausalWorldPacket,
) -> Result<[u8; 32], BindingError> {
    hex_decode(&packet.packet_id)
        .and_then(|bytes| bytes.try_into().ok())
        .ok_or(BindingError::MalformedPacketId)
}

fn hex_decode(text: &str) -> Option<Vec<u8>> {
    if text.len() % 2 != 0 {
        return None;
    }
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(text.get(i..i + 2)?, 16).ok())
        .collect()
}

pub fn bind_addressable_world_package(
    logical_id: [u8; 32],
    parent_logical_id: Option<[u8; 32]>,
    reconstruction_fingerprint: [u8; 32],
    world_input: &WorldGenerationInput,
    packet: &CausalWorldPacket,
    recipe: Vec<u8>,
) -> Result<HierarchyDescriptor, BindingError> {
    validate_world_packet(world_input, packet).map_err(|_| BindingError::InvalidCausalPacket)?;
    if packet.content.schema_version != BOUND_DERIVED_WORLD_SCHEMA {
        return Err(BindingError::UnsupportedSchema(
            packet.content.schema_version,
        ));
    }
    let world_conditions_contract =
        world_conditions_contract_fingerprint(packet.content.schema_version);
    let world_conditions_fingerprint = world_conditions_packet_fingerprint(packet)?;
    HierarchyDescriptor::new(
        logical_id,
        parent_logical_id,
        reconstruction_fingerprint,
        world_conditions_contract,
        world_conditions_fingerprint,
        DescriptorOrigin::Procedural,
        recipe,
    )
    .map_err(BindingError::Descriptor)
}

#[cfg(test)]
mod tests {
    use super::*;
    use derived_world_rules::{
        ClimateContract, ClimateInput, GeologicalAtmosphericContract, GeologicalAtmosphericInput,
        HydrologicalContract, HydrologicalInput, RegionalEnvironmentContract,
        RegionalEnvironmentInput, SignalChannel, SignalPotential, StellarOrbitalContract,
        StellarOrbitalInput, WorldGenerationInput, compile_climate, compile_geological_atmospheric,
        compile_hydrological, compile_regional_environment, compile_stellar_orbital, compile_world,
    };
    use field_basis::{FieldRecipe, ONE, Term};

    fn regional_contract(reconstruction_id: [u8; 32]) -> RegionalEnvironmentContract {
        compile_regional_environment(&RegionalEnvironmentInput {
            schema_version: 1,
            reconstruction_id,
            regional_source_id: [8; 32],
            field_recipe_bytes: FieldRecipe::new(vec![Term::Constant(ONE)], 0)
                .unwrap()
                .encode_canonical()
                .unwrap(),
            moisture_source_id: [9; 32],
            moisture_field_recipe_bytes: FieldRecipe::new(vec![Term::Constant(0)], 0)
                .unwrap()
                .encode_canonical()
                .unwrap(),
            coordinate_q32_32: [0, 0],
        })
        .unwrap()
    }

    fn stellar_contract(reconstruction_id: [u8; 32]) -> StellarOrbitalContract {
        compile_stellar_orbital(&StellarOrbitalInput {
            schema_version: 1,
            reconstruction_id,
            stellar_source_id: [3; 32],
            primary_mass_milli_solar: 1_000,
            stellar_luminosity_millionths_solar: 1_000_000,
            stellar_spectrum_rgb_permille: [400, 350, 250],
            semi_major_axis_milli_au: 1_000,
            eccentricity_millionths: 0,
        })
        .unwrap()
    }

    fn geological_contract(reconstruction_id: [u8; 32]) -> GeologicalAtmosphericContract {
        compile_geological_atmospheric(&GeologicalAtmosphericInput {
            schema_version: 1,
            reconstruction_id,
            planetary_body_id: [4; 32],
            stellar_orbital: stellar_contract(reconstruction_id),
            planet_mass_milli_earth: 1_000,
            planet_radius_milli_earth: 1_000,
            internal_heat_flux_milli_w_m2: 87,
            solid_surface_fraction_permille: 600,
            atmospheric_column_mass_g_m2: 10_332_000,
            gas_transmission_rgb_permille: [800, 900, 950],
            aerosol_transmission_rgb_permille: [1_000; 3],
        })
        .unwrap()
    }

    fn hydrological_contract(reconstruction_id: [u8; 32]) -> HydrologicalContract {
        compile_hydrological(&HydrologicalInput {
            schema_version: 1,
            reconstruction_id,
            hydrological_source_id: [5; 32],
            geological_atmospheric: geological_contract(reconstruction_id),
            total_water_column_g_m2: 2_000_000,
            phase_partition_permille: [100, 850, 50],
            surface_accessible_liquid_fraction_permille: 700,
        })
        .unwrap()
    }

    fn climate_contract(reconstruction_id: [u8; 32]) -> ClimateContract {
        compile_climate(&ClimateInput {
            schema_version: 1,
            reconstruction_id,
            climate_source_id: [6; 32],
            hydrological: hydrological_contract(reconstruction_id),
            bond_albedo_permille: 300,
            outgoing_longwave_fraction_of_incident_permille: 700,
        })
        .unwrap()
    }

    fn surface_material_contract(
        reconstruction_id: [u8; 32],
    ) -> derived_world_rules::SurfaceMaterialContract {
        derived_world_rules::compile_surface_material(&derived_world_rules::SurfaceMaterialInput {
            schema_version: 1,
            reconstruction_id,
            material_source_id: [7; 32],
            climate: climate_contract(reconstruction_id),
            dominant_surface_reflectance_rgb_permille: [500, 400, 300],
        })
        .unwrap()
    }

    fn base_input() -> WorldGenerationInput {
        WorldGenerationInput {
            schema_version: 1,
            field_contract_version: field_basis::CONTRACT_VERSION,
            reconstruction_id: [1; 32],
            surface_material: surface_material_contract([1; 32]),
            regional_environment: regional_contract([1; 32]),
            signal_potentials: vec![SignalPotential {
                channel: SignalChannel::VisibleRadiance,
                baseline_strength_permille: 900,
            }],
        }
    }

    #[test]
    fn real_causal_packet_binds_deterministically_and_replays() {
        let packet = compile_world(&base_input()).unwrap();
        let input = base_input();
        let a = bind_addressable_world_package([1; 32], None, [2; 32], &input, &packet, vec![9])
            .unwrap();
        let b = bind_addressable_world_package([1; 32], None, [2; 32], &input, &packet, vec![9])
            .unwrap();
        assert_eq!(a.fingerprint().unwrap(), b.fingerprint().unwrap());
        assert_eq!(
            HierarchyDescriptor::decode_strict(&a.encode_canonical().unwrap()).unwrap(),
            a
        );
    }

    #[test]
    fn changed_physical_drivers_move_world_conditions_without_moving_place_identity() {
        let mut input = base_input();
        let packet1 = compile_world(&input).unwrap();
        input
            .surface_material
            .input
            .climate
            .input
            .hydrological
            .input
            .geological_atmospheric
            .input
            .gas_transmission_rgb_permille = [200, 900, 950];
        input
            .surface_material
            .input
            .climate
            .input
            .hydrological
            .input
            .geological_atmospheric = compile_geological_atmospheric(
            &input
                .surface_material
                .input
                .climate
                .input
                .hydrological
                .input
                .geological_atmospheric
                .input,
        )
        .unwrap();
        input.surface_material.input.climate.input.hydrological = compile_hydrological(
            &input
                .surface_material
                .input
                .climate
                .input
                .hydrological
                .input,
        )
        .unwrap();
        input.surface_material.input.climate =
            compile_climate(&input.surface_material.input.climate.input).unwrap();
        input.surface_material =
            derived_world_rules::compile_surface_material(&input.surface_material.input).unwrap();
        let packet2 = compile_world(&input).unwrap();

        let first_input = base_input();
        let a =
            bind_addressable_world_package([1; 32], None, [2; 32], &first_input, &packet1, vec![9])
                .unwrap();
        let b = bind_addressable_world_package([1; 32], None, [2; 32], &input, &packet2, vec![9])
            .unwrap();

        assert_eq!(a.logical_id, b.logical_id);
        assert_eq!(a.reconstruction_fingerprint, b.reconstruction_fingerprint);
        assert_ne!(
            a.world_conditions_fingerprint,
            b.world_conditions_fingerprint
        );
        assert_ne!(a.fingerprint().unwrap(), b.fingerprint().unwrap());
    }

    /// Retained limitation: the whole-packet fingerprint is provenance-
    /// sensitive (it embeds `input_id`), so two places with different
    /// provenance but byte-identical physical palette/signal content still
    /// bind to different `world_conditions_fingerprint` values. A future
    /// physical-only fingerprint (excluding provenance) is not implemented
    /// here and remains open for any downstream system that wants to detect
    /// "physically identical conditions" across different places.
    #[test]
    fn different_provenance_same_physical_drivers_yields_same_physical_content_but_different_fingerprint()
     {
        let mut input_one = base_input();
        input_one.reconstruction_id = [7; 32];
        input_one.surface_material = surface_material_contract([7; 32]);
        input_one.regional_environment = regional_contract([7; 32]);
        let mut input_two = base_input();
        input_two.reconstruction_id = [8; 32];
        input_two.surface_material = surface_material_contract([8; 32]);
        input_two.regional_environment = regional_contract([8; 32]);

        let packet_one = compile_world(&input_one).unwrap();
        let packet_two = compile_world(&input_two).unwrap();

        assert_eq!(
            packet_one.content.physical_palette_rgb_permille,
            packet_two.content.physical_palette_rgb_permille
        );
        assert_eq!(packet_one.content.signals, packet_two.content.signals);

        let a = bind_addressable_world_package(
            [1; 32],
            None,
            [2; 32],
            &input_one,
            &packet_one,
            vec![9],
        )
        .unwrap();
        let b = bind_addressable_world_package(
            [1; 32],
            None,
            [2; 32],
            &input_two,
            &packet_two,
            vec![9],
        )
        .unwrap();
        assert_ne!(
            a.world_conditions_fingerprint,
            b.world_conditions_fingerprint
        );
    }

    #[test]
    fn unsupported_schema_and_malformed_descriptor_inputs_fail_closed() {
        let mut poisoned = compile_world(&base_input()).unwrap();
        poisoned.content.schema_version = 99;
        assert_eq!(
            bind_addressable_world_package(
                [1; 32],
                None,
                [2; 32],
                &base_input(),
                &poisoned,
                vec![9]
            ),
            Err(BindingError::InvalidCausalPacket)
        );

        let packet = compile_world(&base_input()).unwrap();
        assert!(matches!(
            bind_addressable_world_package([1; 32], None, [2; 32], &base_input(), &packet, vec![]),
            Err(BindingError::Descriptor(_))
        ));
    }

    #[test]
    fn packet_from_another_input_is_rejected() {
        let input = base_input();
        let mut other = input.clone();
        other.reconstruction_id = [9; 32];
        other.surface_material = surface_material_contract([9; 32]);
        other.regional_environment = regional_contract([9; 32]);
        let packet = compile_world(&other).unwrap();
        assert_eq!(
            bind_addressable_world_package([1; 32], None, [2; 32], &input, &packet, vec![9]),
            Err(BindingError::InvalidCausalPacket)
        );
    }
}
