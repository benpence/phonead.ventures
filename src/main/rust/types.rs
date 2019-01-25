use std::collections::HashMap;

#[derive(Debug)]
pub struct Caller {
    pub phone: Phone,
    pub dial_number: Option<usize>,
}

pub type Phone = String;

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
    pub dial_number: usize,
    pub description: AudioFile,
}

#[derive(Clone, Debug)]
pub struct AudioFile {
    pub path: String,
}

pub trait AdventureMachine {
    fn next_action(&mut self, caller: &Caller) -> Result<Action, String>;
}

pub trait CallPlanner {
    fn extract_caller(&self, &WebParams) -> Result<Caller, String>;
    fn format_action(&self, action: &Action) -> Result<String, String>;
}
