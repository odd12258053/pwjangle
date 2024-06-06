use crate::parser::Args;

pub trait Runner {
    fn run(&self) -> i32;
}

pub trait Helper {
    fn help() -> String;
}

/// Version
pub struct Version {}

impl Version {
    pub fn new(_args: Args) -> Self {
        Self {}
    }
    fn version<'a>() -> &'a str {
        env!("CARGO_PKG_VERSION")
    }
}

impl Runner for Version {
    fn run(&self) -> i32 {
        println!("version: {}", Self::version());
        0
    }
}

/// Help
pub struct Help {}

impl Help {
    pub fn new(_args: Args) -> Self {
        Self {}
    }
}

impl Runner for Help {
    fn run(&self) -> i32 {
        println!("help message");
        println!("{}", Init::help());
        println!("{}", Add::help());
        println!("{}", List::help());
        println!("{}", Show::help());
        println!("{}", Remove::help());
        0
    }
}

/// Init
pub struct Init {}

impl Init {
    pub fn new(_args: Args) -> Self {
        Self {}
    }
}

impl Runner for Init {
    fn run(&self) -> i32 {
        todo!()
    }
}

impl Helper for Init {
    fn help() -> String {
        "Help Message for Init".to_string()
    }
}

/// Add
pub struct Add {}

impl Add {
    pub fn new(_args: Args) -> Self {
        Self {}
    }
}

impl Runner for Add {
    fn run(&self) -> i32 {
        todo!()
    }
}

impl Helper for Add {
    fn help() -> String {
        "Help Message for Add".to_string()
    }
}

/// List
pub struct List {}

impl List {
    pub fn new(_args: Args) -> Self {
        Self {}
    }
}

impl Runner for List {
    fn run(&self) -> i32 {
        todo!()
    }
}

impl Helper for List {
    fn help() -> String {
        "Help Message for List".to_string()
    }
}

/// Show
pub struct Show {}

impl Show {
    pub fn new(_args: Args) -> Self {
        Self {}
    }
}

impl Runner for Show {
    fn run(&self) -> i32 {
        todo!()
    }
}

impl Helper for Show {
    fn help() -> String {
        "Help Message for Show".to_string()
    }
}

/// Remove
pub struct Remove {}

impl Remove {
    pub fn new(_args: Args) -> Self {
        Self {}
    }
}

impl Runner for Remove {
    fn run(&self) -> i32 {
        todo!()
    }
}

impl Helper for Remove {
    fn help() -> String {
        "Help Message for Remove".to_string()
    }
}
