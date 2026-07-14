use minicbor::{Decoder, Encoder};
use serde::Serialize;

use crate::{CONTRACT_VERSION, HierarchyHistoryError, codec, hash};

const DESCRIPTOR_DOMAIN: &[u8] = b"mindwarp/hierarchy-history/descriptor/v1\0";
const CHILD_LOGICAL_DOMAIN: &[u8] = b"mindwarp/hierarchy-history/fixture-child/v1\0";
const CHILD_DESCRIPTOR_DOMAIN: &[u8] = b"mindwarp/hierarchy-history/fixture-child-descriptor/v1\0";
const MAX_RECIPE_BYTES: usize = 4096;
pub const MAX_CHILD_WINDOW: u16 = 256;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[repr(u8)]
pub enum DescriptorOrigin {
    Procedural = 0,
    Dynamic = 1,
}

impl TryFrom<u8> for DescriptorOrigin {
    type Error = HierarchyHistoryError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Procedural),
            1 => Ok(Self::Dynamic),
            _ => Err(HierarchyHistoryError::Invalid("descriptor origin")),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct HierarchyDescriptor {
    pub logical_id: [u8; 32],
    pub parent_logical_id: Option<[u8; 32]>,
    pub reconstruction_fingerprint: [u8; 32],
    pub world_conditions_contract: [u8; 32],
    pub world_conditions_fingerprint: [u8; 32],
    pub origin: DescriptorOrigin,
    pub recipe: Vec<u8>,
}

impl HierarchyDescriptor {
    pub fn new(
        logical_id: [u8; 32],
        parent_logical_id: Option<[u8; 32]>,
        reconstruction_fingerprint: [u8; 32],
        world_conditions_contract: [u8; 32],
        world_conditions_fingerprint: [u8; 32],
        origin: DescriptorOrigin,
        recipe: Vec<u8>,
    ) -> Result<Self, HierarchyHistoryError> {
        if recipe.is_empty() || recipe.len() > MAX_RECIPE_BYTES {
            return Err(HierarchyHistoryError::Invalid("descriptor recipe length"));
        }
        Ok(Self {
            logical_id,
            parent_logical_id,
            reconstruction_fingerprint,
            world_conditions_contract,
            world_conditions_fingerprint,
            origin,
            recipe,
        })
    }

    pub fn encode_canonical(&self) -> Result<Vec<u8>, HierarchyHistoryError> {
        let mut out = Vec::new();
        let mut encoder = Encoder::new(&mut out);
        encoder
            .array(8)
            .and_then(|e| e.u16(CONTRACT_VERSION))
            .and_then(|e| e.bytes(&self.logical_id))
            .map_err(codec)?;
        match self.parent_logical_id {
            Some(parent) => encoder.bytes(&parent).map_err(codec)?,
            None => encoder.null().map_err(codec)?,
        };
        encoder
            .bytes(&self.reconstruction_fingerprint)
            .and_then(|e| e.bytes(&self.world_conditions_contract))
            .and_then(|e| e.bytes(&self.world_conditions_fingerprint))
            .and_then(|e| e.u8(self.origin as u8))
            .and_then(|e| e.bytes(&self.recipe))
            .map_err(codec)?;
        Ok(out)
    }

    pub fn decode_strict(bytes: &[u8]) -> Result<Self, HierarchyHistoryError> {
        let mut decoder = Decoder::new(bytes);
        if decoder.array().map_err(codec)? != Some(8)
            || decoder.u16().map_err(codec)? != CONTRACT_VERSION
        {
            return Err(HierarchyHistoryError::Invalid("descriptor envelope"));
        }
        let logical_id = bytes32(decoder.bytes().map_err(codec)?)?;
        let parent_logical_id = match decoder.datatype().map_err(codec)? {
            minicbor::data::Type::Null => {
                decoder.null().map_err(codec)?;
                None
            }
            minicbor::data::Type::Bytes => Some(bytes32(decoder.bytes().map_err(codec)?)?),
            _ => return Err(HierarchyHistoryError::Invalid("descriptor parent")),
        };
        let descriptor = Self::new(
            logical_id,
            parent_logical_id,
            bytes32(decoder.bytes().map_err(codec)?)?,
            bytes32(decoder.bytes().map_err(codec)?)?,
            bytes32(decoder.bytes().map_err(codec)?)?,
            DescriptorOrigin::try_from(decoder.u8().map_err(codec)?)?,
            decoder.bytes().map_err(codec)?.to_vec(),
        )?;
        if decoder.position() != bytes.len() || descriptor.encode_canonical()? != bytes {
            return Err(HierarchyHistoryError::Invalid("noncanonical descriptor"));
        }
        Ok(descriptor)
    }

