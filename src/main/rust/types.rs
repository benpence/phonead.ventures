use std::collections::HashMap;

#[derive(Debug)]
pub enum Caller {
    Caller(Phone),
    CallerWithChoice(Phone, usize),
}

impl Caller {
    pub fn phone(&self) -> &Phone {
        match self {
            Caller::Caller(phone)              => &phone,
            Caller::CallerWithChoice(phone, _) => &phone,
        }
    }
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

#[derive(Debug)]
pub struct AudioFile {
    pub path: String,
}

impl Clone for AudioFile {
    fn clone(&self) -> AudioFile {
        AudioFile { path: self.path.clone() }
    }
}


pub trait AdventureMachine {
    fn next_action(&mut self, caller: Caller) -> Result<Action, String>;
}

pub trait CallPlanner {
    fn extract_caller(&self, &WebParams) -> Result<Caller, String>;
    fn format_action(&self, action: &Action) -> Result<String, String>;
}
