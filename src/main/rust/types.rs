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

pub enum Action {
    Play(AudioFile),
    Choices(Vec<Choice>),
}

pub struct Choice {
    pub dial_number: i32,
    pub description: AudioFile,
}

pub struct AudioFile {
    pub path: String,
}

pub trait AdventureStateMachine {
    fn next_action(&mut self, caller: &Caller) -> Result<Action, String>;
    fn set_action(&mut self, caller: &Caller, action: &Action) -> Result<(), String>;
}

pub trait CallPlanner {
    fn extract_caller(&self, &WebParams) -> Result<Caller, String>;
    fn format_action(&self, action: &Action) -> Result<String, String>;
}
