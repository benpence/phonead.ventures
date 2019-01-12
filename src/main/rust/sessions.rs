use crate::types::*;

pub struct DummySessions;

impl Sessions for DummySessions {
    fn get(&self, _key: &Vec<u8>) -> Result<Vec<u8>, String> {
        // TODO:
        panic!("Unimplemented");
    }

    fn put(&mut self, _key: &[u8], _val: &[u8]) -> Result<(), String> {
        // TODO:
        panic!("Unimplemented");
    }
}
