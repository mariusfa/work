use std::error::Error;

pub trait Actions {
    fn start(&mut self) -> Result<(), Box<dyn Error>>;
    fn status(&mut self) -> Result<(), Box<dyn Error>>;
    // fn help(&mut self);
}