    pub fn fingerprint(&self) -> Result<[u8; 32], HierarchyHistoryError> {
        Ok(hash(DESCRIPTOR_DOMAIN, &self.encode_canonical()?))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct ChildCursor {
    pub parent_descriptor: [u8; 32],
    pub child_kind: u16,
    pub next_index: u64,
}

impl ChildCursor {
    pub fn encode_canonical(&self) -> Result<Vec<u8>, HierarchyHistoryError> {
        let mut out = Vec::new();
        Encoder::new(&mut out)
            .array(4)
            .and_then(|e| e.u16(CONTRACT_VERSION))
            .and_then(|e| e.bytes(&self.parent_descriptor))
            .and_then(|e| e.u16(self.child_kind))
            .and_then(|e| e.u64(self.next_index))
            .map_err(codec)?;
        Ok(out)
    }

    pub fn decode_strict(bytes: &[u8]) -> Result<Self, HierarchyHistoryError> {
        let mut decoder = Decoder::new(bytes);
        if decoder.array().map_err(codec)? != Some(4)
            || decoder.u16().map_err(codec)? != CONTRACT_VERSION
        {
            return Err(HierarchyHistoryError::Invalid("cursor envelope"));
        }
        let cursor = Self {
            parent_descriptor: bytes32(decoder.bytes().map_err(codec)?)?,
            child_kind: decoder.u16().map_err(codec)?,
            next_index: decoder.u64().map_err(codec)?,
        };
        if cursor.child_kind == 0
            || decoder.position() != bytes.len()
            || cursor.encode_canonical()? != bytes
        {
            return Err(HierarchyHistoryError::Invalid("noncanonical cursor"));
        }
        Ok(cursor)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct ChildDescriptorRef {
    pub index: u64,
    pub logical_id: [u8; 32],
    pub descriptor_fingerprint: [u8; 32],
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ObservationWindow {
    pub parent_descriptor: [u8; 32],
    pub child_kind: u16,
    pub start_index: u64,
    pub requested_limit: u16,
    pub children: Vec<ChildDescriptorRef>,
    pub next_cursor: Option<ChildCursor>,
    pub has_more: bool,
    pub examined: u16,
}

pub fn observe_fixture_window(
    parent: &HierarchyDescriptor,
    child_kind: u16,
    cursor: Option<ChildCursor>,
    limit: u16,
    logical_count: u64,
    work_budget: u16,
) -> Result<ObservationWindow, HierarchyHistoryError> {
    if child_kind == 0 || limit == 0 || limit > MAX_CHILD_WINDOW {
        return Err(HierarchyHistoryError::Invalid("window kind/limit"));
    }
    let parent_descriptor = parent.fingerprint()?;
    let start = if let Some(cursor) = cursor {
        if cursor.parent_descriptor != parent_descriptor || cursor.child_kind != child_kind {
            return Err(HierarchyHistoryError::StaleCursor);
        }
        cursor.next_index
    } else {
        0
    };
    if start > logical_count {
        return Err(HierarchyHistoryError::StaleCursor);
    }
    let count = logical_count.saturating_sub(start).min(u64::from(limit)) as u16;
    if work_budget < count {
        return Err(HierarchyHistoryError::Cancelled);
    }
    let mut children = Vec::with_capacity(usize::from(count));
    for offset in 0..u64::from(count) {
        let index = start
            .checked_add(offset)
            .ok_or(HierarchyHistoryError::Invalid("index"))?;
        let mut preimage = Vec::with_capacity(42);
        preimage.extend_from_slice(&parent.logical_id);
        preimage.extend_from_slice(&child_kind.to_be_bytes());
        preimage.extend_from_slice(&index.to_be_bytes());
        let logical_id = hash(CHILD_LOGICAL_DOMAIN, &preimage);
        preimage.extend_from_slice(&parent_descriptor);
        children.push(ChildDescriptorRef {
            index,
            logical_id,
            descriptor_fingerprint: hash(CHILD_DESCRIPTOR_DOMAIN, &preimage),
        });
    }
    let end = start
        .checked_add(u64::from(count))
        .ok_or(HierarchyHistoryError::Invalid("window end"))?;
    let has_more = end < logical_count;
    Ok(ObservationWindow {
        parent_descriptor,
        child_kind,
        start_index: start,
        requested_limit: limit,
        children,
        next_cursor: has_more.then_some(ChildCursor {
            parent_descriptor,
            child_kind,
            next_index: end,
        }),
        has_more,
        examined: count,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum ResidencyState {
    Cold,
    Warm,
    Evicted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct MaterializationReceipt {
    pub descriptor_fingerprint: [u8; 32],
    pub state: ResidencyState,
    pub measured_cost_units: u64,
}

impl MaterializationReceipt {
    pub fn for_descriptor(
        descriptor: &HierarchyDescriptor,
        state: ResidencyState,
        measured_cost_units: u64,
    ) -> Result<Self, HierarchyHistoryError> {
        Ok(Self {
            descriptor_fingerprint: descriptor.fingerprint()?,
            state,
            measured_cost_units,
        })
    }
}

fn bytes32(bytes: &[u8]) -> Result<[u8; 32], HierarchyHistoryError> {
    bytes
        .try_into()
        .map_err(|_| HierarchyHistoryError::Invalid("expected 32 bytes"))
}
