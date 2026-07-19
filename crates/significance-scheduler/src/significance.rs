use minicbor::{Decoder, Encoder};
use serde::Serialize;

use crate::{CONTRACT_VERSION, SignificanceSchedulerError, bytes32, codec, hash};

const PACKET_DOMAIN: &[u8] = b"mindwarp/significance/packet/v1\0";
const POLICY_DOMAIN: &[u8] = b"mindwarp/significance/hysteresis-policy/v1\0";
pub const PROTECT_INTERACTION: u8 = 1;
pub const PROTECT_THREAT: u8 = 2;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[repr(u8)]
pub enum ImportanceTier {
    Dormant = 0,
    Background = 1,
    Visible = 2,
    Critical = 3,
}

impl TryFrom<u8> for ImportanceTier {
    type Error = SignificanceSchedulerError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Dormant),
            1 => Ok(Self::Background),
            2 => Ok(Self::Visible),
            3 => Ok(Self::Critical),
            _ => Err(SignificanceSchedulerError::Invalid("importance tier")),
        }
    }
}

impl ImportanceTier {
    fn lower(self) -> Self {
        match self {
            Self::Critical => Self::Visible,
            Self::Visible => Self::Background,
            Self::Background | Self::Dormant => Self::Dormant,
        }
    }

    fn higher(self) -> Self {
        match self {
            Self::Dormant => Self::Background,
            Self::Background => Self::Visible,
            Self::Visible | Self::Critical => Self::Critical,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct SignalVector {
    pub focus: u16,
    pub visibility: u16,
    pub interaction: u16,
    pub threat: u16,
    pub prediction: u16,
}

impl SignalVector {
    pub fn peak(self) -> u16 {
        [
            self.focus,
            self.visibility,
            self.interaction,
            self.threat,
            self.prediction,
        ]
        .into_iter()
        .max()
        .unwrap_or(0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ImportancePacket {
    pub target_descriptor: [u8; 32],
    pub request_epoch: u64,
    pub signals: SignalVector,
    pub reason_flags: u16,
    pub protection_flags: u8,
}

impl ImportancePacket {
    pub fn new(
        target_descriptor: [u8; 32],
        request_epoch: u64,
        signals: SignalVector,
        reason_flags: u16,
        protection_flags: u8,
    ) -> Result<Self, SignificanceSchedulerError> {
        if target_descriptor == [0; 32]
            || request_epoch == 0
            || protection_flags & !(PROTECT_INTERACTION | PROTECT_THREAT) != 0
        {
            return Err(SignificanceSchedulerError::Invalid("importance packet"));
        }
        Ok(Self {
            target_descriptor,
            request_epoch,
            signals,
            reason_flags,
            protection_flags,
        })
    }

    pub fn encode_canonical(&self) -> Result<Vec<u8>, SignificanceSchedulerError> {
        let mut out = Vec::new();
        let mut encoder = Encoder::new(&mut out);
        encoder
            .array(6)
            .and_then(|e| e.u16(CONTRACT_VERSION))
            .and_then(|e| e.bytes(&self.target_descriptor))
            .and_then(|e| e.u64(self.request_epoch))
            .and_then(|e| e.array(5))
            .and_then(|e| e.u16(self.signals.focus))
            .and_then(|e| e.u16(self.signals.visibility))
            .and_then(|e| e.u16(self.signals.interaction))
            .and_then(|e| e.u16(self.signals.threat))
            .and_then(|e| e.u16(self.signals.prediction))
            .and_then(|e| e.u16(self.reason_flags))
            .and_then(|e| e.u8(self.protection_flags))
            .map_err(codec)?;
        Ok(out)
    }

    pub fn decode_strict(bytes: &[u8]) -> Result<Self, SignificanceSchedulerError> {
        let mut decoder = Decoder::new(bytes);
        if decoder.array().map_err(codec)? != Some(6)
            || decoder.u16().map_err(codec)? != CONTRACT_VERSION
        {
            return Err(SignificanceSchedulerError::NonCanonical);
        }
        let target_descriptor = bytes32(decoder.bytes().map_err(codec)?)?;
        let request_epoch = decoder.u64().map_err(codec)?;
        if decoder.array().map_err(codec)? != Some(5) {
            return Err(SignificanceSchedulerError::NonCanonical);
        }
        let packet = Self::new(
            target_descriptor,
            request_epoch,
            SignalVector {
                focus: decoder.u16().map_err(codec)?,
                visibility: decoder.u16().map_err(codec)?,
                interaction: decoder.u16().map_err(codec)?,
                threat: decoder.u16().map_err(codec)?,
                prediction: decoder.u16().map_err(codec)?,
            },
            decoder.u16().map_err(codec)?,
            decoder.u8().map_err(codec)?,
        )?;
        if decoder.position() != bytes.len() || packet.encode_canonical()? != bytes {
            return Err(SignificanceSchedulerError::NonCanonical);
        }
        Ok(packet)
    }

    pub fn fingerprint(&self) -> Result<[u8; 32], SignificanceSchedulerError> {
        Ok(hash(PACKET_DOMAIN, &self.encode_canonical()?))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct HysteresisPolicy {
    pub enter: [u16; 3],
    pub exit: [u16; 3],
    pub minimum_hold_steps: u16,
}

impl HysteresisPolicy {
    pub fn new(
        enter: [u16; 3],
        exit: [u16; 3],
        minimum_hold_steps: u16,
    ) -> Result<Self, SignificanceSchedulerError> {
        if minimum_hold_steps == 0
            || !enter.windows(2).all(|pair| pair[0] < pair[1])
            || !exit.windows(2).all(|pair| pair[0] < pair[1])
            || exit.iter().zip(enter).any(|(exit, enter)| *exit > enter)
        {
            return Err(SignificanceSchedulerError::Invalid("hysteresis policy"));
        }
        Ok(Self {
            enter,
            exit,
            minimum_hold_steps,
        })
    }

    fn enter_threshold(self, tier: ImportanceTier) -> Option<u16> {
        match tier {
            ImportanceTier::Background => Some(self.enter[0]),
            ImportanceTier::Visible => Some(self.enter[1]),
            ImportanceTier::Critical => Some(self.enter[2]),
            ImportanceTier::Dormant => None,
        }
    }

    fn exit_threshold(self, tier: ImportanceTier) -> Option<u16> {
        match tier {
            ImportanceTier::Background => Some(self.exit[0]),
            ImportanceTier::Visible => Some(self.exit[1]),
            ImportanceTier::Critical => Some(self.exit[2]),
            ImportanceTier::Dormant => None,
        }
    }

    pub fn encode_canonical(&self) -> Result<Vec<u8>, SignificanceSchedulerError> {
        let mut out = Vec::new();
        let mut e = Encoder::new(&mut out);
        e.array(4)
            .and_then(|e| e.u16(CONTRACT_VERSION))
            .and_then(|e| e.array(3))
            .map_err(codec)?;
        for value in self.enter {
            e.u16(value).map_err(codec)?;
        }
        e.array(3).map_err(codec)?;
        for value in self.exit {
            e.u16(value).map_err(codec)?;
        }
        e.u16(self.minimum_hold_steps).map_err(codec)?;
        Ok(out)
    }

    pub fn decode_strict(bytes: &[u8]) -> Result<Self, SignificanceSchedulerError> {
        let mut d = Decoder::new(bytes);
        if d.array().map_err(codec)? != Some(4)
            || d.u16().map_err(codec)? != CONTRACT_VERSION
            || d.array().map_err(codec)? != Some(3)
        {
            return Err(SignificanceSchedulerError::NonCanonical);
        }
        let enter = [
            d.u16().map_err(codec)?,
            d.u16().map_err(codec)?,
            d.u16().map_err(codec)?,
        ];
        if d.array().map_err(codec)? != Some(3) {
            return Err(SignificanceSchedulerError::NonCanonical);
        }
        let exit = [
            d.u16().map_err(codec)?,
            d.u16().map_err(codec)?,
            d.u16().map_err(codec)?,
        ];
        let value = Self::new(enter, exit, d.u16().map_err(codec)?)?;
        if d.position() != bytes.len() || value.encode_canonical()? != bytes {
            return Err(SignificanceSchedulerError::NonCanonical);
        }
        Ok(value)
    }

    pub fn fingerprint(&self) -> Result<[u8; 32], SignificanceSchedulerError> {
        Ok(hash(POLICY_DOMAIN, &self.encode_canonical()?))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct SignificanceState {
    pub(crate) tier: ImportanceTier,
    pub(crate) since_step: u64,
    pub(crate) last_step: u64,
}

impl Default for SignificanceState {
    fn default() -> Self {
        Self {
            tier: ImportanceTier::Dormant,
            since_step: 0,
            last_step: 0,
        }
    }
}

impl SignificanceState {
    pub fn tier(&self) -> ImportanceTier {
        self.tier
    }
    pub fn since_step(&self) -> u64 {
        self.since_step
    }
    pub fn last_step(&self) -> u64 {
        self.last_step
    }
    pub fn advance(
        &mut self,
        packet: &ImportancePacket,
        policy: HysteresisPolicy,
        step: u64,
    ) -> Result<ImportanceTier, SignificanceSchedulerError> {
        if step < self.last_step {
            return Err(SignificanceSchedulerError::Invalid(
                "nonmonotonic significance step",
            ));
        }
        let peak = packet.signals.peak();
        let protected = packet.protection_flags != 0;
        let mut desired = self.tier;
        while desired < ImportanceTier::Critical {
            let next = desired.higher();
            if peak >= policy.enter_threshold(next).unwrap_or(u16::MAX) {
                desired = next;
            } else {
                break;
            }
        }
        if protected {
            desired = ImportanceTier::Critical;
        } else {
            while desired > ImportanceTier::Dormant
                && peak < policy.exit_threshold(desired).unwrap_or(0)
            {
                desired = desired.lower();
            }
        }
        let promotion = desired > self.tier;
        let held = step.saturating_sub(self.since_step) >= u64::from(policy.minimum_hold_steps);
        if promotion || (desired < self.tier && held) {
            self.tier = desired;
            self.since_step = step;
        }
        self.last_step = step;
        Ok(self.tier)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct ConsumerFidelityMap {
    pub consumer: u16,
    pub levels: [u8; 4],
}

impl ConsumerFidelityMap {
    pub fn new(consumer: u16, levels: [u8; 4]) -> Result<Self, SignificanceSchedulerError> {
        if consumer == 0
            || levels.iter().any(|level| *level > 16)
            || !levels.windows(2).all(|pair| pair[0] <= pair[1])
        {
            return Err(SignificanceSchedulerError::Invalid("consumer fidelity map"));
        }
        Ok(Self { consumer, levels })
    }

    pub fn fidelity(self, tier: ImportanceTier) -> u8 {
        self.levels[tier as usize]
    }
}
