use crate::adventure::reading;
use crate::adventure::script;
use crate::sessions;
use std::collections::HashMap;

use crate::types::*;

//pub struct DummyAdventureMachine;
//
//impl AdventureMachine for DummyAdventureMachine {
//    fn next_action(&mut self, _caller: &Caller) -> Result<Action, String> {
//        Ok(Action::Play(AudioFile { path: String::from("static/audio/piano.mp3") }))
//    }
//}

pub struct ScriptMachine {
    pub sessions: Box<sessions::Sessions>,
    pub readings: HashMap<script::ScriptName, reading::Reading>,
}

impl ScriptMachine {
    fn next_scene(caller: Caller, state: &mut sessions::ScriptState) -> Action {
        match (caller, state) {
            (Caller::Caller(_), state @ sessions::ScriptState::AskedForInput) => {
                *state = sessions::ScriptState::AskedForInput;

                Action::Choices(vec![
                    Choice { dial_number: 1, description: String::from("Sassy Susey") },
                    Choice { dial_number: 2, description: String::from("Johny No Names") },
                    Choice { dial_number: 3, description: String::from("Babs Blocks") },
                    Choice { dial_number: 4, description: String::from("James Roy") },
                ])
            },
            (Caller::CallerWithChoice(_, input), state @ sessions::ScriptState::AskedForInput) => {
                *state = sessions::ScriptState::ProvidedInput(input.clone());
                Action::Line(format!("Thank you for pressing {}", input))
            },

            (_, sessions::ScriptState::ProvidedInput(input)) =>
                Action::Line(format!("Thank you for pressing {}", input)),
        }
    }
}

impl AdventureMachine for ScriptMachine {
    fn next_action(&mut self, caller: Caller) -> Result<Action, String> {
        self.sessions
            .entry(caller.phone())
            .map(|state_entry| {
                let state = state_entry.or_insert(sessions::ScriptState::AskedForInput);
                ScriptMachine::next_scene(caller, state)
            })
    }
}
