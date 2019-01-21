use crate::adventure::script;
use std::collections::HashMap;
use std::collections::hash_map;

use crate::types::*;

pub enum ScriptState {
    ChooseScript,
    BeginScene {
        script_name: script::ScriptName,
        scene_name: script::SceneName,
    },
    ChooseTransition {
        script_name: script::ScriptName,
        scene_name: script::SceneName,
    },
}

pub trait Sessions: Send {
    fn entry(
        &mut self,
        phone: &Phone
    ) -> Result<hash_map::Entry<Phone, ScriptState>, String>;
}

pub struct InMemorySessions {
    data: HashMap<Phone, ScriptState>
}

impl InMemorySessions {
    pub fn new() -> InMemorySessions {
        InMemorySessions { data: HashMap::new() }
    }
}

impl Sessions for InMemorySessions {
    fn entry(
        &mut self,
        phone: &Phone
    ) -> Result<hash_map::Entry<Phone, ScriptState>, String> {
        Ok(self.data.entry(phone.to_string()))
    }
}
