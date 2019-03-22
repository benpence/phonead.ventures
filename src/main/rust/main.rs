extern crate phone_adventures;
#[macro_use]
extern crate rouille;

use phone_adventures::adventure::machine;
use phone_adventures::adventure::reading;
use phone_adventures::adventure::script;
use phone_adventures::sessions;
use phone_adventures::twilio::planner;
use phone_adventures::web;
use std::io;
use std::sync;

fn main() {
    println!("Now listening on 0.0.0.0:8888");

    let handler = web::Handler {
        machine: Box::new(machine::ScriptMachine {
            sessions: Box::new(sessions::InMemorySessions::new()),
            // TODO: Move before
            readings: load_readings("data/arthur-and-jane/").map_err(|e| panic!(e) ).unwrap(),
        }),
        planner: Box::new(planner::TwilioPlanner {
            base_url: String::from("https://phonead.ventures"),
        }),
    };

    let handler_mutex = sync::Mutex::new(handler);

    rouille::start_server("0.0.0.0:8888", move |request| {
        rouille::log(&request, io::stdout(), || {
            if let Some(modified_request) = request.remove_prefix("/static") {
                rouille::match_assets(&modified_request, "src/main/resources/static")
            } else {
                router!(request,                                    
                    (POST) (/) => {  
                        let mut handler = handler_mutex.lock().unwrap();
                        handler.handle(request)
                    },
        
                    _ => rouille::Response::empty_404()
                )
            }
        })
    });
}

fn load_readings(readings_dir: &str) -> Result<Vec<(script::ScriptName, reading::Reading)>, String> {
    // TODO: Load each readings_dir in this dir
    let reading = reading::Reading::try_from_directory(readings_dir)?;

    Ok(vec![(reading.script_name.to_string(), reading)])
}

