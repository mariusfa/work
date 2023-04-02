use std::error::Error;

#[derive(PartialEq, Debug)]
pub enum Command {
    Start,
    Status,
    Help,
}

pub fn parse_args(args: Vec<String>) -> Result<Command, Box<dyn Error>> {
    if args.len() < 2 {
        return Err("Insufficient arguments".into());
    }

    if args.len() > 2 {
        return Err("Too many arguments".into());
    }

    match args[1].as_str() {
        "start" => Ok(Command::Start),
        "status" => Ok(Command::Status),
        "help" | "--help" => Ok(Command::Help),
        _ => Err("Invalid command".into()),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args_insufficient_arguments() {
        let args = vec![String::from("work_cli")];

        let result = parse_args(args);

        assert!(result.is_err());
        assert_eq!(format!("{}", result.unwrap_err()), "Insufficient arguments");
    }

    #[test]
    fn test_parse_args_start() {
        let args = vec![String::from("work_cli"), String::from("start")];

        let command = parse_args(args).unwrap();

        assert_eq!(command, Command::Start);
    }

    #[test]
    fn test_parse_args_status() {
        let args = vec![String::from("work_cli"), String::from("status")];

        let command = parse_args(args).unwrap();

        assert_eq!(command, Command::Status);
    }

    #[test]
    fn test_parse_args_help() {
        let args = vec![String::from("work_cli"), String::from("help")];

        let command = parse_args(args).unwrap();

        assert_eq!(command, Command::Help);
    }

    #[test]
    fn test_parse_args_help_with_double_dash() {
        let args = vec![String::from("work_cli"), String::from("--help")];

        let command = parse_args(args).unwrap();

        assert_eq!(command, Command::Help);
    }

    #[test]
    fn test_parse_args_invalid_command() {
        let args = vec![String::from("work_cli"), String::from("invalid_command")];

        let result = parse_args(args);

        assert!(result.is_err());
        assert_eq!(format!("{}", result.unwrap_err()), "Invalid command");
    }

    #[test]
    fn test_parse_args_too_many_arguments() {
        let args = vec![
            String::from("work_cli"),
            String::from("start"),
            String::from("extra_arg"),
        ];

        let result = parse_args(args);

        assert!(result.is_err());
        assert_eq!(format!("{}", result.unwrap_err()), "Too many arguments");
    }
}
