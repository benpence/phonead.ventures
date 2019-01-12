use std::collections::HashMap;
use std::vec::Vec;

pub struct Script {
    pub title: String,
    pub start: SceneName,
    pub scenes: HashMap<SceneName, Scene>,
}

pub struct Scene {
    pub dialog: Vec<Line>
}

pub struct Line {
    pub character: String,
    pub tone: Option<String>,
    pub content: String,
    pub transitions: Vec<Transition>,
}

pub type SceneName = String;

pub struct Transition {
    pub next_scene: SceneName,
    pub content: String,
}

pub fn parse_script(_script_text: &str) -> Result<Script, String> {
    // TODO:
    panic!("Unimplemented");
}
