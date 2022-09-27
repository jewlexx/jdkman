use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "JDKMan", version, author, about)]
pub struct JdkManArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    Add,
    List,
    Remove,
    Use,
}

impl JdkManArgs {
    pub fn parse() -> Self {
        debug!("Parsing Args");
        clap::Parser::parse()
    }
}
