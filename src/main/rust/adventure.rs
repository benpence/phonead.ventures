use crate::types::*;

pub struct DummyStateMachine;

impl AdventureStateMachine for DummyStateMachine {
    fn next_action(&mut self, _caller: &Caller) -> Result<Action, String> {
        Ok(
            Action::Choices(vec![
                Choice {
                    dial_number: 1,
                    description: AudioFilePath { path: "/".to_string() },
                }
            ])
        )
    }

    fn set_action(&mut self, _caller: &Caller, _action: &Action) -> Result<(), String> {
        Ok(())
    }
}
