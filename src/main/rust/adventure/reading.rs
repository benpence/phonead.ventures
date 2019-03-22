use crate::adventure::script;
use std::collections::HashMap;
use std::path;

use crate::types::*;

#[derive(Debug)]
pub struct Reading {
    pub script_name: script::ScriptName,
    pub first_scene: script::SceneName,
    // Terminal scenes must map onto an empty vector
    pub transitions: HashMap<script::SceneName, Vec<script::SceneName>>,
    pub voice_over: VoiceOver,
}

impl Reading {
    pub fn try_from_directory(
        reading_dir: &str
    ) -> Result<Reading, String> {
        let script_path = script::find_in_dir(reading_dir)?;
        let script = script::load(&script_path)?;

        let script_name = script.name.to_string();

        let first_scene = script.first_scene.to_string();

        let mut transitions = HashMap::new();
        for scene in &script.scenes {
            let mut files = Vec::new();
            for transition in &scene.transitions {
                files.push(transition.next_scene.to_string());
            }

            transitions.insert(scene.name.to_string(), files);
        }

        let voice_over = VoiceOver::from_script_and_dir(
            &script,
            reading_dir
        );

        let reading = Reading {
            script_name,
            first_scene,
            transitions,
            voice_over,
        };

        Ok(reading)
    }
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

pub static RECORDING_EXTENSION: &str = ".mp3";

impl VoiceOver {
    pub fn from_script_and_dir(
        script: &script::Script,
        reading_dir: &str
    ) -> VoiceOver {
        let script_name: path::PathBuf =
            [reading_dir, &format!("{}{}", script.name, RECORDING_EXTENSION)]
            .iter()
            .collect();
        let script_name = AudioFile { path: script_name.to_str().unwrap().to_string() };

        let mut scene_content = HashMap::new();
        let mut transitions = HashMap::new();

        for scene in &script.scenes {
            // Recording of scene
            let path: path::PathBuf =
                [reading_dir, &format!("{}{}", &scene.name, RECORDING_EXTENSION)]
                .iter()
                .collect();
            let scene_file = AudioFile { path: path.to_str().unwrap().to_string() };

            scene_content.insert( scene.name.to_string(), scene_file);

            // Recordings of transitions
            let mut scene_transitions = Vec::new();
            for (i, _) in scene.transitions.iter().enumerate() {
                let path: path::PathBuf =
                    [reading_dir, &format!("{} transition {}{}", &scene.name, i, RECORDING_EXTENSION)]
                    .iter()
                    .collect();

                scene_transitions.push(AudioFile { path: path.to_str().unwrap().to_string() })
            }

            transitions.insert(scene.name.to_string(), scene_transitions);
        }

        VoiceOver { script_name, scene_content, transitions }
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
