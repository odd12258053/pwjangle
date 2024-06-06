#[derive(Debug, PartialEq)]
pub enum Flag {
    Short(String),
    Long(String),
    Value(String),
}

#[derive(Debug)]
pub enum FlagRef<'a> {
    Short(&'a str),
    Long(&'a str),
    Value(&'a str),
}

impl Flag {
    pub fn as_ref(&self) -> FlagRef {
        match self {
            Flag::Short(s) => FlagRef::Short(s),
            Flag::Long(s) => FlagRef::Long(s),
            Flag::Value(s) => FlagRef::Value(s),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Matched {
    None,
    Short(String),
    Long(String),
    Value(String),
}

#[derive(Debug)]
pub struct FlagParser {
    args: std::vec::IntoIter<String>,
    matched: Matched,
}

impl FlagParser {
    pub fn new(args: std::vec::IntoIter<String>) -> Self {
        Self { args, matched: Matched::None }
    }

    #[allow(dead_code)]
    pub fn from_env() -> Self {
        Self::from_iter(std::env::args())
    }
}

impl FromIterator<String> for FlagParser {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Self::new(Vec::from_iter(iter).into_iter())
    }
}

impl std::iter::Iterator for FlagParser {
    type Item = Flag;
    fn next(&mut self) -> Option<Self::Item> {
        match self.matched {
            Matched::Short(ref s) => {
                let mut chars = s.chars().skip(1);
                let flag = match chars.next() {
                    Some('-') | None => Some(Flag::Short("".to_string())),
                    Some(c) => Some(Flag::Short(c.to_string())),
                };
                let value = "-".to_string() + &String::from_iter(chars);
                self.matched = if value.len() > 1 { Matched::Short(value) } else { Matched::None };
                return flag;
            }
            Matched::Long(ref s) => {
                let mut v = s.splitn(2, '=');
                let flag = Flag::Long(String::from_iter(v.next().unwrap().chars().skip(2)));
                match v.next() {
                    Some(opt) => self.matched = Matched::Value(opt.to_string()),
                    None => self.matched = Matched::None,
                }
                return Some(flag);
            }
            Matched::Value(ref s) => {
                let value = s.clone();
                self.matched = Matched::None;
                return Some(Flag::Value(value));
            }
            Matched::None => (),
        }
        match self.args.next() {
            Some(s) => {
                if s.starts_with("--") {
                    self.matched = Matched::Long(s);
                    self.next()
                } else if s.starts_with('-') {
                    self.matched = Matched::Short(s);
                    self.next()
                } else {
                    self.matched = Matched::Value(s);
                    self.next()
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn args_parse() {
        let args = vec![
            String::from("a"),
            String::from("b"),
            String::from("-c"),
            String::from("--d"),
            String::from("e"),
            String::from("--f123=g"),
            String::from("--h123="),
            String::from("--"),
            String::from("-"),
            String::from("-vvv"),
        ];
        let mut parser = FlagParser::new(args.into_iter());
        assert_eq!(parser.next(), Some(Flag::Value("a".to_string())));
        assert_eq!(parser.next(), Some(Flag::Value("b".to_string())));
        assert_eq!(parser.next(), Some(Flag::Short("c".to_string())));
        assert_eq!(parser.next(), Some(Flag::Long("d".to_string())));
        assert_eq!(parser.next(), Some(Flag::Value("e".to_string())));
        assert_eq!(parser.next(), Some(Flag::Long("f123".to_string())));
        assert_eq!(parser.next(), Some(Flag::Value("g".to_string())));
        assert_eq!(parser.next(), Some(Flag::Long("h123".to_string())));
        assert_eq!(parser.next(), Some(Flag::Value("".to_string())));
        assert_eq!(parser.next(), Some(Flag::Long("".to_string())));
        assert_eq!(parser.next(), Some(Flag::Short("".to_string())));
        assert_eq!(parser.next(), Some(Flag::Short("v".to_string())));
        assert_eq!(parser.next(), Some(Flag::Short("v".to_string())));
        assert_eq!(parser.next(), Some(Flag::Short("v".to_string())));
    }
}
