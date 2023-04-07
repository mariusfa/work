use crate::args::{parse_args, Command};
use crate::actions::Actions;
use std::error::Error;

pub fn run(args: Vec<String>, actions: &mut dyn Actions) -> Result<(), Box<dyn Error>> {
    let command = parse_args(&args)?;

    match command {
        Command::Start => actions.start()?,
        Command::Status => actions.status()?,
        Command::Help => actions.help()?,
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions_fake::ActionsFake;

    #[test]
    fn test_run_start() {
        let mut actions = ActionsFake {};
        let args = vec![String::from("work_cli"), String::from("start")];
        let result = run(args, &mut actions);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_status() {
        let mut actions = ActionsFake {};
        let args = vec![String::from("work_cli"), String::from("status")];
        let result = run(args, &mut actions);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_help() {
        let mut actions = ActionsFake {};
        let args = vec![String::from("work_cli"), String::from("help")];
        let result = run(args, &mut actions);
        assert!(result.is_ok());
    }
}
