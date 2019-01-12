use std::collections::HashMap;

#[derive(Debug)]
pub enum Caller {
    Caller(String),
    CallerWithChoice(String, i32),
}

#[derive(Debug)]
pub struct WebParams {
    pub http_headers: HashMap<String, Vec<String>>,
    pub query_params: HashMap<String, String>,
    pub body_params: HashMap<String, String>,
}

#[derive(Debug)]
pub enum Action {
    Play(AudioFile),
    Choices(Vec<Choice>),
}

#[derive(Debug)]
pub struct Choice {
    pub dial_number: i32,
    pub description: AudioFile,
}

#[derive(Debug)]
pub struct AudioFile {
    pub path: String,
}

impl Clone for AudioFile {
    fn clone(&self) -> AudioFile {
        AudioFile { path: self.path.clone() }
    }
}

pub trait Sessions: Send {
    fn get(&self, key: &Vec<u8>) -> Result<Vec<u8>, String>;
    fn put(&mut self, key: &[u8], val: &[u8]) -> Result<(), String>;
}

pub trait AdventureMachine {
    fn next_action(&mut self, caller: &Caller) -> Result<Action, String>;
}

pub trait CallPlanner {
    fn extract_caller(&self, &WebParams) -> Result<Caller, String>;
    fn format_action(&self, action: &Action) -> Result<String, String>;
}
