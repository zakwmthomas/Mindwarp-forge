//! Capability-free exact swept fixed-orientation AABB passage evidence.
//! Not runtime collision response, navigation, organism, planet, or biome logic.

use physical_path_substrate::{
    CellEvidenceV1, CellIndex3V1, Id, MAX_PHYSICAL_VOLUME_PROOF_CELLS, PhysicalVolumeRecipeV1,
    PhysicalVolumeV1, UnitRationalV1, validate_physical_volume,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const CONTRACT_VERSION: u16 = 1;
pub const MAX_WITNESSES: usize = 65_536;
pub const MAX_CANONICAL_RESULT_BYTES: usize = 32 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MotionKindV1 {
    FixedOrientationTranslation,
    UnsupportedRotation,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MechanicalDispositionV1 {
    BlocksTranslation,
    DoesNotBlockTranslation,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MechanicalRuleV1 {
    pub subject: CellEvidenceV1,
    pub disposition: MechanicalDispositionV1,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SweptAabbQueryV1 {
    pub schema_version: u16,
    pub physical_volume_id: Id,
    pub query_source_id: Id,
    pub half_extents_q32_32: [i64; 3],
    pub start_q32_32: [i64; 3],
    pub end_q32_32: [i64; 3],
    pub motion_kind: MotionKindV1,
    pub mechanical_profile_source_id: Id,
    pub mechanical_rules: Vec<MechanicalRuleV1>,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContactClassV1 {
    ContactOnly,
    InteriorInterval,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExpandedCellWitnessV1 {
    pub index: CellIndex3V1,
    pub evidence: CellEvidenceV1,
    pub disposition: Option<MechanicalDispositionV1>,
    pub t_enter: UnitRationalV1,
    pub t_exit: UnitRationalV1,
    pub contact_class: ContactClassV1,
    pub entry_axis_mask: u8,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PassageOutcomeV1 {
    StationarySeparated,
    StationaryContactOnly,
    StationaryInteriorOverlap,
    SweepSeparated,
    SweepContactOnly,
    SweepFirstInteriorEntry,
    InitialInteriorOverlap,
    OuterDomainContact,
    InitialOuterDomainOverlap,
    UnavailableEvidence,
    InteractionModelRequired,
    UnsupportedMotion,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SweptAabbResultV1 {
    pub schema_version: u16,
    pub physical_volume_id: Id,
    pub query_id: Id,
    pub outcome: PassageOutcomeV1,
    pub first_event_t: Option<UnitRationalV1>,
    pub witnesses: Vec<ExpandedCellWitnessV1>,
    pub result_id: Id,
    pub limitations: Vec<String>,
    pub authority_effect: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SweptAabbError {
    Invalid(&'static str),
    Codec(String),
}

impl SweptAabbQueryV1 {
    pub fn to_bytes(
        &self,
        r: &PhysicalVolumeRecipeV1,
        v: &PhysicalVolumeV1,
    ) -> Result<Vec<u8>, SweptAabbError> {
        validate_query(r, v, self)?;
        encode(self)
    }
    pub fn from_bytes(
        r: &PhysicalVolumeRecipeV1,
        v: &PhysicalVolumeV1,
        b: &[u8],
    ) -> Result<Self, SweptAabbError> {
        let x: Self = decode(b)?;
        validate_query(r, v, &x)?;
        canonical(&x, b, "noncanonical query bytes")?;
        Ok(x)
    }
}
impl SweptAabbResultV1 {
    pub fn to_bytes(
        &self,
        r: &PhysicalVolumeRecipeV1,
        v: &PhysicalVolumeV1,
        q: &SweptAabbQueryV1,
    ) -> Result<Vec<u8>, SweptAabbError> {
        validate_result(r, v, q, self)?;
        encode(self)
    }
    pub fn from_bytes(
        r: &PhysicalVolumeRecipeV1,
        v: &PhysicalVolumeV1,
        q: &SweptAabbQueryV1,
        b: &[u8],
    ) -> Result<Self, SweptAabbError> {
        let x: Self = decode(b)?;
        validate_result(r, v, q, &x)?;
        canonical(&x, b, "noncanonical result bytes")?;
        Ok(x)
    }
}

pub fn compile_swept_aabb(
    r: &PhysicalVolumeRecipeV1,
    v: &PhysicalVolumeV1,
    q: &SweptAabbQueryV1,
) -> Result<SweptAabbResultV1, SweptAabbError> {
    validate_query(r, v, q)?;
    let qid = hash(b"mindwarp.swept-aabb.query.v1", &encode(q)?);
    if q.motion_kind != MotionKindV1::FixedOrientationTranslation {
        return finish(v, qid, PassageOutcomeV1::UnsupportedMotion, None, vec![]);
    }
    let outer = outer_event(r, q)?;
    if let Some((initial, t)) = outer
        && (initial || t == UnitRationalV1::zero())
    {
        return finish(
            v,
            qid,
            if initial {
                PassageOutcomeV1::InitialOuterDomainOverlap
            } else {
                PassageOutcomeV1::OuterDomainContact
            },
            Some(t),
            vec![],
        );
    }
    let cells = reconstruct(r)?;
    let mut ws = Vec::new();
    for x in 0..r.input.extent[0] {
        for y in 0..r.input.extent[1] {
            for z in 0..r.input.extent[2] {
                let index = CellIndex3V1 { x, y, z };
                let evidence = cells[flat(r, index)?].clone();
                let bounds = expanded(r, index, q.half_extents_q32_32)?;
                if let Some(h) = intersect(q.start_q32_32, q.end_q32_32, bounds.0, bounds.1)? {
                    ws.push(ExpandedCellWitnessV1 {
                        index,
                        evidence: evidence.clone(),
                        disposition: lookup(q, &evidence),
                        t_enter: h.enter,
                        t_exit: h.exit,
                        contact_class: if h.interior {
                            ContactClassV1::InteriorInterval
                        } else {
                            ContactClassV1::ContactOnly
                        },
                        entry_axis_mask: h.mask,
                    })
                }
            }
        }
    }
    ws.sort_by(|a, b| {
        a.t_enter
            .cmp(&b.t_enter)
            .then(a.t_exit.cmp(&b.t_exit))
            .then(a.index.cmp(&b.index))
    });
    if ws.len() > MAX_WITNESSES {
        return Err(SweptAabbError::Invalid("witness ceiling exceeded"));
    }
    let stationary = q.start_q32_32 == q.end_q32_32;
    let mut out = if stationary {
        PassageOutcomeV1::StationarySeparated
    } else {
        PassageOutcomeV1::SweepSeparated
    };
    let mut first = None;
    for w in &ws {
        if matches!(w.evidence, CellEvidenceV1::Unavailable) {
            out = PassageOutcomeV1::UnavailableEvidence;
            first = Some(w.t_enter);
            break;
        }
        let Some(d) = w.disposition else {
            out = PassageOutcomeV1::InteractionModelRequired;
            first = Some(w.t_enter);
            break;
        };
        if d != MechanicalDispositionV1::BlocksTranslation {
            continue;
        }
        let initial = w.contact_class == ContactClassV1::InteriorInterval
            && w.t_enter == UnitRationalV1::zero()
            && inside(q.start_q32_32, expanded(r, w.index, q.half_extents_q32_32)?);
        out = if initial {
            PassageOutcomeV1::InitialInteriorOverlap
        } else if stationary && w.contact_class == ContactClassV1::InteriorInterval {
            PassageOutcomeV1::StationaryInteriorOverlap
        } else if stationary {
            PassageOutcomeV1::StationaryContactOnly
        } else if w.contact_class == ContactClassV1::InteriorInterval {
            PassageOutcomeV1::SweepFirstInteriorEntry
        } else {
            PassageOutcomeV1::SweepContactOnly
        };
        first = Some(w.t_enter);
        break;
    }
    if let Some((_, t)) = outer
        && first.is_none_or(|f| t < f)
    {
        out = PassageOutcomeV1::OuterDomainContact;
        first = Some(t)
    }
    finish(v, qid, out, first, ws)
}
pub fn validate_result(
    r: &PhysicalVolumeRecipeV1,
    v: &PhysicalVolumeV1,
    q: &SweptAabbQueryV1,
    x: &SweptAabbResultV1,
) -> Result<(), SweptAabbError> {
    if &compile_swept_aabb(r, v, q)? != x {
        Err(SweptAabbError::Invalid("result drift"))
    } else {
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct Hit {
    enter: UnitRationalV1,
    exit: UnitRationalV1,
    interior: bool,
    mask: u8,
}
fn intersect(
    start: [i64; 3],
    end: [i64; 3],
    lo: [i64; 3],
    hi: [i64; 3],
) -> Result<Option<Hit>, SweptAabbError> {
    let mut enter = UnitRationalV1::zero();
    let mut exit = UnitRationalV1::one();
    let mut ae = [None; 3];
    let mut boundary = false;
    for a in 0..3 {
        let s = i128::from(start[a]);
        let d = i128::from(end[a]) - s;
        let l = i128::from(lo[a]);
        let h = i128::from(hi[a]);
        if d == 0 {
            if s < l || s > h {
                return Ok(None);
            }
            if s == l || s == h {
                boundary = true
            }
            continue;
        }
        let (re, rx, speed) = if d > 0 {
            (l - s, h - s, d)
        } else {
            (s - h, s - l, -d)
        };
        if rx < 0 || re > speed {
            return Ok(None);
        }
        let en = ratio(re.max(0), speed)?;
        let ex = ratio(rx.min(speed), speed)?;
        if re >= 0 {
            ae[a] = Some(en)
        }
        if en > enter {
            enter = en
        }
        if ex < exit {
            exit = ex
        }
        if enter > exit {
            return Ok(None);
        }
    }
    let mut mask = 0;
    for (a, e) in ae.into_iter().enumerate() {
        if e == Some(enter) {
            mask |= 1 << a
        }
    }
    Ok(Some(Hit {
        enter,
        exit,
        interior: enter < exit && !boundary,
        mask,
    }))
}
fn ratio(n: i128, d: i128) -> Result<UnitRationalV1, SweptAabbError> {
    UnitRationalV1::new(
        u64::try_from(n).map_err(|_| SweptAabbError::Invalid("rational overflow"))?,
        u64::try_from(d).map_err(|_| SweptAabbError::Invalid("rational overflow"))?,
    )
    .map_err(|_| SweptAabbError::Invalid("invalid rational"))
}
fn expanded(
    r: &PhysicalVolumeRecipeV1,
    i: CellIndex3V1,
    half: [i64; 3],
) -> Result<([i64; 3], [i64; 3]), SweptAabbError> {
    let ix = [i.x, i.y, i.z];
    let mut lo = [0; 3];
    let mut hi = [0; 3];
    for a in 0..3 {
        let m = i128::from(r.input.origin_q32_32[a])
            + i128::from(r.input.cell_step_q32_32) * i128::from(ix[a]);
        lo[a] = i64::try_from(m - i128::from(half[a]))
            .map_err(|_| SweptAabbError::Invalid("expanded bounds overflow"))?;
        hi[a] = i64::try_from(m + i128::from(r.input.cell_step_q32_32) + i128::from(half[a]))
            .map_err(|_| SweptAabbError::Invalid("expanded bounds overflow"))?
    }
    Ok((lo, hi))
}
fn inside(p: [i64; 3], b: ([i64; 3], [i64; 3])) -> bool {
    (0..3).all(|a| p[a] > b.0[a] && p[a] < b.1[a])
}
fn outer_event(
    r: &PhysicalVolumeRecipeV1,
    q: &SweptAabbQueryV1,
) -> Result<Option<(bool, UnitRationalV1)>, SweptAabbError> {
    let mut lo = [0; 3];
    let mut hi = [0; 3];
    for a in 0..3 {
        let vm = i128::from(r.input.origin_q32_32[a])
            + i128::from(r.input.cell_step_q32_32) * i128::from(r.input.extent[a]);
        lo[a] = i64::try_from(
            i128::from(r.input.origin_q32_32[a]) + i128::from(q.half_extents_q32_32[a]),
        )
        .map_err(|_| SweptAabbError::Invalid("contracted bounds overflow"))?;
        hi[a] = i64::try_from(vm - i128::from(q.half_extents_q32_32[a]))
            .map_err(|_| SweptAabbError::Invalid("contracted bounds overflow"))?;
        if lo[a] > hi[a] {
            return Ok(Some((true, UnitRationalV1::zero())));
        }
    }
    if !(0..3).all(|a| q.start_q32_32[a] >= lo[a] && q.start_q32_32[a] <= hi[a]) {
        return Ok(Some((true, UnitRationalV1::zero())));
    }
    if (0..3).any(|a| q.start_q32_32[a] == lo[a] || q.start_q32_32[a] == hi[a]) {
        return Ok(Some((false, UnitRationalV1::zero())));
    }
    if (0..3).all(|a| q.end_q32_32[a] > lo[a] && q.end_q32_32[a] < hi[a]) {
        return Ok(None);
    }
    Ok(intersect(q.start_q32_32, q.end_q32_32, lo, hi)?.map(|h| (false, h.exit)))
}
fn validate_query(
    r: &PhysicalVolumeRecipeV1,
    v: &PhysicalVolumeV1,
    q: &SweptAabbQueryV1,
) -> Result<(), SweptAabbError> {
    validate_physical_volume(r, v).map_err(|_| SweptAabbError::Invalid("invalid volume"))?;
    if q.schema_version != CONTRACT_VERSION {
        return Err(SweptAabbError::Invalid("unsupported schema"));
    }
    if q.physical_volume_id != v.physical_volume_id {
        return Err(SweptAabbError::Invalid("volume mismatch"));
    }
    if q.query_source_id == [0; 32] || q.mechanical_profile_source_id == [0; 32] {
        return Err(SweptAabbError::Invalid("zero source id"));
    }
    if q.half_extents_q32_32.iter().any(|x| *x <= 0) {
        return Err(SweptAabbError::Invalid("nonpositive half extent"));
    }
    if q.mechanical_rules.len() > 65_536 {
        return Err(SweptAabbError::Invalid("profile ceiling exceeded"));
    }
    let mut previous_subject_bytes: Option<Vec<u8>> = None;
    for x in &q.mechanical_rules {
        if let CellEvidenceV1::Gas {
            substance_source_id,
        }
        | CellEvidenceV1::Liquid {
            substance_source_id,
        }
        | CellEvidenceV1::Solid {
            substance_source_id,
        } = &x.subject
            && *substance_source_id == [0; 32]
        {
            return Err(SweptAabbError::Invalid("zero substance id"));
        }
        let subject_bytes = encode(&x.subject)?;
        if previous_subject_bytes
            .as_ref()
            .is_some_and(|previous| previous >= &subject_bytes)
        {
            return Err(SweptAabbError::Invalid(
                "mechanical subjects must be unique and canonically ordered",
            ));
        }
        previous_subject_bytes = Some(subject_bytes);
    }
    Ok(())
}
fn lookup(q: &SweptAabbQueryV1, e: &CellEvidenceV1) -> Option<MechanicalDispositionV1> {
    q.mechanical_rules
        .iter()
        .find(|r| &r.subject == e)
        .map(|r| r.disposition)
}
fn reconstruct(r: &PhysicalVolumeRecipeV1) -> Result<Vec<CellEvidenceV1>, SweptAabbError> {
    let count = r.input.extent.iter().try_fold(1_u64, |a, x| {
        a.checked_mul(u64::from(*x))
            .ok_or(SweptAabbError::Invalid("cell count overflow"))
    })?;
    if count > MAX_PHYSICAL_VOLUME_PROOF_CELLS {
        return Err(SweptAabbError::Invalid("cell ceiling exceeded"));
    }
    let mut out = vec![
        r.input.default_evidence.clone();
        usize::try_from(count)
            .map_err(|_| SweptAabbError::Invalid("cell count overflow"))?
    ];
    for run in &r.input.column_runs {
        for z in run.z_start..run.z_start + run.length {
            let i = CellIndex3V1 {
                x: run.x_index,
                y: run.y_index,
                z,
            };
            out[flat(r, i)?] = run.evidence.clone()
        }
    }
    Ok(out)
}
fn flat(r: &PhysicalVolumeRecipeV1, i: CellIndex3V1) -> Result<usize, SweptAabbError> {
    usize::try_from(
        u64::from(i.x) * u64::from(r.input.extent[1]) * u64::from(r.input.extent[2])
            + u64::from(i.y) * u64::from(r.input.extent[2])
            + u64::from(i.z),
    )
    .map_err(|_| SweptAabbError::Invalid("index overflow"))
}
fn finish(
    v: &PhysicalVolumeV1,
    qid: Id,
    outcome: PassageOutcomeV1,
    first_event_t: Option<UnitRationalV1>,
    witnesses: Vec<ExpandedCellWitnessV1>,
) -> Result<SweptAabbResultV1, SweptAabbError> {
    let semantic = encode(&(
        v.physical_volume_id,
        qid,
        outcome,
        first_event_t,
        &witnesses,
    ))?;
    let x = SweptAabbResultV1 {
        schema_version: 1,
        physical_volume_id: v.physical_volume_id,
        query_id: qid,
        outcome,
        first_event_t,
        witnesses,
        result_id: hash(b"mindwarp.swept-aabb.result.v1", &semantic),
        limitations: vec![
            "fixed_orientation_translation_only".into(),
            "bounded_reference_not_runtime_collision".into(),
            "no_response_force_friction_support_buoyancy_drag_or_restitution".into(),
            "no_navigation_walkability_organism_planet_or_biome_authority".into(),
        ],
        authority_effect: "none_evidence_only".into(),
    };
    if encode(&x)?.len() > MAX_CANONICAL_RESULT_BYTES {
        Err(SweptAabbError::Invalid("result byte ceiling exceeded"))
    } else {
        Ok(x)
    }
}
fn encode<T: Serialize>(x: &T) -> Result<Vec<u8>, SweptAabbError> {
    serde_json::to_vec(x).map_err(|e| SweptAabbError::Codec(e.to_string()))
}
fn decode<T: for<'a> Deserialize<'a>>(b: &[u8]) -> Result<T, SweptAabbError> {
    serde_json::from_slice(b).map_err(|e| SweptAabbError::Codec(e.to_string()))
}
fn canonical<T: Serialize>(x: &T, b: &[u8], m: &'static str) -> Result<(), SweptAabbError> {
    if encode(x)? == b {
        Ok(())
    } else {
        Err(SweptAabbError::Invalid(m))
    }
}
fn hash(d: &[u8], b: &[u8]) -> Id {
    let mut h = Sha256::new();
    h.update(d);
    h.update(b);
    h.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use physical_path_substrate::{
        AdjacencyV1, BoundaryModeV1, CoordinateFrameV1, PhysicalVolumeRecipeInputV1,
        compile_physical_volume, compile_physical_volume_recipe,
    };
    const Q: i64 = 1_i64 << 32;
    fn id(x: u8) -> Id {
        [x; 32]
    }
    fn fixture(e: CellEvidenceV1, extent: [u32; 3]) -> (PhysicalVolumeRecipeV1, PhysicalVolumeV1) {
        let i = PhysicalVolumeRecipeInputV1 {
            schema_version: 1,
            recipe_source_id: id(1),
            scope_id: id(2),
            reconstruction_id: id(3),
            recipe_revision: 1,
            coordinate_frame: CoordinateFrameV1::CartesianQ32_32Volume3dV1,
            origin_q32_32: [0; 3],
            cell_step_q32_32: Q,
            extent,
            boundary_mode: BoundaryModeV1::BoundedAbsent,
            adjacency: AdjacencyV1::SharedFace6,
            default_evidence: e,
            column_runs: vec![],
        };
        let r = compile_physical_volume_recipe(&i).unwrap();
        let v = compile_physical_volume(&r).unwrap();
        (r, v)
    }
    fn q(
        v: &PhysicalVolumeV1,
        s: [i64; 3],
        e: [i64; 3],
        rules: Vec<MechanicalRuleV1>,
    ) -> SweptAabbQueryV1 {
        SweptAabbQueryV1 {
            schema_version: 1,
            physical_volume_id: v.physical_volume_id,
            query_source_id: id(8),
            half_extents_q32_32: [Q / 4; 3],
            start_q32_32: s,
            end_q32_32: e,
            motion_kind: MotionKindV1::FixedOrientationTranslation,
            mechanical_profile_source_id: id(9),
            mechanical_rules: rules,
        }
    }
    fn rule(s: CellEvidenceV1, d: MechanicalDispositionV1) -> MechanicalRuleV1 {
        MechanicalRuleV1 {
            subject: s,
            disposition: d,
        }
    }
    #[test]
    fn initial_solid_overlap() {
        let s = CellEvidenceV1::Solid {
            substance_source_id: id(4),
        };
        let (r, v) = fixture(s.clone(), [3, 1, 1]);
        let x = compile_swept_aabb(
            &r,
            &v,
            &q(
                &v,
                [Q / 2; 3],
                [Q * 5 / 2, Q / 2, Q / 2],
                vec![rule(s, MechanicalDispositionV1::BlocksTranslation)],
            ),
        )
        .unwrap();
        assert_eq!(x.outcome, PassageOutcomeV1::InitialInteriorOverlap)
    }
    #[test]
    fn face_slide_contact_only() {
        let hit = intersect([-4, 1, 0], [4, 1, 0], [-1; 3], [1; 3])
            .unwrap()
            .unwrap();
        assert!(!hit.interior);
        assert_eq!(hit.enter, UnitRationalV1::new(3, 8).unwrap());
        assert_eq!(hit.exit, UnitRationalV1::new(5, 8).unwrap());
    }
    #[test]
    fn oracle_hostile_vectors_match_exactly() {
        let cases = [
            ([-4, 0, 0], [4, 0, 0], true, 3, 8, 5, 8),
            ([-4, 1, 1], [4, 1, 1], false, 3, 8, 5, 8),
            ([-1, 0, 0], [-4, 0, 0], false, 0, 1, 0, 1),
            ([-1, 0, 0], [0, 0, 0], true, 0, 1, 1, 1),
        ];
        for (start, end, interior, en, ed, xn, xd) in cases {
            let hit = intersect(start, end, [-1; 3], [1; 3]).unwrap().unwrap();
            assert_eq!(hit.interior, interior);
            assert_eq!(hit.enter, UnitRationalV1::new(en, ed).unwrap());
            assert_eq!(hit.exit, UnitRationalV1::new(xn, xd).unwrap());
        }
    }
    #[test]
    fn unavailable_and_missing_profile_are_typed() {
        let (r, v) = fixture(CellEvidenceV1::Unavailable, [1, 1, 1]);
        assert_eq!(
            compile_swept_aabb(&r, &v, &q(&v, [Q / 2; 3], [Q / 2; 3], vec![]))
                .unwrap()
                .outcome,
            PassageOutcomeV1::UnavailableEvidence
        );
        let (r, v) = fixture(CellEvidenceV1::Vacuum, [1, 1, 1]);
        assert_eq!(
            compile_swept_aabb(&r, &v, &q(&v, [Q / 2; 3], [Q / 2; 3], vec![]))
                .unwrap()
                .outcome,
            PassageOutcomeV1::InteractionModelRequired
        )
    }
    #[test]
    fn phase_does_not_default_mechanics() {
        let (r, v) = fixture(
            CellEvidenceV1::Gas {
                substance_source_id: id(5),
            },
            [1, 1, 1],
        );
        assert_eq!(
            compile_swept_aabb(&r, &v, &q(&v, [Q / 2; 3], [Q / 2; 3], vec![]))
                .unwrap()
                .outcome,
            PassageOutcomeV1::InteractionModelRequired
        )
    }
    #[test]
    fn codec_and_forgery_fail() {
        let (r, v) = fixture(CellEvidenceV1::Vacuum, [1, 1, 1]);
        let q = q(
            &v,
            [Q / 2; 3],
            [Q / 2; 3],
            vec![rule(
                CellEvidenceV1::Vacuum,
                MechanicalDispositionV1::DoesNotBlockTranslation,
            )],
        );
        let b = q.to_bytes(&r, &v).unwrap();
        assert_eq!(SweptAabbQueryV1::from_bytes(&r, &v, &b).unwrap(), q);
        let mut j: serde_json::Value = serde_json::from_slice(&b).unwrap();
        j.as_object_mut()
            .unwrap()
            .insert("unknown".into(), 1.into());
        assert!(SweptAabbQueryV1::from_bytes(&r, &v, &serde_json::to_vec(&j).unwrap()).is_err());
        let mut x = compile_swept_aabb(&r, &v, &q).unwrap();
        x.result_id = id(99);
        assert!(validate_result(&r, &v, &q, &x).is_err())
    }
    #[test]
    fn unsupported_and_oversized_are_explicit() {
        let (r, v) = fixture(CellEvidenceV1::Vacuum, [1, 1, 1]);
        let mut x = q(&v, [Q / 2; 3], [Q / 2; 3], vec![]);
        x.motion_kind = MotionKindV1::UnsupportedRotation;
        assert_eq!(
            compile_swept_aabb(&r, &v, &x).unwrap().outcome,
            PassageOutcomeV1::UnsupportedMotion
        );
        let mut x = q(&v, [Q / 2; 3], [Q / 2; 3], vec![]);
        x.half_extents_q32_32 = [i64::MAX; 3];
        assert_eq!(
            compile_swept_aabb(&r, &v, &x).unwrap().outcome,
            PassageOutcomeV1::InitialOuterDomainOverlap
        )
    }
    #[test]
    fn maximum_cell_fixture_stays_bounded() {
        let (r, v) = fixture(CellEvidenceV1::Vacuum, [64, 32, 32]);
        assert_eq!(v.cell_count, 65_536);
        let x = compile_swept_aabb(
            &r,
            &v,
            &q(
                &v,
                [Q / 2; 3],
                [Q / 2; 3],
                vec![rule(
                    CellEvidenceV1::Vacuum,
                    MechanicalDispositionV1::DoesNotBlockTranslation,
                )],
            ),
        )
        .unwrap();
        assert!(
            x.to_bytes(
                &r,
                &v,
                &q(
                    &v,
                    [Q / 2; 3],
                    [Q / 2; 3],
                    vec![rule(
                        CellEvidenceV1::Vacuum,
                        MechanicalDispositionV1::DoesNotBlockTranslation
                    )]
                )
            )
            .unwrap()
            .len()
                < MAX_CANONICAL_RESULT_BYTES
        );
    }
    #[test]
    fn true_expansion_overflow_fails_closed() {
        let input = PhysicalVolumeRecipeInputV1 {
            schema_version: 1,
            recipe_source_id: id(1),
            scope_id: id(2),
            reconstruction_id: id(3),
            recipe_revision: 1,
            coordinate_frame: CoordinateFrameV1::CartesianQ32_32Volume3dV1,
            origin_q32_32: [i64::MAX - Q; 3],
            cell_step_q32_32: Q,
            extent: [1; 3],
            boundary_mode: BoundaryModeV1::BoundedAbsent,
            adjacency: AdjacencyV1::SharedFace6,
            default_evidence: CellEvidenceV1::Vacuum,
            column_runs: vec![],
        };
        let r = compile_physical_volume_recipe(&input).unwrap();
        let v = compile_physical_volume(&r).unwrap();
        let centre = [i64::MAX - Q / 2; 3];
        let x = q(
            &v,
            centre,
            centre,
            vec![rule(
                CellEvidenceV1::Vacuum,
                MechanicalDispositionV1::DoesNotBlockTranslation,
            )],
        );
        assert_eq!(
            compile_swept_aabb(&r, &v, &x),
            Err(SweptAabbError::Invalid("expanded bounds overflow"))
        );
    }
}
