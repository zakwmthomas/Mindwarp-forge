//! Capability-free typed model and pure state-transition table for
//! `SELECTIVE_LIVING_ENTITY_AGING_DESIGN.md`.
//!
//! This crate implements steps 1-3 of that design document's cheap proof
//! plan only: strict bounded enums/fixed-point progress values, a pure
//! state-transition table, and deterministic/metamorphic property tests.
//! It does not implement population sampling, a species-authored
//! `PresentationProfile`, morph/mesh/shader work, a wall clock, or any
//! mortality/death path; `PresentedStage` has no such variant by
//! construction, so no event sequence in this crate can produce one.

use serde::Serialize;

/// Fixed-point progress unit: parts per thousand, matching the permille
/// convention already used by `field-basis` and `derived-world-rules`.
pub const PERMILLE_MAX: u16 = 1000;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[repr(u8)]
pub enum AgeCohort {
    Young = 0,
    Juvenile = 1,
    Adult = 2,
    Elderly = 3,
}

impl AgeCohort {
    /// Deterministic maturity/elder progress consistent with this cohort.
    /// Used only when an ambient entity is first generated; never used to
    /// silently advance an already-tracked entity.
    pub fn baseline_progress(self) -> (u16, u16) {
        match self {
            Self::Young => (0, 0),
            Self::Juvenile => (PERMILLE_MAX / 2, 0),
            Self::Adult => (PERMILLE_MAX, 0),
            Self::Elderly => (PERMILLE_MAX, PERMILLE_MAX),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum LifecycleMode {
    /// Cheap lane: a stable identity characteristic, never ticked.
    Ambient,
    /// Explicitly tracked relationship entity that progresses biologically.
    Tracked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct LifecycleState {
    pub mode: LifecycleMode,
    pub cohort: AgeCohort,
    pub maturity_permille: u16,
    pub elder_permille: u16,
    pub appearance_lock: bool,
}

impl LifecycleState {
    pub fn ambient(cohort: AgeCohort) -> Self {
        let (maturity_permille, elder_permille) = cohort.baseline_progress();
        Self {
            mode: LifecycleMode::Ambient,
            cohort,
            maturity_permille,
            elder_permille,
            appearance_lock: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum PresentedStage {
    Young,
    Juvenile,
    Adult,
    Elderly,
}

/// Presentation-only projection. Never mutates canonical state and never
/// implies death; the appearance lock only clamps what is shown, never
/// what is true.
pub fn present(state: &LifecycleState) -> PresentedStage {
    if state.maturity_permille == 0 {
        PresentedStage::Young
    } else if state.maturity_permille < PERMILLE_MAX {
        PresentedStage::Juvenile
    } else if state.appearance_lock || state.elder_permille == 0 {
        PresentedStage::Adult
    } else {
        PresentedStage::Elderly
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifecycleEvent {
    /// Converts an ambient entity to a tracked lifecycle, preserving its
    /// exact current cohort and progress (no reroll, no visible change).
    BeginTracking,
    AdvanceMaturity {
        delta_permille: u16,
    },
    AdvanceElder {
        delta_permille: u16,
    },
    SetAppearanceLock {
        locked: bool,
    },
    /// A save/reload cycle. Canonical state must be unchanged by this
    /// event; it exists only to prove reload does not reroll or mutate.
    Reload,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifecycleError {
    InvalidState,
    AmbientEntitiesDoNotTick,
    AmbientEntitiesDoNotUseAppearanceLock,
    AlreadyTracked,
    MaturityAlreadyComplete,
    ElderRequiresCompletedMaturity,
    Overflow,
}

pub fn validate_state(state: &LifecycleState) -> Result<(), LifecycleError> {
    if state.maturity_permille > PERMILLE_MAX
        || state.elder_permille > PERMILLE_MAX
        || (state.elder_permille > 0 && state.maturity_permille < PERMILLE_MAX)
    {
        return Err(LifecycleError::InvalidState);
    }
    Ok(())
}

pub fn apply(
    state: LifecycleState,
    event: LifecycleEvent,
) -> Result<LifecycleState, LifecycleError> {
    validate_state(&state)?;
    match event {
        LifecycleEvent::Reload => Ok(state),
        LifecycleEvent::SetAppearanceLock { locked } => {
            if matches!(state.mode, LifecycleMode::Ambient) {
                return Err(LifecycleError::AmbientEntitiesDoNotUseAppearanceLock);
            }
            Ok(LifecycleState {
                appearance_lock: locked,
                ..state
            })
        }
        LifecycleEvent::BeginTracking => {
            if matches!(state.mode, LifecycleMode::Tracked) {
                return Err(LifecycleError::AlreadyTracked);
            }
            Ok(LifecycleState {
                mode: LifecycleMode::Tracked,
                ..state
            })
        }
        LifecycleEvent::AdvanceMaturity { delta_permille } => {
            if matches!(state.mode, LifecycleMode::Ambient) {
                return Err(LifecycleError::AmbientEntitiesDoNotTick);
            }
            if state.maturity_permille >= PERMILLE_MAX {
                return Err(LifecycleError::MaturityAlreadyComplete);
            }
            let next = state
                .maturity_permille
                .checked_add(delta_permille)
                .ok_or(LifecycleError::Overflow)?;
            Ok(LifecycleState {
                maturity_permille: next.min(PERMILLE_MAX),
                ..state
            })
        }
        LifecycleEvent::AdvanceElder { delta_permille } => {
            if matches!(state.mode, LifecycleMode::Ambient) {
                return Err(LifecycleError::AmbientEntitiesDoNotTick);
            }
            if state.maturity_permille < PERMILLE_MAX {
                return Err(LifecycleError::ElderRequiresCompletedMaturity);
            }
            let next = state
                .elder_permille
                .checked_add(delta_permille)
                .ok_or(LifecycleError::Overflow)?;
            Ok(LifecycleState {
                elder_permille: next.min(PERMILLE_MAX),
                ..state
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tracked_adult() -> LifecycleState {
        let ambient = LifecycleState::ambient(AgeCohort::Adult);
        apply(ambient, LifecycleEvent::BeginTracking).unwrap()
    }

    #[test]
    fn deterministic_replay() {
        let start = tracked_adult();
        let events = [
            LifecycleEvent::AdvanceElder {
                delta_permille: 200,
            },
            LifecycleEvent::SetAppearanceLock { locked: true },
            LifecycleEvent::AdvanceElder {
                delta_permille: 300,
            },
        ];
        let run = |mut state: LifecycleState| -> LifecycleState {
            for event in events {
                state = apply(state, event).unwrap();
            }
            state
        };
        assert_eq!(run(start), run(start));
    }

    #[test]
    fn monotonic_progress_never_decreases() {
        let mut state = tracked_adult();
        let mut last = (state.maturity_permille, state.elder_permille);
        for delta in [50u16, 100, 400, 1000] {
            state = apply(
                state,
                LifecycleEvent::AdvanceElder {
                    delta_permille: delta,
                },
            )
            .unwrap();
            let now = (state.maturity_permille, state.elder_permille);
            assert!(now.0 >= last.0 && now.1 >= last.1);
            last = now;
        }
    }

    #[test]
    fn juveniles_unaffected_by_appearance_lock() {
        let ambient = LifecycleState::ambient(AgeCohort::Juvenile);
        let mut state = apply(ambient, LifecycleEvent::BeginTracking).unwrap();
        assert_eq!(present(&state), PresentedStage::Juvenile);
        state = apply(state, LifecycleEvent::SetAppearanceLock { locked: true }).unwrap();
        assert_eq!(present(&state), PresentedStage::Juvenile);
        state = apply(state, LifecycleEvent::SetAppearanceLock { locked: false }).unwrap();
        assert_eq!(present(&state), PresentedStage::Juvenile);
    }

    #[test]
    fn adult_presentation_clamps_while_locked_and_reveals_after_unlock() {
        let mut state = tracked_adult();
        state = apply(state, LifecycleEvent::SetAppearanceLock { locked: true }).unwrap();
        state = apply(
            state,
            LifecycleEvent::AdvanceElder {
                delta_permille: 400,
            },
        )
        .unwrap();
        assert_eq!(present(&state), PresentedStage::Adult);
        let hidden_elder_progress = state.elder_permille;

        let unlocked = apply(state, LifecycleEvent::SetAppearanceLock { locked: false }).unwrap();
        assert_eq!(present(&unlocked), PresentedStage::Elderly);
        assert_eq!(
            unlocked.elder_permille, hidden_elder_progress,
            "unlock must not rejuvenate or advance progress"
        );
    }

    #[test]
    fn ambient_entities_do_not_tick() {
        let ambient = LifecycleState::ambient(AgeCohort::Adult);
        assert_eq!(
            apply(
                ambient,
                LifecycleEvent::AdvanceMaturity { delta_permille: 1 }
            ),
            Err(LifecycleError::AmbientEntitiesDoNotTick)
        );
        assert_eq!(
            apply(ambient, LifecycleEvent::AdvanceElder { delta_permille: 1 }),
            Err(LifecycleError::AmbientEntitiesDoNotTick)
        );
    }

    #[test]
    fn adoption_preserves_cohort_and_progress_without_reroll() {
        let ambient = LifecycleState::ambient(AgeCohort::Elderly);
        let before = present(&ambient);
        let tracked = apply(ambient, LifecycleEvent::BeginTracking).unwrap();
        assert_eq!(tracked.cohort, ambient.cohort);
        assert_eq!(tracked.maturity_permille, ambient.maturity_permille);
        assert_eq!(tracked.elder_permille, ambient.elder_permille);
        assert_eq!(present(&tracked), before);
        assert_eq!(
            apply(tracked, LifecycleEvent::BeginTracking),
            Err(LifecycleError::AlreadyTracked)
        );
    }

    #[test]
    fn elder_progress_requires_completed_maturity() {
        let ambient = LifecycleState::ambient(AgeCohort::Juvenile);
        let state = apply(ambient, LifecycleEvent::BeginTracking).unwrap();
        assert_eq!(
            apply(state, LifecycleEvent::AdvanceElder { delta_permille: 1 }),
            Err(LifecycleError::ElderRequiresCompletedMaturity)
        );
    }

    #[test]
    fn maturity_saturates_then_rejects_further_advance() {
        let ambient = LifecycleState::ambient(AgeCohort::Young);
        let mut state = apply(ambient, LifecycleEvent::BeginTracking).unwrap();
        state = apply(
            state,
            LifecycleEvent::AdvanceMaturity {
                delta_permille: 900,
            },
        )
        .unwrap();
        state = apply(
            state,
            LifecycleEvent::AdvanceMaturity {
                delta_permille: 900,
            },
        )
        .unwrap();
        assert_eq!(state.maturity_permille, PERMILLE_MAX);
        assert_eq!(
            apply(state, LifecycleEvent::AdvanceMaturity { delta_permille: 1 }),
            Err(LifecycleError::MaturityAlreadyComplete)
        );
    }

    #[test]
    fn reload_is_identity_and_never_rerolls() {
        let state = tracked_adult();
        assert_eq!(apply(state, LifecycleEvent::Reload).unwrap(), state);
    }

    #[test]
    fn overflow_fails_closed() {
        let mut state = tracked_adult();
        state = apply(
            state,
            LifecycleEvent::AdvanceElder {
                delta_permille: 1000,
            },
        )
        .unwrap();
        assert_eq!(
            apply(
                state,
                LifecycleEvent::AdvanceElder {
                    delta_permille: u16::MAX
                }
            ),
            Err(LifecycleError::Overflow)
        );
    }

    #[test]
    fn invalid_public_state_and_ambient_lock_fail_closed() {
        let invalid = LifecycleState {
            maturity_permille: PERMILLE_MAX + 1,
            ..LifecycleState::ambient(AgeCohort::Adult)
        };
        assert_eq!(
            apply(invalid, LifecycleEvent::Reload),
            Err(LifecycleError::InvalidState)
        );
        assert_eq!(
            apply(
                LifecycleState::ambient(AgeCohort::Elderly),
                LifecycleEvent::SetAppearanceLock { locked: true }
            ),
            Err(LifecycleError::AmbientEntitiesDoNotUseAppearanceLock)
        );
    }
}
