use futures::future;

use crate::types::*;

struct TwilioRejectPlanner;

impl CallPlanner for TwilioRejectPlanner {

    fn extract_caller(&self, params: &WebParams) -> Future<Caller> {
        let optCaller = params.body_params.get("From");
        let optDigits = params.body_params.get("Input").and_then(|s| s.parse::<i32>().ok());

        println!("{:?}", params.body_params);

        Box::new(match (optCaller, optDigits) {
            (Some(phone), Some(n)) if 0 < n && n < 10 => future::ok(Caller::CallerWithChoice(*phone, n)),
            (Some(phone), _      )                    => future::ok(Caller::Caller(*phone)),
            (_,           _      )                    => future::err(String::from("HEEEE")),
        })
    }

    fn format_action(&self, action: Action) -> Future<String> {
        let reject = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Response><Reject /></Response>";
        Box::new(future::ok(String::from(reject)))
    }
}
