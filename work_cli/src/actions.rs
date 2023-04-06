use std::error::Error;

pub trait Actions {
    fn start(&mut self) -> Result<(), Box<dyn Error>>;
    // fn status(&mut self);
    // fn help(&mut self);
}
