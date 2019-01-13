use rouille;
use rouille::input::post;
use std::collections::HashMap;
use url::form_urlencoded;

use crate::types::*;

pub struct Handler {
    pub machine: Box<AdventureMachine + Send>,
    pub planner: Box<CallPlanner + Send>,
}

impl Handler {
    pub fn handle(&mut self, request: &rouille::Request) -> rouille::Response {
        let web_params = extract_web_params(request);

        let output_result = self.planner
            .extract_caller(&web_params)
            .and_then(|caller| self.machine.next_action(caller))
            .and_then(|action| self.planner.format_action(&action));

        match output_result {
            Ok(output) => rouille::Response::html(output),
            // TODO: Log error
            Err(_error) => rouille::Response::empty_404()
        }
    }
}

fn extract_web_params(req: &rouille::Request) -> WebParams {
    let http_headers = extract_http_headers(&req);
    let query_params = extract_query_params(&req);
    let body_params = extract_body_params(&req);

    WebParams { http_headers, query_params, body_params }
}

fn extract_http_headers(request: &rouille::Request) -> HashMap<String, Vec<String>> {
    let mut headers: HashMap<String, Vec<String>> = HashMap::new();

    for (key, val) in request.headers() {
        headers
            .entry(key.to_string())
            .or_insert(Vec::new())
            .push(val.to_string());
    }

    headers
}

fn extract_query_params(request: &rouille::Request) -> HashMap<String, String> {
    form_urlencoded::parse(request.raw_query_string().as_bytes())
        .into_owned()
        .collect::<HashMap<String, String>>()
}

fn extract_body_params(request: &rouille::Request) -> HashMap<String, String> {
    post::raw_urlencoded_post_input(request)
        .map(|pairs| {
            let mut map = HashMap::new();

            for (key, val) in pairs {
                map.insert(key, val);
            }

            map
        })
        .unwrap_or(HashMap::new())
}
