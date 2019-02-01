use crate::adventure::script;
use std::collections::HashMap;

use crate::types::*;

#[derive(Debug)]
pub struct Reading {
    pub script_name: script::ScriptName,
    pub first_scene: script::SceneName,
    // Terminal scenes must map onto an empty vector
    pub transitions: HashMap<script::SceneName, Vec<script::SceneName>>,
    pub voice_over: VoiceOver,
}

#[derive(Debug)]
pub enum Target {
    Script,
    Scene(script::SceneName),
    Choice(script::SceneName, usize),
}

#[derive(Debug)]
pub struct VoiceOver {
    pub script_name: AudioFile,
    pub scene_content: HashMap<script::SceneName, AudioFile>,
    pub transitions: HashMap<script::SceneName, Vec<AudioFile>>
}

impl VoiceOver {
    pub fn try_from_directory(
        _script: &script::Script,
        _recordings_directory: String
    ) -> Result<VoiceOver, String> {
        panic!("Unimplemented");
    }

    pub fn of(
        &self,
        target: &Target
    ) -> Result<AudioFile, String> {
        match target {
            Target::Script => Ok(self.script_name.clone()),

            Target::Scene(scene_name) => self
                .scene_content
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
