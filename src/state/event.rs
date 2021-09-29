use super::ProjectSaveState;

#[derive(Debug, Clone)]
pub enum StateSystemEvent {
    Transport(TransportEvent),
    Tempo(TempoEvent),
    Project(ProjectEvent),
}

// TODO: Remove this once tuix removes the `PartialEq` requirement
// on messages.
impl PartialEq for StateSystemEvent {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
pub enum ProjectEvent {
    LoadProject(Box<ProjectSaveState>),
}

#[derive(Debug, Clone)]
pub enum TempoEvent {
    SetBPM(f64),
}

#[derive(Debug, Clone)]
pub enum TransportEvent {
    Play,
    Stop,
    Pause,
}

impl From<ProjectEvent> for StateSystemEvent {
    fn from(e: ProjectEvent) -> Self {
        Self::Project(e)
    }
}

impl From<TempoEvent> for StateSystemEvent {
    fn from(e: TempoEvent) -> Self {
        Self::Tempo(e)
    }
}

impl From<TransportEvent> for StateSystemEvent {
    fn from(e: TransportEvent) -> Self {
        Self::Transport(e)
    }
}
