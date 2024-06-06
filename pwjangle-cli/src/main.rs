mod command;
mod flags;
mod parser;

use command::Runner;

fn main() {
    let args = parser::Parse::from_env().parse();
    let code = match args.action {
        parser::Action::Init => command::Init::new(args).run(),
        parser::Action::Add => command::Add::new(args).run(),
        parser::Action::List => command::List::new(args).run(),
        parser::Action::Show => command::Show::new(args).run(),
        parser::Action::Remove => command::Remove::new(args).run(),
        parser::Action::None if args.version && !args.help => command::Version::new(args).run(),
        _ => command::Help::new(args).run(),
    };
    std::process::exit(code);
}
