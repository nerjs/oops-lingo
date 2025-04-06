use dioxus::signals::{Readable, Signal, Writable};

#[derive(Clone, Debug, PartialEq)]
pub enum ProcessingState {
    InterpretationOptions(Vec<String>),
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum LingoState {
    #[default]
    UserInput,
    Processing {
        user_input: String,
        processing_state: ProcessingState,
    },
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct StateInfo {
    pub state: LingoState,
    pub error: Option<String>,
    pub loading: bool,
}

#[derive(Clone)]
pub struct StateInfoUpdater {
    state: StateInfo,
    signal: Signal<StateInfo>,
}

impl StateInfoUpdater {
    fn new(signal: &Signal<StateInfo>) -> Self {
        Self {
            state: signal.read().clone(),
            signal: signal.clone(),
        }
    }

    pub fn with_loading(mut self, loading: bool) -> Self {
        self.state.loading = loading;
        self
    }

    pub fn with_error<T>(mut self, text_error: T) -> Self
    where
        T: Into<String>,
    {
        self.state.error = Some(text_error.into());
        self
    }

    pub fn without_error(mut self) -> Self {
        self.state.error = None;
        self
    }

    pub fn with_state(mut self, state: LingoState) -> Self {
        self.state.state = state;
        self
    }

    pub fn update(mut self) -> Self {
        *self.signal.write() = self.state.clone();
        self
    }
}

pub trait UpdateSignal {
    fn update(&self) -> StateInfoUpdater;
}

impl UpdateSignal for Signal<StateInfo> {
    fn update(&self) -> StateInfoUpdater {
        StateInfoUpdater::new(self)
    }
}
