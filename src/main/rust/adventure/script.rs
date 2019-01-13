use std::collections::HashMap;
use std::vec::Vec;

pub struct Script {
    pub name: ScriptName,
    pub start: SceneName,
    pub scenes: HashMap<SceneName, Scene>,
}

pub type ScriptName = String;
pub type SceneName = String;

pub struct Scene {
    pub dialog: Vec<Line>
}

pub struct Line {
    pub character: String,
    pub tone: Option<String>,
    pub content: String,
    pub transitions: Vec<Transition>,
}

pub struct Transition {
    pub next_scene: SceneName,
    pub content: String,
}

pub fn parse_script(_script_text: &str) -> Result<Script, String> {
    // TODO:
    panic!("Unimplemented");
}
