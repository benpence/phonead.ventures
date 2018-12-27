extern crate phone_adventures;
#[macro_use]
extern crate rouille;

use phone_adventures::adventure;
use phone_adventures::twilio::planner;
use phone_adventures::web;
use std::io;

fn main() {
    println!("Now listening on 0.0.0.0:8888");

    rouille::start_server("0.0.0.0:8888", move |request| {
        rouille::log(&request, io::stdout(), || {
            if let Some(modified_request) = request.remove_prefix("/static") {
                rouille::match_assets(&modified_request, "src/main/resources/static")
            } else {
                router!(request,                                    
                    (POST) (/) => {  
                        let mut handler = web::Handler {
                            adventure: adventure::DummyStateMachine,
                            planner: planner::TwilioPlanner {
                                base_url: String::from("https://phonead.ventures"),
                            },
                        };
        
                        handler.handle(request)
                    },
        
                    _ => rouille::Response::empty_404()
                )
            }
        })
    });
}
