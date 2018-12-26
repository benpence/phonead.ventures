use crate::types::*;

pub struct DummyStateMachine;

impl AdventureStateMachine for DummyStateMachine {
    fn next_action(&mut self, _caller: &Caller) -> Result<Action, String> {
        Ok(Action::Play(AudioFile { path: String::from("static/audio/piano.mp3") }))
    }

    fn set_action(&mut self, _caller: &Caller, _action: &Action) -> Result<(), String> {
        Ok(())
    }
}
