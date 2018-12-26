extern crate phone_adventures;
#[macro_use]
extern crate rouille;

use phone_adventures::adventure;
use phone_adventures::web::call;
use phone_adventures::web::route;
use std::io;

fn main() {
    println!("Now listening on 0.0.0.0:8888");

    rouille::start_server("0.0.0.0:8888", move |request| {
        rouille::log(&request, io::stdout(), || {
            router!(request,                                    
                (GET) (/static/{_rest:String}) => {
                    let static_directory = "static";
                    if let Some(modified_request) = request.remove_prefix(&format!("/{}", static_directory)) {
                        println!("{}", modified_request.raw_url());
                        rouille::match_assets(&modified_request, "static")
                    } else {
                        println!("no path");
                        rouille::Response::empty_404()
                    }
                },
    
                (POST) (/) => {  
                    let mut handler = route::Handler {
                        adventure: adventure::DummyStateMachine,
                        planner: call::TwilioRejectPlanner,
                    };
    
                    handler.handle(request)
                },
    
                _ => rouille::Response::empty_404()
            )
        })
    });
}
