use futures::future;
use hyper;
use std::collections::HashMap;

pub type Future<A> = Box<future::Future<Item = A, Error = String>>;

#[derive(Debug)]
pub enum Caller {
    Caller(String),
    CallerWithChoice(String, i32),
}

pub struct WebParams {
    pub http_headers: hyper::HeaderMap<hyper::header::HeaderValue>,
    pub query_params: HashMap<String, String>,
    pub body_params: HashMap<String, String>,
}

pub enum Action {
    Play(AudioFilePath),
    Choice(Vec<Choice>),
}

pub struct Choice {
    pub dial_number: i32,
    pub description: AudioFilePath
}

pub struct AudioFilePath {
    path: String,
}

pub trait AdventureStateMachine {
    fn next_action(&mut self, phone: String) -> Future<Action>;
    fn process_input(&mut self, phone: String, number: i32) -> Future<Action>;
    fn set_action(&mut self, phone: String, action: Action) -> Future<()>;
}

pub trait CallPlanner {
    fn extract_caller(&self, &WebParams) -> Future<Caller>;
    fn format_action(&self, action: Action) -> Future<String>;
}
