//! Bounded work orchestration for the solo-studio control plane.

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorkStage {
    Research,
    Design,
    Readiness,
    Implementation,
    Verification,
    Promotion,
    Monitoring,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthorityLane {
    Automatic,
    Delegated,
    BatchForOwner,
    ImmediateAuthorization,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkItem {
    pub id: String,
    pub title: String,
    pub stage: WorkStage,
    pub authority: AuthorityLane,
    pub risk: u8,
    pub blocked: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Brief {
    pub automatic_completed: usize,
    pub immediate: Vec<WorkItem>,
    pub decisions: Vec<WorkItem>,
}

#[derive(Default)]
pub struct ControlPlane {
    items: Vec<WorkItem>,
}

impl ControlPlane {
    pub fn add(&mut self, item: WorkItem) -> Result<(), ControlPlaneError> {
        if self.items.iter().any(|existing| existing.id == item.id) {
            return Err(ControlPlaneError::DuplicateWorkItem(item.id));
        }
        self.items.push(item);
        Ok(())
    }

    pub fn advance(&mut self, id: &str, next: WorkStage) -> Result<(), ControlPlaneError> {
        let item = self
            .items
            .iter_mut()
            .find(|item| item.id == id)
            .ok_or_else(|| ControlPlaneError::UnknownWorkItem(id.into()))?;
        if item.blocked {
            return Err(ControlPlaneError::BlockedWorkItem(id.into()));
        }
        if successor(&item.stage) != Some(next.clone()) {
            return Err(ControlPlaneError::InvalidStageTransition {
                current: item.stage.clone(),
                requested: next,
            });
        }
        item.stage = next;
        Ok(())
    }

    pub fn brief(&self) -> Brief {
        let automatic_completed = self
            .items
            .iter()
            .filter(|item| {
                item.authority == AuthorityLane::Automatic && item.stage == WorkStage::Monitoring
            })
            .count();

        let mut immediate: Vec<_> = self
            .items
            .iter()
            .filter(|item| item.authority == AuthorityLane::ImmediateAuthorization)
            .cloned()
            .collect();
        let mut decisions: Vec<_> = self
            .items
            .iter()
            .filter(|item| item.authority == AuthorityLane::BatchForOwner)
            .cloned()
            .collect();
        immediate.sort_by_key(|item| std::cmp::Reverse(item.risk));
        decisions.sort_by_key(|item| std::cmp::Reverse(item.risk));
        decisions.truncate(5);

        Brief {
            automatic_completed,
            immediate,
            decisions,
        }
    }
}

fn successor(stage: &WorkStage) -> Option<WorkStage> {
    match stage {
        WorkStage::Research => Some(WorkStage::Design),
        WorkStage::Design => Some(WorkStage::Readiness),
        WorkStage::Readiness => Some(WorkStage::Implementation),
        WorkStage::Implementation => Some(WorkStage::Verification),
        WorkStage::Verification => Some(WorkStage::Promotion),
        WorkStage::Promotion => Some(WorkStage::Monitoring),
        WorkStage::Monitoring => None,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ControlPlaneError {
    DuplicateWorkItem(String),
    UnknownWorkItem(String),
    BlockedWorkItem(String),
    InvalidStageTransition {
        current: WorkStage,
        requested: WorkStage,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(id: &str, authority: AuthorityLane, risk: u8) -> WorkItem {
        WorkItem {
            id: id.into(),
            title: id.into(),
            stage: WorkStage::Research,
            authority,
            risk,
            blocked: false,
        }
    }

    #[test]
    fn work_cannot_skip_lifecycle_stages() {
        let mut plane = ControlPlane::default();
        plane
            .add(item("kernel", AuthorityLane::Delegated, 1))
            .unwrap();
        assert!(matches!(
            plane.advance("kernel", WorkStage::Readiness),
            Err(ControlPlaneError::InvalidStageTransition { .. })
        ));
        plane.advance("kernel", WorkStage::Design).unwrap();
    }

    #[test]
    fn normal_brief_is_limited_to_five_highest_risk_decisions() {
        let mut plane = ControlPlane::default();
        for risk in 0..7 {
            plane
                .add(item(
                    &format!("decision-{risk}"),
                    AuthorityLane::BatchForOwner,
                    risk,
                ))
                .unwrap();
        }
        let brief = plane.brief();
        assert_eq!(brief.decisions.len(), 5);
        assert_eq!(brief.decisions[0].risk, 6);
        assert_eq!(brief.decisions[4].risk, 2);
    }

    #[test]
    fn immediate_authorization_is_never_hidden_by_normal_brief_limit() {
        let mut plane = ControlPlane::default();
        plane
            .add(item("publish", AuthorityLane::ImmediateAuthorization, 10))
            .unwrap();
        for risk in 0..7 {
            plane
                .add(item(
                    &format!("decision-{risk}"),
                    AuthorityLane::BatchForOwner,
                    risk,
                ))
                .unwrap();
        }
        let brief = plane.brief();
        assert_eq!(brief.immediate.len(), 1);
        assert_eq!(brief.decisions.len(), 5);
    }
}
