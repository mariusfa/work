use crate::actions::Actions;
use std::error::Error;

pub struct ActionsFake {}

impl Actions for ActionsFake {
    fn start(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
