use crate::adventure::script;
use nom::*;

pub fn script_from_text(script_text: &str) -> Result<Vec<&str>, String> {
    match dashes(script_text) {
        Ok((_, output)) => Ok(output),
        Err(err) => Err(format!("Failed parsing script: {:?}", err)),
    }
}

named!(script<&str, script::Script>,
    do_parse!(
        name: script_name >>
        scenes: many1!(scene) >>
        ({
            let first_scene = scenes.get(0).unwrap().name.to_string();

            script::Script {
                name: name.to_string(),
                first_scene,
                scenes
            }
        })
    )
);

named!(script_name<&str, &str>,
    do_parse!(
        double_dashes >>
        name: ws!(is_not!("=")) >>
        double_dashes >>
        ( name )
    )
);

named!(scene<&str, script::Scene>,
    do_parse!(
        name: scene_name >>
        dialog: many0!(line) >>
        transitions: transitions >>
        ( script::Scene {
            name: name.to_string(),
            dialog,
            transitions
        })
    )
);

named!(scene_name<&str, &str>,
    do_parse!(
        dashes >>
        name: ws!(is_not!("-")) >>
        dashes >>
        ( name )
    )
);

named!(line<&str, script::Line>,
    do_parse!(
        character: take_until_either1!("(:") >>
        tone: opt!(
            delimited!(
                tag!("("),
                take_until1!(")"),
                tag!(")")
            )
        ) >>
        tag!(":") >>
        content: take_until1!("\n") >>
        ( script::Line {
            character: character.to_string(),
            tone: tone.map(|s| s.to_string()),
            content: content.to_string()
        })
    )
);

named!(transitions<&str, Vec<script::Transition>>,
    map!(
        opt!(
            do_parse!(
                tag!("Choices:\n") >>
                transitions: many1!(transition) >>
                ( transitions )
            )
        ),
        |transitions_opt| transitions_opt.unwrap_or(Vec::new())
    )
);

named!(transition<&str, script::Transition>,
    do_parse!(
        tag!("- ") >>
        before: take_until!("[") >>
        next_scene: delimited!(
            tag!("["),
            take_until1!("]"),
            tag!("]")
        ) >>
        after: take_until!("\n") >>
        ( script::Transition {
            next_scene: next_scene.to_string(),
            before: before.to_string(),
            after: after.to_string()
        })
    )
);

named!(dashes<&str, Vec<&str>>, many1!(tag!("-")));
named!(double_dashes<&str,Vec<&str>>, many1!(tag!("=")));
