use std::vec::Vec;

pub fn to_xml_output(response: &Response) -> String {
    let mut output = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    let response_output = xml_to_string(&response.to_xml());

    output.push_str(&response_output);

    output
}

fn xml_to_string(xml: &XmlNode) -> String {
    let mut output = String::new();

    output.push_str(&format!("<{}", xml.tag));

    for (attr_name, attr_value) in &xml.attributes {
        output.push_str(&format!(" {}=\"{}\"", attr_name, attr_value));
    }

    match &xml.content {
        Content::Text(text) => {
            output.push_str(">");
            output.push_str(&text);
            output.push_str(&format!("</{}>", xml.tag));
        },
        Content::Children(children) => {
            output.push_str(">");

            for child in children {
                output.push_str(&xml_to_string(&child));
            }

            output.push_str(&format!("</{}>", xml.tag));
        },
        Content::Singleton => {
            output.push_str("/>");
        },
    }

    output
}

struct XmlNode {
    tag:        &'static str,
    content:    Content,
    attributes: Vec<(&'static str, String)>,
}

enum Content {
    Text(String),
    Children(Vec<XmlNode>),
    Singleton,
}

trait ToXml {
    fn to_xml(&self) -> XmlNode;
}

#[derive(Debug)]
pub struct Response {
    pub verbs: Vec<Verb>,
}

impl ToXml for Response {
    fn to_xml(&self) -> XmlNode {
        XmlNode {
            tag: "Response",
            content: Content::Children(self.verbs.iter().map(|v| v.to_xml()).collect()),
            attributes: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum Verb {
    Gather(Gather),
    Hangup(Hangup),
    Play(Play),
    Reject(Reject),
    Say(Say),
}

impl ToXml for Verb {
    fn to_xml(&self) -> XmlNode {
        match self {
            Verb::Gather(gather)     => gather.to_xml(),
            Verb::Hangup(hangup)     => hangup.to_xml(),
            Verb::Play(play)         => play.to_xml(),
            Verb::Reject(reject)     => reject.to_xml(),
            Verb::Say(say)           => say.to_xml(),
        }
    }
}

#[derive(Debug)]
pub struct Gather {
    pub children:        Vec<GatherChild>,
    pub num_digits:      i32,
}

impl ToXml for Gather {
    fn to_xml(&self) -> XmlNode {
        XmlNode {
            tag: "Gather",
            content: Content::Children(self.children.iter().map(|v| v.to_xml()).collect()),
            attributes: vec![
                ("input",          String::from("dtmf")),
                ("numDigits",      self.num_digits.to_string()),
            ],
        }
    }
}

#[derive(Debug)]
pub enum GatherChild {
    Play(Play),
    Say(Say),
}

impl ToXml for GatherChild {
    fn to_xml(&self) -> XmlNode {
        match self {
            GatherChild::Play(play) => play.to_xml(),
            GatherChild::Say(say)   => say.to_xml(),
        }
    }
}

#[derive(Debug)]
pub struct Hangup;

impl ToXml for Hangup {
    fn to_xml(&self) -> XmlNode {
        XmlNode {
            tag: "Hangup",
            content: Content::Singleton,
            attributes: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Play {
    pub audio_file_url: String,
}

impl ToXml for Play {
    fn to_xml(&self) -> XmlNode {
        XmlNode {
            tag: "Play",
            content: Content::Text(self.audio_file_url.clone()),
            attributes: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Reject;

impl ToXml for Reject {
    fn to_xml(&self) -> XmlNode {
        XmlNode {
            tag: "Reject",
            content: Content::Singleton,
            attributes: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Say {
    pub text:     String,
}

impl ToXml for Say {
    fn to_xml(&self) -> XmlNode {
        XmlNode {
            tag: "Say",
            content: Content::Text(self.text.clone()),
            attributes: Vec::new(),
        }
    }
}
