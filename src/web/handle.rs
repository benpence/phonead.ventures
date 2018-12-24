use futures::future;
use hyper;
use hyper::rt;
use hyper::rt::{Future as HFuture, Stream};
use std::collections::HashMap;
use url::form_urlencoded;
use url::Url;

use crate::types::*;

/// We need to return different futures depending on the route matched,
/// and we can do that with an enum, such as `futures::Either`, or with
/// trait objects.
///
/// A boxed Future (trait object) is used as it is easier to understand
/// and extend with more types. Advanced users could switch to `Either`.
type BoxFut = Box<rt::Future<Item = hyper::Response<hyper::Body>, Error = hyper::Error> + Send>;


struct Handler <A, B>
where A: AdventureStateMachine,
      B: CallPlanner,
{
    adventure: A,
    planner: B,
}

impl<A, B> Handler<A, B>
where A: AdventureStateMachine,
      B: CallPlanner,
{
    pub fn handle(&self, req: hyper::Request<hyper::Body>) -> Future<hyper::Response<hyper::Body>> {
        match (req.method(), req.uri().path()) {
            (&hyper::Method::POST, "/") => {
                Box::new(extract_params(req)
                    .and_then(|params| self.planner.extract_caller(&params))
                    .and_then(|caller| {
                        match caller {
                            Caller::Caller(phone) => {
                                self.adventure.next_action(phone)
                            },
                            Caller::CallerWithChoice(phone, digit) => {
                                self.adventure.process_input(phone, digit)
                            },
                        }
                    })
                    .and_then(|action| self.planner.format_action(action))
                    .map(|output| hyper::Response::new(hyper::Body::from(output)))
                )
            },

            // The 404 Not Found route...
            _ => {
                let mut response = hyper::Response::new(hyper::Body::empty());
                *response.status_mut() = hyper::StatusCode::NOT_FOUND;
                Box::new(future::ok(response))
            },
        }
    }
}

fn extract_params(request: hyper::Request<hyper::Body>) -> Future<WebParams> {
    Box::new(request.body()
        .concat2()
        .map_err(|e| e.to_string())
        .map(|chunk| {
            let http_headers = *request.headers();

            let query_params = Url::parse(&request.uri().to_string())
                .map(|url| {
                    let params = HashMap::new();
                    for (key, val) in url.query_pairs() {
                        params.insert(key.to_string(), val.to_string());
                    }
                    params
                })
                .unwrap_or(HashMap::new());

            let chunks = chunk.iter().cloned().collect::<Vec<u8>>();
            let body_params = form_urlencoded::parse(chunks.as_ref())
                .into_owned()
                .collect::<HashMap<String, String>>();

            WebParams { http_headers, query_params, body_params }
        })
    )
}
