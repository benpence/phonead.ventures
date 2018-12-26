use crate::types::*;

pub struct TwilioRejectPlanner;

impl CallPlanner for TwilioRejectPlanner {

    fn extract_caller(&self, params: &WebParams) -> Result<Caller, String> {
        let opt_caller = params.body_params.get("From");
        let opt_digits = params.body_params.get("Input").and_then(|s| s.parse::<i32>().ok());

        println!("{:?}", params);

        match (opt_caller, opt_digits) {
            (Some(phone), Some(n)) if 0 < n && n < 10 => Ok(Caller::CallerWithChoice(phone.clone(), n)),
            (Some(phone), _      )                    => Ok(Caller::Caller(phone.clone())),
            (_,           _      )                    => Err(String::from("HEEEE")),
        }
    }

    fn format_action(&self, _action: &Action) -> Result<String, String> {
        Ok(String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?><Response><Reject /></Response>"))
    }
}

pub struct TwilioPlayPlanner {
    pub base_url: String,
}

impl CallPlanner for TwilioPlayPlanner {

    fn extract_caller(&self, params: &WebParams) -> Result<Caller, String> {
        let opt_caller = params.body_params.get("From");
        let opt_digits = params.body_params.get("Input").and_then(|s| s.parse::<i32>().ok());

        println!("{:?}", params);

        match (opt_caller, opt_digits) {
            (Some(phone), Some(n)) if 0 < n && n < 10 => Ok(Caller::CallerWithChoice(phone.clone(), n)),
            (Some(phone), _      )                    => Ok(Caller::Caller(phone.clone())),
            (_,           _      )                    => Err(String::from("HEEEE")),
        }
    }

    fn format_action(&self, action: &Action) -> Result<String, String> {
        let output = match action {
            Action::Play(audio_file) => {
                let audio_file_url = format!("{}/{}", self.base_url, audio_file.path);

                String::from(format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?><Play loop=\"0\">{}</Play>/Response>", audio_file_url))
            }
                    
            Action::Choices(_choices) => {
                String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?><Response><Reject /></Response>")
            }
        };

        Ok(output)
    }
}
