use pom::parser::*;
use pom::Parser;
use std::vec::Vec;


#[derive(Debug)]
pub struct Script {
    pub name: ScriptName,
    pub first_scene: SceneName,
    pub scenes: Vec<Scene>,
}

pub type ScriptName = String;
pub type SceneName = String;

#[derive(Debug)]
pub struct Scene {
    pub name: SceneName,
    pub dialog: Vec<Line>,
    pub transitions: Vec<Transition>,
}

#[derive(Debug)]
pub struct Transition {
    pub next_scene: SceneName,
    pub before: String,
    pub after: String,
}

#[derive(Debug)]
pub struct Line {
    pub character: String,
    pub tone: Option<String>,
    pub content: String,
}

pub fn parse(content: &'static str) -> Result<Script, String> {
    script().parse(content.as_bytes()).map_err(|e| {
        format!("Failed parsing script: {:?}", e)
    })
}

fn script() -> Parser<u8, Script> {
    let name = call(script_name);
    let scenes = call(scene).repeat(1..);

    (name + scenes).map(|(name, scenes)| {
        let first_scene = scenes.get(0).unwrap().name.to_string();

        Script {
            name,
            first_scene,
            scenes,
        }
    })
}

fn script_name() -> Parser<u8, ScriptName> {
    call(double_dashes) * char1(none_of(b"=")) - call(double_dashes) - sym(b'\n')
}

fn scene() -> Parser<u8, Scene> {
    let name = call(scene_name);
    let dialog = call(line).repeat(0..);
    let transitions = call(transitions);

    (name + dialog + transitions).map(|((name, dialog), transitions)| Scene {
        name,
        dialog,
        transitions,
    })
}

fn scene_name() -> Parser<u8, SceneName> {
   call(single_dashes) * char1(none_of(b"-")) - call(single_dashes) - sym(b'\n')
}

fn line() -> Parser<u8, Line> {
    let character = char1(none_of(b"(:"));
    let tone = (sym(b'(') * char1(none_of(b")")) - sym(b')')).opt();
    let content = char1(none_of(b"\n"));

    let line = character + tone - sym(b':') + content - sym(b'\n');

    line.map(|((character, tone), content)| Line {
        character,
        tone,
        content
    })
}

fn transitions() -> Parser<u8, Vec<Transition>> {
    let heading = seq(b"Choices:\n");
    let bullets = call(transition).repeat(1..);

    (heading * bullets).opt().map(|trans_opt| trans_opt.unwrap_or(Vec::new()))
}

fn transition() -> Parser<u8, Transition> {
    let bullet = seq(b"- ");
    let before = char0(none_of(b"["));
    let next_scene = sym(b'[') * char1(none_of(b"]")) - sym(b']');
    let after = char0(none_of(b"\n"));

    let a = bullet * before + next_scene + after;
    a.map(|((b, s), a)| Transition {
        next_scene: s,
        before: b,
        after: a,
    })
}

fn single_dashes() -> Parser<u8, String> {
    char1(sym(b'-'))
}
fn double_dashes() -> Parser<u8, String> {
    char1(sym(b'='))
}

fn char0(p: Parser<u8, u8>) -> Parser<u8, String> {
    p.repeat(0..).collect().convert(|s| String::from_utf8(s.to_vec()))
}
fn char1(p: Parser<u8, u8>) -> Parser<u8, String> {
   p.repeat(1..).collect().convert(|s| String::from_utf8(s.to_vec()))
}
