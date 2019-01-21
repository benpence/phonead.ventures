use crate::adventure::script;
use std::collections::HashMap;

use crate::types::*;

pub struct Reading {
    pub script_name: script::ScriptName,
    pub first_scene: script::SceneName,
    // Terminal scenes must map onto an empty vector
    pub transitions: HashMap<script::SceneName, Vec<script::SceneName>>,
    pub voice_over: Box<VoiceOver + Send>,
}

impl Reading {

    fn verify_voice_overs(&mut self) -> Result<(), String> {
        self.voice_over.of(&Target::Script)?;

        for (scene_name, next_scenes) in self.transitions.iter() {
            self.voice_over.of(&Target::Scene(scene_name.to_string()))?;

            for (idx, next_scene_name) in next_scenes.iter().enumerate() {
                // TODO: Centralize idx<->dial_number transitions
                self.voice_over.of(&Target::Choice(next_scene_name.to_string(), idx + 1))?;
            }
        }

        Ok(())
    }

    // TODO: Add methods for accessing the graph
}

pub enum Target {
    Script,
    Scene(script::SceneName),
    Choice(script::SceneName, usize),
}

pub trait VoiceOver {
    fn of(
        &self,
        target: &Target,
    ) -> Result<AudioFile, String>;
}

pub struct LocalVoiceOver {
    pub script_name: AudioFile,
    pub scenes: HashMap<script::SceneName, AudioFile>,
    pub transitions: HashMap<script::SceneName, Vec<AudioFile>>
}

impl LocalVoiceOver {
    pub fn from_directory(
        _script: &script::Script,
        _recordings_directory: String
    ) -> Result<LocalVoiceOver, String> {
        panic!("Unimplemented");
    }
}

impl VoiceOver for LocalVoiceOver {
    fn of(
        &self,
        target: &Target
    ) -> Result<AudioFile, String> {
        match target {
            Target::Script => Ok(self.script_name.clone()),

            Target::Scene(scene_name) => self
                .scenes
                .get(scene_name)
                .map(|file| file.clone())
                .ok_or(format!("No recording for scene name: \"{}\"", scene_name)),

            Target::Choice(scene_name, idx) => self
                .transitions
                .get(scene_name)
                .and_then(|files| files.get(idx.clone()))
                .map(|file| file.clone())
                .ok_or(format!("Scene \"{}\" missing recording for transition index \"{}\"", scene_name, idx)),
        }
    }
}

pub struct VoiceryVoiceOver {
    pub api_key: String,
}

impl VoiceOver for VoiceryVoiceOver {
    fn of(
        &self,
        _target: &Target,
    ) -> Result<AudioFile, String> {
        // TODO:
        panic!("Unimplemented");
    }
}

