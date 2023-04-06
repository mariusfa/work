use work_cli::{run::run, actions_impl::ActionsImpl};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut actions = ActionsImpl {};

    if let Err(e) = run(args, &mut actions) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

