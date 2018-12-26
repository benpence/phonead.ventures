use rouille;
use rouille::input::post;
use std::collections::HashMap;
//use url::form_urlencoded;
use url::Url;

use crate::types::*;

pub struct Handler <A, B>
where A: AdventureStateMachine,
      B: CallPlanner,
{
    pub adventure: A,
    pub planner: B,
}

impl<A, B> Handler<A, B>
where A: AdventureStateMachine,
      B: CallPlanner,
{
    pub fn handle(&mut self, request: &rouille::Request) -> rouille::Response {
        let web_params = extract_web_params(request);

        let output_result = self.planner
            .extract_caller(&web_params)
            .and_then(|caller| self.adventure.next_action(&caller))
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
    let url_result = Url::parse(request.raw_url());

    let query_params_result = url_result.map(|url| {
        url
            .query_pairs()
            .map(|(cow_key, cow_val)| (cow_key.to_string(), cow_val.to_string()))
            .collect::<HashMap<String, String>>()
    });

    query_params_result.unwrap_or(HashMap::new())
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
