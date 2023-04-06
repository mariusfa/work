use crate::actions::Actions;
use std::error::Error;

pub struct ActionsImpl {}

impl Actions for ActionsImpl {
    fn start(&mut self) -> Result<(), Box<dyn Error>> {
        unimplemented!();
    }

    fn status(&mut self) -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }
}