extern crate phone_adventures;
#[macro_use]
extern crate rouille;
#[macro_use]
extern crate structopt;

use phone_adventures::adventure::machine;
use phone_adventures::adventure::reading;
use phone_adventures::adventure::script;
use phone_adventures::sessions;
use phone_adventures::twilio::planner;
use phone_adventures::util;
use phone_adventures::web;
use std::io;
use std::path;
use std::sync;
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();
    println!("Now listening on {}", args.socket);

    let handler = web::Handler {
        machine: Box::new(machine::ScriptMachine {
            sessions: Box::new(sessions::InMemorySessions::new()),
            // TODO: Move before
            readings: load_readings("src/main/resources/static/")
                .map_err(|e| panic!(e) ).unwrap(),
        }),
        planner: Box::new(planner::TwilioPlanner {
            base_url: args.base_url.to_string(),
            static_dir: path::Path::new("src/main/resources/static/")
        }),
    };

    let handler_mutex = sync::Mutex::new(handler);

    rouille::start_server(args.socket.to_string(), move |request| {
        rouille::log(&request, io::stdout(), || {
            if let Some(modified_request) = request.remove_prefix("/static") {
                rouille::match_assets(&modified_request, "src/main/resources/static/")
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

#[derive(structopt::StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Cli {
    #[structopt(long = "socket")]
    socket: String,

    #[structopt(long = "base_url")]
    base_url: String,
}

/// Load all readings from `readings_dir`. If any of the directories within
/// `readings_dir` do not contain a `.script`, this will fail.
fn load_readings(readings_dir: &str) -> Result<Vec<(script::ScriptName, reading::Reading)>, String> {
    let paths = util::directory_listing(readings_dir).map_err(|e| format!(
        "Unable to read readings dir \"{:?}\": {:?}, ",
        readings_dir,
        e
    ))?;

    let mut readings = Vec::new();

    for path in paths {
        if path.is_dir() {
            if let Some(dir) = path.to_str() {
                let reading = reading::Reading::try_from_directory(dir)?;
                readings.push((reading.script_name.to_string(), reading));
            } else {
                Err(format!("Invalid path: \"{:?}\"", path))?;
            }
        }
    }

    Ok(readings)
}

