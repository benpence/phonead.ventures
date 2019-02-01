use crate::adventure::script;
use std::collections::HashMap;

use crate::types::*;

#[derive(Clone, Debug)]
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
    fn get<'a>(
        &'a self,
        phone: &Phone
    ) -> Result<Option<&'a ScriptState>, String>;

    fn set(
        &mut self,
        phone: Phone,
        state: ScriptState
    ) -> Result<(), String>;
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
    fn get<'a>(
        &'a self,
        phone: &Phone
    ) -> Result<Option<&'a ScriptState>, String> {
        Ok(self.data.get(phone))
    }

    fn set(
        &mut self,
        phone: Phone,
        state: ScriptState
    ) -> Result<(), String> {
        self.data.insert(phone, state);
        Ok(())
    }
}
