use crate::adventure::reading;
use crate::adventure::script;
use crate::sessions;

use crate::types::*;

pub struct DummyAdventureMachine;

impl AdventureMachine for DummyAdventureMachine {
    fn next_action(&mut self, _caller: Caller) -> Result<Action, String> {
        Ok(Action::Play(AudioFile { path: String::from("static/audio/piano.mp3") }))
    }
}

pub struct ScriptMachine {
    pub sessions: Box<sessions::Sessions>,
    pub readings: Vec<(script::ScriptName, reading::Reading)>,
}

impl AdventureMachine for ScriptMachine {
    fn next_action(&mut self, caller: Caller) -> Result<Action, String> {
        let state_entry = self.sessions.entry(caller.phone())?;

        // For new callers, start them off on 
        let state = state_entry.or_insert(sessions::ScriptState::ChooseScript);

        println!("{:?} -> {:?}", caller.phone(), state);

        match caller {
            Caller::Caller(_) =>
                ScriptMachine::no_input_transition(&self.readings, state),
            Caller::CallerWithChoice(_, input) =>
                ScriptMachine::input_transition(&self.readings, state, input),
        };

        println!("{:?} -> {:?}", caller.phone(), state);
        
        ScriptMachine::next_action(&self.readings, state)
    }
}

impl ScriptMachine {
    fn get_transitions<'a>(
        readings: &'a [(script::ScriptName, reading::Reading)],
        script_name: &script::ScriptName,
        scene_name: &script::SceneName,
    ) -> Result<(&'a reading::VoiceOver, &'a [script::SceneName]), String> {
        // Valid script name?
        let reading = readings
            .into_iter()
            .find(|(name, _)| name == script_name)
            .map(|tuple| &tuple.1)
            .ok_or(format!("Invalid script: \"{}\"", script_name))?;

        // Valid scene name?
        let voice_over_and_transitions = reading
            .transitions
            .get(scene_name)
            .map(|transitions| (&reading.voice_over, &transitions[..]))
            .ok_or(format!("No transitions in script \"{}\" for scene: \"{}\"", script_name, scene_name))?;

        Ok(voice_over_and_transitions)
    }

    fn no_input_transition(
        readings: &Vec<(script::ScriptName, reading::Reading)>,
        state: &mut sessions::ScriptState
    ) {
        *state = match std::mem::replace(state, sessions::ScriptState::ChooseScript) {
            sessions::ScriptState::BeginScene { script_name, scene_name } => {
                let transitions_result = ScriptMachine::get_transitions(
                    readings,
                    &script_name,
                    &scene_name
                );

                if let Ok((_, transitions)) = transitions_result {
                    // End of story? --> Choose another adventure
                    if transitions.is_empty() {
                        sessions::ScriptState::ChooseScript
                    
                    // Does this input have a matching index in the next scene choices?
                    } else {
                        sessions::ScriptState::ChooseTransition {
                            script_name: script_name.to_string(),
                            scene_name: scene_name.to_string(),
                        }
                    }
                } else {
                    sessions::ScriptState::BeginScene {
                        script_name,
                        scene_name
                    }
                }
            },
            s => s,
        }
    }

    fn input_transition(
        readings: &Vec<(script::ScriptName, reading::Reading)>,
        state: &mut sessions::ScriptState,
        input: usize
    ) {
        *state = match std::mem::replace(state, sessions::ScriptState::ChooseScript)  {

            // Chooses an adventure
            sessions::ScriptState::ChooseScript => {
                // Does this input have a matching index in the readings?
                if let Some((script_name, reading)) = readings.get(input - 1) {
                    // Start first scene of script
                    sessions::ScriptState::BeginScene {
                        script_name: script_name.to_string(),
                        scene_name: reading.first_scene.to_string(),
                    }
                } else {
                    // Keep state the same
                    sessions::ScriptState::ChooseScript
                }
            },

            // Chooses the next scene
            sessions::ScriptState::ChooseTransition { script_name, scene_name } => {
                let transitions_result = ScriptMachine::get_transitions(
                    readings,
                    &script_name,
                    &scene_name
                );

                if let Ok((_, transitions)) = transitions_result {
                    // End of story? --> Choose another adventure
                    if transitions.is_empty() {
                        sessions::ScriptState::ChooseScript
                    
                    // Does this input have a matching index in the next scene choices?
                    } else if let Some(next_scene_name) = transitions.get(input - 1) {
                        sessions::ScriptState::BeginScene {
                            script_name: script_name,
                            scene_name: next_scene_name.to_string(),
                        }
                    } else {
                        sessions::ScriptState::ChooseTransition {
                            script_name,
                            scene_name
                        }
                    }
                } else {
                    sessions::ScriptState::ChooseTransition {
                        script_name,
                        scene_name
                    }
                }
            },

            // This is an invalid combination
            // TODO: Log this
            s => s,
        };
    }


    fn next_action(
        readings: &Vec<(script::ScriptName, reading::Reading)>,
        state: &sessions::ScriptState
    ) -> Result<Action, String> {

        match state {
            // Ask to choose a script
            sessions::ScriptState::ChooseScript => {
                let choices_result: Result<Vec<_>, _> = readings
                    .iter()
                    .enumerate()
                    .map(|(idx, (_, reading))| {
                        let target = &reading::Target::Script;
                        let description_result = reading.voice_over.of(target);

                        description_result.map(|description| Choice { dial_number: idx + 1, description })
                    })
                    .collect();
				
                choices_result.map(|choices| Action::Choices(choices))
            },

            // Play the reconding of a scene
            sessions::ScriptState::BeginScene { script_name, scene_name } => {
                let transitions_result = ScriptMachine::get_transitions(
                    readings,
                    script_name,
                    scene_name
                );

                transitions_result.and_then(|(voice_over, _)| voice_over
                    .of(&reading::Target::Scene(scene_name.to_string()))
                    .map(|scene_file| Action::Play(scene_file))
                )
            },

            // Ask to choose a next scene
            sessions::ScriptState::ChooseTransition { script_name, scene_name } => {
                let (voice_over, transitions) = ScriptMachine::get_transitions(
                    readings,
                    script_name,
                    scene_name
                )?;

                let choices_result: Result<Vec<_>, _> = transitions
                    .iter()
                    .enumerate()
                    .map(|(idx, _)| {
                        let target = &reading::Target::Choice(scene_name.to_string(), idx);
                        let description_result = voice_over.of(target);

                        description_result.map(|description| Choice { dial_number: idx + 1, description })
                    })
                    .collect();

                choices_result.map(|choices| Action::Choices(choices))
            },
        }
    }
}
