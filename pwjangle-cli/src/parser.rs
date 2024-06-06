use crate::flags::{Flag, FlagParser, FlagRef};

#[derive(Debug, PartialEq)]
pub enum Action {
    None,
    Init,
    Add,
    List,
    Show,
    Remove,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct Args {
    pub version: bool,
    pub help: bool,
    pub action: Action,
    pub user: Option<Flag>,
    pub name: Option<Flag>,
    pub public_key: Option<Flag>,
    pub private_key_path: Option<Flag>,
    pub replace: bool,
}

impl Args {
    fn new() -> Self {
        Self {
            version: false,
            help: false,
            action: Action::None,
            user: None,
            name: None,
            public_key: None,
            private_key_path: None,
            replace: false,
        }
    }
}

#[allow(dead_code)]
pub struct Parse {
    fp: FlagParser,
}

impl Parse {
    fn new(fp: FlagParser) -> Self {
        Self { fp }
    }
    pub fn parse(&mut self) -> Args {
        let mut command = Args::new();
        while let Some(f) = self.fp.next() {
            match f.as_ref() {
                FlagRef::Long("help") | FlagRef::Short("h") => command.help = true,
                FlagRef::Long("version") | FlagRef::Short("V") => command.version = true,
                FlagRef::Long("user") => command.user = self.fp.next(),
                FlagRef::Long("name") => command.name = self.fp.next(),
                FlagRef::Long("public_key") => command.public_key = self.fp.next(),
                FlagRef::Long("private_key_path") => command.private_key_path = self.fp.next(),
                FlagRef::Long("replace") => command.replace = true,
                FlagRef::Value("init") => command.action = Action::Init,
                FlagRef::Value("add") => command.action = Action::Add,
                FlagRef::Value("list") => command.action = Action::List,
                FlagRef::Value("show") => command.action = Action::Show,
                FlagRef::Value("remove") => command.action = Action::Remove,
                _ => (),
            }
        }
        command
    }

    pub fn from_env() -> Self {
        Self::new(FlagParser::from_env())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn args_parse_with_version() {
        let mut expected = Args::new();
        expected.version = true;

        let args = vec![String::from("--version")];
        let parsed = Parse::new(FlagParser::new(args.into_iter())).parse();
        assert_eq!(parsed, expected)
    }

    #[test]
    fn args_parse_with_help() {
        let mut expected = Args::new();
        expected.help = true;

        let args = vec![String::from("--help")];
        let parsed = Parse::new(FlagParser::new(args.into_iter())).parse();
        assert_eq!(parsed, expected)
    }
}
