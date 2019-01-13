use crate::twilio::twiml;

use crate::types::*;

pub struct TwilioRejectPlanner;

impl CallPlanner for TwilioRejectPlanner {

    fn extract_caller(&self, params: &WebParams) -> Result<Caller, String> {
        extract_twilio_caller(params)
    }

    fn format_action(&self, _action: &Action) -> Result<String, String> {
        let response = twiml::Response {
            verbs: vec![
                twiml::Verb::Reject(twiml::Reject)
            ]
        };

        Ok(twiml::to_xml_output(&response))
    }
}

pub struct TwilioPlanner {
    pub base_url: String,
}

impl CallPlanner for TwilioPlanner {

    fn extract_caller(&self, params: &WebParams) -> Result<Caller, String> {
        extract_twilio_caller(params)
    }

    fn format_action(&self, action: &Action) -> Result<String, String> {
        let response = match action {
            Action::Choices(choices) => twiml::Response {
                verbs: vec![
                    twiml::Verb::Gather(twiml::Gather {
                        children: choices
                            .iter()
                            .map(|choice| {
                                let text = format!("For {}, press {}", choice.description, choice.dial_number);
                                twiml::GatherChild::Say(twiml::Say { text })
                            })
                            .collect(),
                        num_digits: 1,
                    })
                ]
            },
            Action::Line(text) => twiml::Response {
                verbs: vec![
                    twiml::Verb::Say(twiml::Say { text: text.to_string() })
                ]
            },

        //    Action::Choices(choices) => twiml::Response {
        //        verbs: vec![
        //            twiml::Verb::Gather(twiml::Gather {
        //                children: choices.iter().map(|choice|
        //                    twiml::GatherChild::Play(twiml::Play { audio_file_url: self.to_url(&choice.description) })
        //                ).collect(),
        //                num_digits: 1,
        //            })
        //        ]
        //    },
        //            
        //    Action::Play(audio_file) => twiml::Response {
        //        verbs: vec![
        //            twiml::Verb::Play(twiml::Play { audio_file_url: self.to_url(audio_file) }),
        //        ]
        //    },
        };

        Ok(twiml::to_xml_output(&response))
    }
}

impl TwilioPlanner {
    fn to_url(&self, audio_file: &AudioFile) -> String {
        format!("{}/{}", self.base_url, audio_file.path)
    }
}

fn extract_twilio_caller(params: &WebParams) -> Result<Caller, String> {
    let opt_caller = params.body_params.get("From");
    let opt_digits = params.body_params.get("Input").and_then(|s| s.parse::<i32>().ok());

    println!("{:?}", params);

    match (opt_caller, opt_digits) {
        (Some(phone), Some(n)) if 0 < n && n < 10 => Ok(Caller::CallerWithChoice(phone.clone(), n)),
        (Some(phone), _      )                    => Ok(Caller::Caller(phone.clone())),
        (_,           _      )                    => Err(String::from("\"From\" body param missing from request")),
    }
}
