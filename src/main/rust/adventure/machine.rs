use crate::adventure::reading;

use crate::types::*;

pub struct DummyAdventureMachine;

impl AdventureMachine for DummyAdventureMachine {
    fn next_action(&mut self, _caller: &Caller) -> Result<Action, String> {
        Ok(Action::Play(AudioFile { path: String::from("static/audio/piano.mp3") }))
    }
}

pub struct ScriptMachine {
    pub sessions: Box<Sessions>,
    pub readings: Vec<reading::Reading>,
}

impl AdventureMachine for ScriptMachine {
    // TODO: Interact w/ sessions
    fn next_action(&mut self, _caller: &Caller) -> Result<Action, String> {
        Ok(Action::Play(AudioFile { path: String::from("static/audio/piano.mp3") }))
    }
}
