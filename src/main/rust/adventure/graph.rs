use crate::adventure::script;
use std::collections::HashMap;

use crate::types::*;

pub struct Adventure<A: VoiceOver> {
    pub script: script::Script,
    pub voice_over: A,
}

impl <A: VoiceOver> Adventure<A> {
    fn verify_voice_overs(&self) -> Result<(), String> {
        for scene_name in self.script.scenes.keys() {
            match self.voice_over.of_scene(scene_name) {
                Ok(_) => continue,
                e     => return e,
            }
        }

        Ok(())
    }

    // TODO: Add methods for accessing the graph
}

pub trait VoiceOver {
    fn of_scene(&self, script: &script::Script, scene_name: &script::SceneName) -> Result<AudioFile, String>;
}

pub struct LocalVoiceOver {
    pub recordings: HashMap<script::SceneName, AudioFile>
}

impl LocalVoiceOver {
    pub fn from_directory(script: &script::Script, recordings_directory: String) -> Result<LocalVoiceOver, String> {
        panic!("Unimplemented");
    }
}

impl VoiceOver for LocalVoiceOver {
    fn of_scene(&self, scene: &script::Scene) -> Result<AudioFile, String> {
        // TODO: Fix to use scene
        self.recordings
            .get(scene)
            .ok_or(String::from(format!("SceneName not found: {}", scene)))
    }
}

pub struct VoiceryVoiceOver {
    pub api_key: String,
}

impl VoiceOver for VoiceryVoiceOver {
    fn of_scene(&self, scene: &script::Scene) -> Result<AudioFile, String> {
        // TODO:
        panic!("Unimplemented");
    }
}
