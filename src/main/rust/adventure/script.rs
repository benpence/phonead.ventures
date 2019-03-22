use pom::parser::*;
use std::fs;
use std::vec::Vec;

#[derive(PartialEq, Debug)]
pub struct Script {
    pub name: ScriptName,
    pub first_scene: SceneName,
    pub scenes: Vec<Scene>,
}

pub type ScriptName = String;
pub type SceneName = String;

#[derive(PartialEq, Debug)]
pub struct Scene {
    pub name: SceneName,
    pub dialog: Vec<Line>,
    pub transitions: Vec<Transition>,
}

#[derive(PartialEq, Debug)]
pub struct Transition {
    pub next_scene: SceneName,
    pub before: String,
    pub after: String,
}

#[derive(PartialEq, Debug)]
pub struct Line {
    pub character: String,
    pub tone: Option<String>,
    pub content: String,
}

pub static SCRIPT_EXTENSION: &str = ".script";

pub fn find_in_dir(dir: &str) -> Result<String, String> {
    let read_dir = fs::read_dir(&dir).map_err(|e| format!(
        "Failed to read directory \"{}\": {}",
        dir,
        e
    ))?;

    for dir_entry_result in read_dir {
        let file = dir_entry_result.map_err(|e| format!(
            "Failed to read file info in dir \"{}\": {}",
            dir,
            e
        ))?;

        if file.file_name().to_str().iter().any(|s| s.ends_with(SCRIPT_EXTENSION)) {
            return Ok(file.path().to_str().unwrap().to_string())
        }
    }

    Err(format!("Couldn't find \"{}\" file in \"{}\"", SCRIPT_EXTENSION, dir))
}

pub fn load(filepath: &str) -> Result<Script, String> {
    let contents = fs::read_to_string(filepath)
        .map_err(|e| format!("Failed to load script '{}': {:?}", filepath, e))?;

    parse(&contents)
}

pub fn parse(content: &str) -> Result<Script, String> {
    script().parse(content.as_bytes()).map_err(|e| {
        format!("Failed parsing script: {:?}", e)
    })
}

fn script<'a>() -> Parser<'a, u8, Script> {
    let name = script_name();
    let scenes = (scene() - space()).repeat(1..);

    (name + scenes).map(|(name, scenes)| {
        let first_scene = scenes.get(0).unwrap().name.to_string();

        Script {
            name,
            first_scene,
            scenes,
        }
    })
}

fn script_name<'a>() -> Parser<'a, u8, ScriptName> {
    double_dashes() * char1(none_of(b"=")) - double_dashes() - sym(b'\n')
}

fn scene<'a>() -> Parser<'a, u8, Scene> {
    let name = scene_name();
    let dialog = line().repeat(0..);
    let transitions = transitions();

    (name + dialog - space() + transitions).map(|((name, dialog), transitions)| Scene {
        name,
        dialog,
        transitions,
    })
}

fn scene_name<'a>() -> Parser<'a, u8, SceneName> {
    let title = space() * trimmed(char1(none_of(b"-"))) - space();
    single_dashes() * title - single_dashes() - sym(b'\n')
}

fn line<'a>() -> Parser<'a, u8, Line> {
    let character = trimmed(char1(none_of(b"(:")));
    let tone = trimmed(sym(b'(') * char1(none_of(b")")) - sym(b')')).opt();
    let content = trimmed(char1(none_of(b"\n")));

    let line = character + tone - sym(b':') + content - sym(b'\n');

    line.map(|((character, tone), content)| Line {
        character,
        tone,
        content
    })
}

fn transitions<'a>() -> Parser<'a, u8, Vec<Transition>> {
    let heading = seq(b"Choices:\n");
    let bullets = transition().repeat(1..);

    (heading * bullets).opt().map(|trans_opt| trans_opt.unwrap_or(Vec::new()))
}

fn transition<'a>() -> Parser<'a, u8, Transition> {
    let bullet = seq(b"- ");
    let before = char0(none_of(b"["));
    let next_scene = sym(b'[') * char1(none_of(b"]")) - sym(b']');
    let after = char0(none_of(b"\n"));

    let a = bullet * before + next_scene + after - sym(b'\n');
    a.map(|((before, next_scene), after)| Transition {
        next_scene,
        before,
        after,
    })
}

fn single_dashes<'a>() -> Parser<'a, u8, String> { char1(sym(b'-')) }
fn double_dashes<'a>() -> Parser<'a, u8, String> { char1(sym(b'=')) }
fn space<'a>() -> Parser<'a, u8, String> { char0(one_of(b" \t\n")) }

fn trimmed<'a>(p: Parser<'a, u8, String>) -> Parser<'a, u8, String> {
    p.map(|s| s.trim().to_string())
}

fn char0<'a>(p: Parser<'a, u8, u8>) -> Parser<'a, u8, String> {
    p.repeat(0..).collect().convert(|s| String::from_utf8(s.to_vec()))
}
fn char1<'a>(p: Parser<'a, u8, u8>) -> Parser<'a, u8, String> {
   p.repeat(1..).collect().convert(|s| String::from_utf8(s.to_vec()))
}

#[test]
fn test_parse() {
    let input = "\
        =====It happened=====\n\
        ----- Intro ------\n\
        Darth ( annoyed): Is anyone here?\n\
        Carren: Yes. I'm here Darth.\n\
        \n\
        Choices:\n\
        - Press [Ending] to say hi\n\
        - To start over, press [Intro]\n\
        \n\
        ----- Ending -----\n\
        Carren (loudly): Hello!\n\
        Darth: What......?\n\
    ";

    let expected = Script {
        name: String::from("It happened"),
        first_scene: String::from("Intro"),
        scenes: vec![
            Scene {
                name: String::from("Intro"),
                dialog: vec![
                    Line {
                        character: String::from("Darth"),
                        tone: Some(String::from("annoyed")),
                        content: String::from("Is anyone here?")
                    },
                    Line {
                        character: String::from("Carren"),
                        tone: None,
                        content: String::from("Yes. I'm here Darth.")
                    },
                ],
                transitions: vec![
                    Transition {
                        next_scene: String::from("Ending"),
                        before: String::from("Press "),
                        after: String::from(" to say hi"),
                    },
                    Transition {
                        next_scene: String::from("Intro"),
                        before: String::from("To start over, press "),
                        after: String::from(""),
                    },
                ]
            },
            Scene {
                name: String::from("Ending"),
                dialog: vec![
                    Line {
                        character: String::from("Carren"),
                        tone: Some(String::from("loudly")),
                        content: String::from("Hello!")
                    },
                    Line {
                        character: String::from("Darth"),
                        tone: None,
                        content: String::from("What......?")
                    },
                ],
                transitions: Vec::new(),
            },
        ]

    };

    assert_eq!(parse(input), Ok(expected));
}
