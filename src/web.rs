use futures::future;
use hyper::rt::{Future, Stream};
use hyper::{Body, Chunk, Method, Request, Response, StatusCode};
use std::collections::HashMap;
use url::form_urlencoded;

/// We need to return different futures depending on the route matched,
/// and we can do that with an enum, such as `futures::Either`, or with
/// trait objects.
///
/// A boxed Future (trait object) is used as it is easier to understand
/// and extend with more types. Advanced users could switch to `Either`.
type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

pub fn handle(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::POST, "/") => {
            let reversed = req
                .into_body()
                .concat2()
                .map(|chunk| {
                    let params = post_params(&chunk);

                    println!("Call = {:?}", call_params(&params));

                    Response::new(Body::from("Hi!"))
                });

            return Box::new(reversed);
        }

        // The 404 Not Found route...
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Box::new(future::ok(response))
}

fn post_params(chunk: &Chunk) -> HashMap<String, String> {
    let chunks = chunk.iter().cloned().collect::<Vec<u8>>();
    
    form_urlencoded::parse(chunks.as_ref())
        .into_owned()
        .collect::<HashMap<String, String>>()
}

#[derive(Debug)]
struct PhoneCall {
    caller: String,
}

fn call_params(params: &HashMap<String, String>) -> Option<PhoneCall> {
    /*
        CallSid
        AccountSid
        From
        To
        CallStatus
        ApiVersion
        Direction
        ForwardedFrom
        CallerName
        ParentCallSid
    */

    params
        .get("From")
        .map(|caller| PhoneCall { caller: caller.to_string() })
}
