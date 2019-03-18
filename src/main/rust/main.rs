extern crate phone_adventures;
#[macro_use]
extern crate rouille;

use phone_adventures::adventure::machine;
use phone_adventures::adventure::reading;
use phone_adventures::adventure::parse;
use phone_adventures::adventure::script;
use phone_adventures::sessions;
use phone_adventures::twilio::planner;
use phone_adventures::web;
use std::collections::HashMap;
use std::io;
use std::sync;

use phone_adventures::types::*;

fn main() {
    println!("{:?}", script::parse("Eric (kindly): What the fuck grace!\n"));
    println!("Now listening on 0.0.0.0:8888");

    let handler = web::Handler {
        machine: Box::new(machine::ScriptMachine {
            sessions: Box::new(sessions::InMemorySessions::new()),
            // TODO: Move before
            readings: load_readings("").map_err(|e| panic!(e) ).unwrap(),
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

fn load_readings(_directory: &str) -> Result<Vec<(script::ScriptName, reading::Reading)>, String> {
    let mut voice_over_transitions: HashMap<String, Vec<AudioFile>> = HashMap::new();
    voice_over_transitions.insert("scene 0".to_string(), vec![AudioFile { path: "scene 1".to_string()}, AudioFile { path: "scene 2".to_string()}]);
    voice_over_transitions.insert("scene 1".to_string(), vec![AudioFile { path: "scene 3".to_string()}, AudioFile { path: "scene 4".to_string()}]);
    voice_over_transitions.insert("scene 2".to_string(), vec![AudioFile { path: "scene 5".to_string()}, AudioFile { path: "scene 6".to_string()}]);
    voice_over_transitions.insert("scene 3".to_string(), vec![]);
    voice_over_transitions.insert("scene 4".to_string(), vec![]);
    voice_over_transitions.insert("scene 5".to_string(), vec![]);
    voice_over_transitions.insert("scene 6".to_string(), vec![]);

    let mut voice_over_scenes: HashMap<String, AudioFile> = HashMap::new();
    voice_over_scenes.insert("scene 0".to_string(), AudioFile { path: "scene 0".to_string()});
    voice_over_scenes.insert("scene 1".to_string(), AudioFile { path: "scene 1".to_string()});
    voice_over_scenes.insert("scene 2".to_string(), AudioFile { path: "scene 2".to_string()});
    voice_over_scenes.insert("scene 3".to_string(), AudioFile { path: "scene 3".to_string()});
    voice_over_scenes.insert("scene 4".to_string(), AudioFile { path: "scene 4".to_string()});
    voice_over_scenes.insert("scene 5".to_string(), AudioFile { path: "scene 5".to_string()});
    voice_over_scenes.insert("scene 6".to_string(), AudioFile { path: "scene 6".to_string()});

    let voice_over = |script_name: String| {
        reading::VoiceOver {
            script_name: AudioFile { path: script_name.clone() },
            scene_content: voice_over_scenes.clone(),
            transitions: voice_over_transitions.clone(),
        }
    };

    let mut transitions: HashMap<String, Vec<String>> = HashMap::new();
    transitions.insert("scene 0".to_string(), vec!["scene 1".to_string(), "scene 2".to_string()]);
    transitions.insert("scene 1".to_string(), vec!["scene 3".to_string(), "scene 4".to_string()]);
    transitions.insert("scene 2".to_string(), vec!["scene 5".to_string(), "scene 6".to_string()]);
    transitions.insert("scene 3".to_string(), vec![]);
    transitions.insert("scene 4".to_string(), vec![]);
    transitions.insert("scene 5".to_string(), vec![]);
    transitions.insert("scene 6".to_string(), vec![]);

    let script = |script_name: String| (script_name.clone(), reading::Reading {
        script_name: script_name.clone(),
        first_scene: String::from("scene 0"),
        transitions: transitions.clone(),
        voice_over: voice_over(script_name),
    });

    Ok(vec![
        script("script 0".to_string()),
        script("script 1".to_string()),
        script("script 2".to_string())
    ])
}
