use crate::adventure::script;
use std::collections::HashMap;

use crate::types::*;

pub struct Reading {
    pub script: script::Script,
    pub voice_over: Box<VoiceOver + Send>,
}

impl Reading {

    fn verify_voice_overs(&mut self) -> Result<(), String> {
        for (scene_name, scene) in self.script.scenes.iter() {
            match self.voice_over.of_scene(&scene_name, &scene) {
                Ok(_)  => continue,
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    // TODO: Add methods for accessing the graph
}

pub trait VoiceOver {
    fn of_scene(
        &mut self,
        scene_name: &script::SceneName,
        scene: &script::Scene
    ) -> Result<AudioFile, String>;
}

pub struct LocalVoiceOver {
    pub recordings: HashMap<script::SceneName, AudioFile>
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
    fn of_scene(
        &mut self,
        scene_name: &script::SceneName,
        _scene: &script::Scene
    ) -> Result<AudioFile, String> {
        self.recordings
            .get(scene_name)
            .map(|audio_file| audio_file.clone())
            .ok_or(String::from(format!("Voice-over for scene \"{}\" not found", scene_name)))
    }
}

pub struct VoiceryVoiceOver {
    pub api_key: String,
}

impl VoiceOver for VoiceryVoiceOver {
    fn of_scene(
        &mut self,
        _scene_name: &script::SceneName,
        _scene: &script::Scene
    ) -> Result<AudioFile, String> {
        // TODO:
        panic!("Unimplemented");
    }
}

